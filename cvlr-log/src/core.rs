/// Declarations of the Prover's call-trace builtins, which the functions of
/// this crate wrap. Their items are also exported at the crate root.
pub mod rt_decls {
    #[cfg_attr(
        all(target_family = "wasm", target_os = "none"),
        link(wasm_import_module = "env")
    )]
    #[allow(improper_ctypes)]
    extern "C" {
        /// Prover builtin: a call-trace line showing `tag`. Wrapped by
        /// [`CvlrLogger::log`](crate::CvlrLogger::log).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_tag(tag: &str);

        /// Prover builtin: a call-trace line `tag: x`. Wrapped by
        /// [`CvlrLogger::log_u64`](crate::CvlrLogger::log_u64).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_u64_1(tag: &str, x: u64);
        /// Prover builtin: a call-trace line `tag: x, y`. Wrapped by
        /// [`CvlrLogger::log_u64_2`](crate::CvlrLogger::log_u64_2).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_u64_2(tag: &str, x: u64, y: u64);
        /// Prover builtin: a call-trace line `tag: x, y, z`. Wrapped by
        /// [`CvlrLogger::log_u64_3`](crate::CvlrLogger::log_u64_3).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_u64_3(tag: &str, x: u64, y: u64, z: u64);
        /// Prover builtin: a call-trace line `tag: x`. Wrapped by
        /// [`CvlrLogger::log_u128`](crate::CvlrLogger::log_u128).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_u128(tag: &str, x: u128);

        /// Prover builtin: a call-trace line `tag: x`, currently displayed
        /// unsigned (see [`CvlrLogger::log_i64`](crate::CvlrLogger::log_i64),
        /// which wraps it).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_i64_1(tag: &str, x: i64);
        /// Prover builtin: a call-trace line `tag: x, y`, currently displayed
        /// unsigned (see [`CvlrLogger::log_i64`](crate::CvlrLogger::log_i64)).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_i64_2(tag: &str, x: i64, y: i64);
        /// Prover builtin: a call-trace line `tag: x, y, z`, currently
        /// displayed unsigned (see
        /// [`CvlrLogger::log_i64`](crate::CvlrLogger::log_i64)).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_i64_3(tag: &str, x: i64, y: i64, z: i64);
        /// Prover builtin: a call-trace line `tag: x`, signed. Wrapped by
        /// [`CvlrLogger::log_i128`](crate::CvlrLogger::log_i128).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_i128(tag: &str, x: i128);

        /// Prover builtin: a call-trace line `tag: v`. Wrapped by
        /// [`CvlrLogger::log_str`](crate::CvlrLogger::log_str).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_string(tag: &str, v: &str);

        /// Prover builtin: `x` shown as a binary fixed-point number with `y`
        /// fractional bits. Wrapped by
        /// [`CvlrLogger::log_u64_as_fp`](crate::CvlrLogger::log_u64_as_fp).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_u64_as_fixed(tag: &str, x: u64, y: u64);
        /// Prover builtin: `x` shown as a decimal number with `y` fractional
        /// digits. Wrapped by
        /// [`CvlrLogger::log_u64_as_dec`](crate::CvlrLogger::log_u64_as_dec).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_u64_as_decimal(tag: &str, x: u64, y: u64);

        /// Prover builtin: a call-trace line `file:line`. Wrapped by
        /// [`CvlrLogger::log_loc`](crate::CvlrLogger::log_loc).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_print_location(file: &str, line: u64);
        /// Prover builtin: attaches `file:line` to the next call-trace entry.
        /// Wrapped by [`CvlrLogger::add_loc`](crate::CvlrLogger::add_loc).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_attach_location(file: &str, line: u64);

        /// Prover builtin: declares the rule's source location. Wrapped by
        /// [`CvlrLogger::log_rule_location`](crate::CvlrLogger::log_rule_location).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_rule_location(file: &str, line: u64);

        /// Prover builtin: opens a call-trace scope. Wrapped by
        /// [`CvlrLogger::log_scope_start`](crate::CvlrLogger::log_scope_start).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_scope_start(name: &str);
        /// Prover builtin: closes a call-trace scope. Wrapped by
        /// [`CvlrLogger::log_scope_end`](crate::CvlrLogger::log_scope_end).
        ///
        /// Use the safe wrapper instead; every string argument must be a string
        /// literal (see [`CvlrLogger`](crate::CvlrLogger)).
        pub fn CVT_calltrace_scope_end(name: &str);
    }
}

#[cfg(feature = "rt")]
#[allow(improper_ctypes_definitions)]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_tag(_tag: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_1(_tag: &str, _x: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_2(_tag: &str, _x: u64, _y: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_3(_tag: &str, _x: u64, _y: u64, _z: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u128(_tag: &str, _x: u128) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_1(_tag: &str, _x: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_2(_tag: &str, _x: i64, _y: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_3(_tag: &str, _x: i64, _y: i64, _z: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i128(_tag: &str, _x: i128) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_as_fixed(_tag: &str, _x: u64, _y: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_as_decimal(_tag: &str, _x: u64, _y: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_string(_tag: &str, _v: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_location(_file: &str, _line: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_attach_location(_file: &str, _line: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_rule_location(_file: &str, _line: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_scope_start(_name: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_scope_end(_name: &str) {}
}
pub use rt_decls::*;

/// Handle for writing entries to the Prover's counterexample call trace.
///
/// Logging never affects what is verified; the entries only appear in the call
/// trace of a counterexample (a failed assertion) or witness (a satisfied
/// `cvlr_satisfy!`), in execution order, at the point where the logging call
/// executed. Most specs log through `clog!` /
/// [`cvlr_log!`](macro@crate::cvlr_log) and the [`CvlrLog`](crate::CvlrLog)
/// trait, which call these methods; use them directly when implementing
/// `CvlrLog` or when a specific rendering is needed. Each method also exists as
/// a free function of the same name that creates a logger for the call (e.g.
/// [`log_u64`]).
///
/// `CvlrLogger` has no state: [`new`](Self::new) is free, and all loggers are
/// equivalent.
///
/// # Strings must be literals
///
/// Every `&str` argument (tags, scope names, file names, string values) must
/// be resolvable by the Prover to constant data, which in practice means a
/// string literal or `stringify!`/`file!` output. A string built or selected
/// at run time is not shown: the entry gets a placeholder label and the Prover
/// emits a warning. Line numbers must likewise be constants.
///
/// # Number display
///
/// Integers are displayed in decimal up to `2^32` and in hexadecimal, or as a
/// named constant such as `u64::MAX`, above that.
///
/// # Soroban
///
/// The Soroban (Wasm) Prover supports only [`log`](Self::log),
/// [`log_u64`](Self::log_u64), [`log_i64`](Self::log_i64),
/// [`log_u128`](Self::log_u128) and [`log_i128`](Self::log_i128) (plus
/// 32-bit forms not exposed here); the rest are Solana-only.
#[derive(Default)]
pub struct CvlrLogger;

impl CvlrLogger {
    /// Creates a logger. Loggers carry no state, so this costs nothing.
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
    /// Adds a line showing the message `v`.
    ///
    /// This is how `&str` values are logged by [`CvlrLog`](crate::CvlrLog):
    /// with no tag.
    #[inline(always)]
    pub fn log(&mut self, v: &str) {
        unsafe {
            CVT_calltrace_print_tag(v);
        }
    }

    /// Adds a line `t: v`, showing the string `v` under the tag `t`.
    #[inline(always)]
    pub fn log_str(&mut self, t: &str, v: &str) {
        unsafe {
            CVT_calltrace_print_string(t, v);
        }
    }

    /// Adds a line `t: v`, showing `v` as an unsigned integer.
    #[inline(always)]
    pub fn log_u64(&mut self, t: &str, v: u64) {
        unsafe {
            CVT_calltrace_print_u64_1(t, v);
        }
    }

    /// Adds a single line `t: v0, v1`.
    #[inline(always)]
    pub fn log_u64_2(&mut self, t: &str, v0: u64, v1: u64) {
        unsafe {
            CVT_calltrace_print_u64_2(t, v0, v1);
        }
    }

    /// Adds a single line `t: v0, v1, v2`.
    #[inline(always)]
    pub fn log_u64_3(&mut self, t: &str, v0: u64, v1: u64, v2: u64) {
        unsafe {
            CVT_calltrace_print_u64_3(t, v0, v1, v2);
        }
    }

    /// Adds a line `t: v`, showing `v` as an unsigned 128-bit integer.
    #[inline(always)]
    pub fn log_u128(&mut self, t: &str, v: u128) {
        unsafe {
            CVT_calltrace_print_u128(t, v);
        }
    }

    /// Adds a line `t: v` for a signed 64-bit value.
    ///
    /// The Solana Prover currently displays the value as unsigned, so a
    /// negative `v` appears as its two's-complement bit pattern (a large number
    /// or a hexadecimal value) rather than with a minus sign. To show a
    /// negative value with its sign, log it as `i128` with
    /// [`log_i128`](Self::log_i128).
    #[inline(always)]
    pub fn log_i64(&mut self, t: &str, v: i64) {
        unsafe {
            CVT_calltrace_print_i64_1(t, v);
        }
    }

    /// Adds a line `t: v`, showing `v` as a signed 128-bit integer.
    #[inline(always)]
    pub fn log_i128(&mut self, t: &str, v: i128) {
        unsafe {
            CVT_calltrace_print_i128(t, v);
        }
    }

    /// Adds a line showing `v` as a binary fixed-point number with `b`
    /// fractional bits, i.e. `v / 2^b`.
    ///
    /// Rendered as `t: <v / 2^b> (<v>)`: the value rounded down to `b` decimal
    /// places with trailing zeros removed, followed by the raw integer. Used by
    /// the `CvlrLog` implementations of `NativeFixedU64` / `NativeFixedU128`.
    #[inline(always)]
    pub fn log_u64_as_fp(&mut self, t: &str, v: u64, b: u64) {
        unsafe {
            CVT_calltrace_print_u64_as_fixed(t, v, b);
        }
    }

    /// Adds a line showing `v` as a decimal number with `d` fractional digits,
    /// i.e. `v / 10^d` (e.g. lamports as SOL with `d = 9`). Used by the
    /// `CvlrLog` implementation of `NativeDecimal`.
    #[inline(always)]
    pub fn log_u64_as_dec(&mut self, t: &str, v: u64, d: u64) {
        unsafe {
            CVT_calltrace_print_u64_as_decimal(t, v, d);
        }
    }

    /// Adds a line `file:line` that links to that source location.
    ///
    /// The line is shown only if `file` is among the source files uploaded with
    /// the verification job. `clog!()` logs the location of the macro call this
    /// way.
    #[inline(always)]
    pub fn log_loc(&mut self, file: &str, line: u32) {
        unsafe {
            CVT_calltrace_print_location(file, line as u64);
        }
    }

    /// Attaches the source location `file:line` to the next call-trace entry
    /// instead of adding a line.
    ///
    /// The location becomes the jump-to-source target of the next assertion,
    /// satisfy, logged value, scope start or nondeterministic choice (the most
    /// recently attached location wins). `cvlr_assert!` and `cvlr_satisfy!` do
    /// this automatically for their own location.
    #[inline(always)]
    pub fn add_loc(&mut self, file: &str, line: u32) {
        unsafe {
            CVT_calltrace_attach_location(file, line as u64);
        }
    }

    /// Declares `file:line` as the source location of the current rule.
    ///
    /// The report uses it as the rule's location when the Prover cannot find
    /// the rule function in the debug information. `#[rule]` emits this for
    /// every rule through [`cvlr_rule_location!`](crate::cvlr_rule_location);
    /// calling it again is unnecessary.
    #[inline(always)]
    pub fn log_rule_location(&mut self, file: &str, line: u64) {
        unsafe {
            crate::CVT_rule_location(file, line);
        }
    }

    /// Opens a nested, collapsible scope named `scope` in the call trace.
    ///
    /// Entries until the matching [`log_scope_end`](Self::log_scope_end) are
    /// shown inside it. Every start must be closed by an end with the same
    /// name, in LIFO order; otherwise the trace structure is garbled.
    /// `#[derive(CvlrLog)]` and the comparison assertion macros use scopes to
    /// group related values.
    #[inline(always)]
    pub fn log_scope_start(&mut self, scope: &str) {
        unsafe {
            CVT_calltrace_scope_start(scope);
        }
    }

    /// Closes the innermost scope, which must have been opened with the same
    /// `scope` name by [`log_scope_start`](Self::log_scope_start).
    #[inline(always)]
    pub fn log_scope_end(&mut self, scope: &str) {
        unsafe {
            CVT_calltrace_scope_end(scope);
        }
    }
}

/// [`CvlrLogger::log`] on a fresh logger.
#[inline(always)]
pub fn log(v: &str) {
    let mut logger = CvlrLogger::new();
    logger.log(v);
}

/// [`CvlrLogger::log_u64_as_fp`] on a fresh logger.
#[inline(always)]
pub fn log_u64_as_fp(t: &str, v: u64, b: u64) {
    let mut logger = CvlrLogger::new();
    logger.log_u64_as_fp(t, v, b);
}

/// [`CvlrLogger::log_u64_as_dec`] on a fresh logger.
#[inline(always)]
pub fn log_u64_as_dec(t: &str, v: u64, d: u64) {
    let mut logger = CvlrLogger::new();
    logger.log_u64_as_dec(t, v, d);
}

/// [`CvlrLogger::log_rule_location`] on a fresh logger.
#[inline(always)]
pub fn log_rule_location(file: &str, line: u64) {
    let mut logger = CvlrLogger::new();
    logger.log_rule_location(file, line);
}

/// [`CvlrLogger::log_scope_start`] on a fresh logger.
#[inline(always)]
pub fn log_scope_start(scope: &str) {
    let mut logger = CvlrLogger::new();
    logger.log_scope_start(scope);
}

/// [`CvlrLogger::log_scope_end`] on a fresh logger.
#[inline(always)]
pub fn log_scope_end(scope: &str) {
    let mut logger = CvlrLogger::new();
    logger.log_scope_end(scope);
}

macro_rules! expose_log_fn {
    ($name: ident, $ty: ty) => {
        #[doc = concat!("[`CvlrLogger::", stringify!($name), "`] on a fresh logger.")]
        #[inline(always)]
        pub fn $name(t: &str, v: $ty) {
            let mut logger = CvlrLogger::new();
            logger.$name(t, v)
        }
    };
}

expose_log_fn! {log_str, &str}
expose_log_fn! {log_u64, u64}
expose_log_fn! {log_i64, i64}
expose_log_fn! {log_u128, u128}
expose_log_fn! {log_i128, i128}
expose_log_fn! {log_loc, u32}
expose_log_fn! {add_loc, u32}

/// Declares the location of the macro call as the current rule's source
/// location; see [`CvlrLogger::log_rule_location`].
///
/// `#[rule]` inserts this as the first statement of every rule, so specs do
/// not need to call it. With the `no-loc` feature the file and line are
/// placeholders, so expansion snapshots do not depend on the source path.
#[macro_export]
macro_rules! cvlr_rule_location {
    () => {{
        $crate::log_rule_location(
            $crate::cvlr_log_core_file!(),
            $crate::cvlr_log_core_line!() as u64,
        )
    }};
}
