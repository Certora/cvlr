#[rustfmt::skip]
macro_rules! impl_bin_assert {
    ($name: ident, $pred: tt, $dollar: tt) => {
        #[doc = concat!("Asserts `lhs ", stringify!($pred), " rhs` and records both operands in the counterexample.")]
        ///
        #[doc = concat!("Syntax: `", stringify!($name), "!(lhs, rhs)` or `", stringify!($name), "!(lhs, rhs, \"description\")`.")]
        /// The optional description must be a string literal; it is accepted
        /// for readability and currently ignored.
        ///
        /// Each operand is evaluated exactly once, `lhs` first. Inside a
        /// call-trace scope named `assert`, the macro logs the source text of
        /// the comparison and the value of each operand (tagged with the
        /// operand's source text), then checks the comparison with
        /// [`cvlr_assert!`](crate::cvlr_assert). A failing assertion therefore
        /// shows the concrete values of both sides; prefer this over
        #[doc = concat!("`cvlr_assert!(lhs ", stringify!($pred), " rhs)`, which records neither.")]
        ///
        /// Both operands must implement `cvlr::log::CvlrLog`, and the
        /// comparison must type-check between them. The expansion refers to
        /// `cvlr::...` paths, so the calling crate must depend on the `cvlr`
        /// crate.
        #[macro_export]
        macro_rules! $name {
        ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let __cvlr_lhs = $lhs;
            let __cvlr_rhs = $rhs;
            cvlr::log::log_scope_start("assert");
            cvlr::clog!(stringify!($lhs $pred $rhs) => "_");
            cvlr::clog!(__cvlr_lhs => stringify!($lhs));
            cvlr::clog!(__cvlr_rhs => stringify!($rhs));
            cvlr::log::log_scope_end("assert");
            $crate::cvlr_assert!(__cvlr_lhs $pred __cvlr_rhs);
        }};
    }
        pub use $name;
    };
}

impl_bin_assert!(cvlr_assert_eq, ==, $);
impl_bin_assert!(cvlr_assert_ne, !=, $);
impl_bin_assert!(cvlr_assert_le, <=, $);
impl_bin_assert!(cvlr_assert_lt, <, $);
impl_bin_assert!(cvlr_assert_ge, >=, $);
impl_bin_assert!(cvlr_assert_gt, >, $);

#[rustfmt::skip]
macro_rules! impl_bin_assume {
    ($name: ident, $pred: tt, $dollar: tt) => {
        #[doc = concat!("Assumes `lhs ", stringify!($pred), " rhs` and records both operands in the counterexample.")]
        ///
        #[doc = concat!("Syntax: `", stringify!($name), "!(lhs, rhs)` or `", stringify!($name), "!(lhs, rhs, \"description\")`.")]
        /// The optional description must be a string literal; it is accepted
        /// for readability and currently ignored.
        ///
        /// Each operand is evaluated exactly once, `lhs` first. Inside a
        /// call-trace scope named `assume`, the macro logs the source text of
        /// the comparison and the value of each operand (tagged with the
        /// operand's source text), then restricts the analysis to executions
        /// where the comparison holds, as [`cvlr_assume!`](crate::cvlr_assume)
        /// does.
        ///
        /// Both operands must implement `cvlr::log::CvlrLog`, and the
        /// comparison must type-check between them. The expansion refers to
        /// `cvlr::...` paths, so the calling crate must depend on the `cvlr`
        /// crate.
        #[macro_export]
        macro_rules! $name {
        ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let __cvlr_lhs = $lhs;
            let __cvlr_rhs = $rhs;
            cvlr::log::log_scope_start("assume");
            cvlr::clog!(stringify!($lhs $pred $rhs) => "_");
            cvlr::clog!(__cvlr_lhs => stringify!($lhs));
            cvlr::clog!(__cvlr_rhs => stringify!($rhs));
            cvlr::log::log_scope_end("assume");
            $crate::cvlr_assume!(__cvlr_lhs $pred __cvlr_rhs);
        }};
    }
        pub use $name;
    };
}

impl_bin_assume!(cvlr_assume_eq, ==, $);
impl_bin_assume!(cvlr_assume_ne, !=, $);
impl_bin_assume!(cvlr_assume_le, <=, $);
impl_bin_assume!(cvlr_assume_lt, <, $);
impl_bin_assume!(cvlr_assume_ge, >=, $);
impl_bin_assume!(cvlr_assume_gt, >, $);

/// Asserts `cond` only when `guard` is true.
///
/// Syntax: `cvlr_assert_if!(guard, cond)`. Expands to
/// `if guard { cvlr_assert!(cond) }`: `cond` is evaluated and checked only when
/// `guard` is true, i.e. this checks the implication `guard ==> cond`. Nothing
/// is logged; for a comparison, the `cvlr_assert_*_if!` forms (e.g.
/// [`cvlr_assert_le_if!`](crate::cvlr_assert_le_if)) also record the values
/// involved.
#[macro_export]
macro_rules! cvlr_assert_if {
    ($guard: expr, $cond: expr) => {
        if $guard {
            $crate::cvlr_assert!($cond);
        }
    };
}

#[rustfmt::skip]
macro_rules! impl_bin_assert_if {
    ($name: ident, $pred: tt, $dollar: tt) => {
        #[doc = concat!("Asserts `lhs ", stringify!($pred), " rhs` only when `guard` is true, recording the values involved.")]
        ///
        #[doc = concat!("Syntax: `", stringify!($name), "!(guard, lhs, rhs)` or `", stringify!($name), "!(guard, lhs, rhs, \"description\")`.")]
        /// The optional description must be a string literal; it is accepted
        /// for readability and currently ignored.
        ///
        #[doc = concat!("Checks the implication `guard ==> lhs ", stringify!($pred), " rhs`. The macro evaluates")]
        /// `guard` once and logs the source text of the whole check and the
        /// value of `guard`. If `guard` is true, it then evaluates `lhs` and
        /// `rhs` once each, logs their values (tagged with their source text),
        /// and checks the comparison with [`cvlr_assert!`](crate::cvlr_assert);
        /// if `guard` is false, the operands are not evaluated and nothing is
        /// asserted.
        ///
        /// `guard` must be a `bool`; both operands must implement
        /// `cvlr::log::CvlrLog`, and the comparison must type-check between
        /// them. The expansion refers to `cvlr::...` paths, so the calling
        /// crate must depend on the `cvlr` crate.
        #[macro_export]
        macro_rules! $name {
        ($guard: expr,$lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let __cvlr_guard = $guard;
            cvlr::clog!(stringify!(assert if $guard { $lhs $pred $rhs }) => "_");
            cvlr::clog!(__cvlr_guard => stringify!($guard));
            if __cvlr_guard {
                let __cvlr_lhs = $lhs;
                let __cvlr_rhs = $rhs;
                cvlr::clog!(__cvlr_lhs => stringify!($lhs));
                cvlr::clog!(__cvlr_rhs => stringify!($rhs));
                $crate::cvlr_assert!(__cvlr_lhs $pred __cvlr_rhs);
            }
        }};
    }
        pub use $name;
    };
}

impl_bin_assert_if!(cvlr_assert_eq_if, ==, $);
impl_bin_assert_if!(cvlr_assert_ne_if, !=, $);
impl_bin_assert_if!(cvlr_assert_le_if, <=, $);
impl_bin_assert_if!(cvlr_assert_lt_if, <, $);
impl_bin_assert_if!(cvlr_assert_ge_if, >=, $);
impl_bin_assert_if!(cvlr_assert_gt_if, >, $);
