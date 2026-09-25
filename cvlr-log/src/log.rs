use crate::CvlrLogger;

/// A value that can be shown in the Prover's counterexample call trace.
///
/// This is the trait behind [`cvlr_log!`](macro@crate::cvlr_log) / `clog!`, and
/// it is the bound on the operands of the logging assertion macros
/// (`cvlr_assert_eq!` and friends). Logging has no effect on what is verified;
/// it only makes counterexamples readable.
///
/// # Provided implementations
///
/// | type | shown as |
/// | --- | --- |
/// | `bool`, `u8`, `u16`, `u32`, `u64`, `usize` | unsigned 64-bit integer under `tag` (`bool` as `0`/`1`) |
/// | `i8`, `i16`, `i32`, `i64` | 64-bit integer under `tag`; negative values currently display as their unsigned two's-complement bit pattern (see [`CvlrLogger::log_i64`]) |
/// | `u128` / `i128` | unsigned / signed 128-bit integer under `tag` |
/// | `&T` | as `T` |
/// | `&str` | the string itself as a message line; **`tag` is ignored** (use [`log_str`](crate::log_str) to show a string under a tag) |
/// | `()` | the string `"()"` under `tag` |
/// | `Option<T>` | `Some(v)` as `v` under `tag` (no `Some` marker); `None` as the string `"None"` under `tag` |
/// | `Result<T, E>` | a message line `Ok` or `Err`, followed by the payload under `tag` |
/// | `NativeInt` (feature `mathint`, on by default) | its value under `tag` |
///
/// Other CVLR crates implement it for their types (e.g. `NativeFixedU64`,
/// `NativeDecimal`).
///
/// Tags must be string literals (or `stringify!` output); see
/// [`CvlrLogger`] for this and other display rules.
///
/// # Implementing for your own types
///
/// Prefer `#[derive(CvlrLog)]` (from `cvlr::derive` / the prelude), which logs
/// each field under its name inside a scope named by `tag`, or
/// [`impl_cvlr_log_for_struct!`](macro@crate::impl_cvlr_log_for_struct) to log
/// only selected fields. A manual implementation should call the logging
/// methods of `logger`, or [`cvlr_log_with`] for nested fields, typically
/// bracketed by [`CvlrLogger::log_scope_start`]/[`CvlrLogger::log_scope_end`]
/// with `tag` so the fields are grouped under it:
///
/// ```
/// use cvlr_log::{cvlr_log_with, CvlrLog, CvlrLogger};
///
/// struct Range { lo: u64, hi: u64 }
///
/// impl CvlrLog for Range {
///     fn log(&self, tag: &str, logger: &mut CvlrLogger) {
///         logger.log_scope_start(tag);
///         cvlr_log_with("lo", &self.lo, logger);
///         cvlr_log_with("hi", &self.hi, logger);
///         logger.log_scope_end(tag);
///     }
/// }
/// ```
pub trait CvlrLog {
    /// Emits `self` into the call trace under the label `tag`.
    fn log(&self, tag: &str, logger: &mut CvlrLogger);
}

/// Logs `val` under `tag` through an explicit `logger`.
///
/// The same as `val.log(tag, logger)`. Use it inside [`CvlrLog`]
/// implementations to log nested fields. [`CvlrLogger`] carries no state, so
/// this behaves exactly like [`cvlr_log()`](fn@crate::cvlr_log).
#[inline(always)]
pub fn cvlr_log_with<T: CvlrLog>(tag: &str, val: &T, logger: &mut CvlrLogger) {
    val.log(tag, logger);
}

/// Logs `val` under `tag` in the counterexample call trace.
///
/// The function form of the [`cvlr_log!`](macro@crate::cvlr_log) macro's `value
/// => tag` case; most specs use the macro (`clog!` in the prelude) instead.
#[inline(always)]
pub fn cvlr_log<T: CvlrLog>(tag: &str, val: &T) {
    let mut logger = CvlrLogger::new();
    val.log(tag, &mut logger);
}

/// Expands to the current source file name, or `"<FILE>"` with feature
/// `no-loc`.
///
/// Implementation detail of [`cvlr_log!`](macro@crate::cvlr_log); exported only
/// so the macro can reach it from other crates. The `no-loc` feature exists to
/// make macro expansion output independent of source locations (for snapshot
/// tests).
#[cfg(not(feature = "no-loc"))]
#[macro_export]
macro_rules! cvlr_log_core_file {
    () => {
        ::core::file!()
    };
}

/// Expands to the current source line, or `0u32` with feature `no-loc`.
///
/// Implementation detail of [`cvlr_log!`](macro@crate::cvlr_log); exported only
/// so the macro can reach it from other crates. The `no-loc` feature exists to
/// make macro expansion output independent of source locations (for snapshot
/// tests).
#[cfg(not(feature = "no-loc"))]
#[macro_export]
macro_rules! cvlr_log_core_line {
    () => {
        ::core::line!()
    };
}

/// Expands to the current source file name, or `"<FILE>"` with feature
/// `no-loc`.
///
/// Implementation detail of [`cvlr_log!`](macro@crate::cvlr_log); exported only
/// so the macro can reach it from other crates.
#[cfg(feature = "no-loc")]
#[macro_export]
macro_rules! cvlr_log_core_file {
    () => {
        "<FILE>"
    };
}

/// Expands to the current source line, or `0u32` with feature `no-loc`.
///
/// Implementation detail of [`cvlr_log!`](macro@crate::cvlr_log); exported only
/// so the macro can reach it from other crates.
#[cfg(feature = "no-loc")]
#[macro_export]
macro_rules! cvlr_log_core_line {
    () => {
        0u32
    };
}

/// Logs values to the counterexample call trace. Also available as `clog!`.
///
/// Logging has no effect on what is verified; it records values so that a
/// counterexample (a failed `cvlr_assert!` or a witness for `cvlr_satisfy!`)
/// shows them at the point where they were logged.
///
/// # Forms
///
/// * `clog!(expr)` — logs the value of `expr`, tagged with the source text of
///   `expr` (e.g. `clog!(vault.shares)` shows under `vault.shares`).
/// * `clog!(expr => tag)` — logs the value under `tag`, a `&str` expression.
/// * `clog!(a, b => "b", c, ...)` — any mix of the two, left to right, one
///   trace entry per value; a trailing comma is allowed.
/// * `clog!()` — logs the current source location (file and line).
/// * `clog!(expr => tag; logger)` — logs through the [`CvlrLogger`] bound to
///   the identifier `logger` (only this single-value form takes a logger).
///
/// Each value is borrowed, not moved, and must implement [`CvlrLog`]; see that
/// trait for how each type is shown. Note that a `&str` value is shown as a
/// message line and its tag is dropped, so `clog!("reached withdraw")` is the
/// way to log a plain message.
///
/// ```ignore
/// use cvlr::prelude::*;
///
/// let amount: u64 = nondet();
/// clog!(amount);                    // under "amount"
/// clog!(amount * 2 => "doubled");   // under "doubled"
/// clog!("before transfer", amount, vault.balance);
/// ```
#[macro_export]
macro_rules! cvlr_log {
    () => {
        $crate::log_loc($crate::cvlr_log_core_file!(), $crate::cvlr_log_core_line!());
    };

    // log with a specified logger
    ($v:expr => $t:expr ; $logger:ident) => {
        $crate::cvlr_log_with($t, &($v), &mut $logger)
    };

    // multiple values with explicit tags
    ($v:expr => $t:expr, $( $vs:expr => $ts:expr ),+ $(,)?) => {
        $crate::cvlr_log! { $v => $t }
        $crate::cvlr_log! { $( $vs => $ts ),+ }
    };

    // first labeled, rest can be mixed (labeled or unlabeled)
    ($v:expr => $t:expr, $( $rest:tt )+) => {
        $crate::cvlr_log! { $v => $t }
        $crate::cvlr_log! { $( $rest )+ }
    };

    ($v:expr => $t:expr) => {
        // TODO: enable when this becomes stable
        // $crate::add_loc(core::file!(), core::line!());
        $crate::cvlr_log($t, &($v));
    };

    ($v:expr $(,)?) => {
        $crate::cvlr_log! { $v => stringify!($v) }
    };

    // first unlabeled, rest can be mixed (labeled or unlabeled)
    ($v:expr, $( $rest:tt )+) => {
        $crate::cvlr_log! { $v }
        $crate::cvlr_log! { $( $rest )+ }
    };
}

pub use cvlr_log as clog;

macro_rules! impl_cvlr_log_for_uint {
    ($t:ty) => {
        impl CvlrLog for $t {
            #[inline(always)]
            fn log(&self, tag: &str, logger: &mut CvlrLogger) {
                logger.log_u64(tag, *self as u64);
            }
        }
    };
}

impl_cvlr_log_for_uint!(bool);
impl_cvlr_log_for_uint!(u8);
impl_cvlr_log_for_uint!(u16);
impl_cvlr_log_for_uint!(u32);
impl_cvlr_log_for_uint!(u64);
impl_cvlr_log_for_uint!(usize);

impl CvlrLog for u128 {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u128(tag, *self);
    }
}

macro_rules! impl_cvlr_log_for_int {
    ($t:ty) => {
        impl CvlrLog for $t {
            #[inline(always)]
            fn log(&self, tag: &str, logger: &mut CvlrLogger) {
                logger.log_i64(tag, *self as i64);
            }
        }
    };
}

impl_cvlr_log_for_int!(i8);
impl_cvlr_log_for_int!(i16);
impl_cvlr_log_for_int!(i32);
impl_cvlr_log_for_int!(i64);

impl CvlrLog for i128 {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_i128(tag, *self);
    }
}

impl<T: CvlrLog> CvlrLog for &T {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        (**self).log(tag, logger);
    }
}

impl CvlrLog for &str {
    #[inline(always)]
    fn log(&self, _tag: &str, logger: &mut CvlrLogger) {
        logger.log(self);
    }
}

impl CvlrLog for () {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_str(tag, "()");
    }
}

impl<T: CvlrLog> CvlrLog for Option<T> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        if let Some(v) = self {
            v.log(tag, logger);
        } else {
            logger.log_str(tag, "None");
        }
    }
}

impl<T: CvlrLog, E: CvlrLog> CvlrLog for Result<T, E> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        match self {
            Ok(v) => {
                logger.log("Ok");
                v.log(tag, logger)
            }
            Err(v) => {
                logger.log("Err");
                v.log(tag, logger)
            }
        }
    }
}

#[cfg(feature = "mathint")]
impl CvlrLog for cvlr_mathint::NativeInt {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u64(tag, self.as_internal());
    }
}

/// Implements CvlrLog trait given a struct and a list of fields
///
/// `impl_cvlr_log_for_struct!(Type, field1, field2, ...)` logs each listed
/// field under its name, inside a call-trace scope named by the tag, and
/// ignores unlisted fields. Every listed field must implement [`CvlrLog`]; the
/// fields must be accessible at the invocation site, and the usual orphan rule
/// applies. Also exported as `impl_clog_for_struct!`. `#[derive(CvlrLog)]` is
/// the alternative that logs every field.
///
/// Example usage
/// ```
/// use cvlr_log::impl_cvlr_log_for_struct;
/// struct Foo {
///     x: u64,
///     y: u64,
/// }
/// impl_cvlr_log_for_struct!(Foo, x, y,);
/// ```
#[macro_export]
macro_rules! impl_cvlr_log_for_struct {
    ($prop:path $(, $field:ident)* $(,)?) => {
        impl $crate::CvlrLog for $prop {
            fn log(&self, tag: &str, logger: &mut $crate::CvlrLogger) {
                logger.log_scope_start(tag);
                let __self = self;
                $(impl_cvlr_log_for_struct!(@field __self, logger, $field);)*
                logger.log_scope_end(tag);
            }
        }
    };

    (@field $self:ident, $logger:ident, $field:ident) => {
        $crate::cvlr_log_with(stringify!($field), &$self.$field, $logger);
    };
}

pub use impl_cvlr_log_for_struct as impl_clog_for_struct;
