#[macro_export]
macro_rules! cvlr_log_lineno {
    () => {
        cvlr::clog!(std::line!() => std::file!());
    };
}

macro_rules! impl_bin_assert {
    ($name: ident, $pred: tt) => {
        #[macro_export]
        macro_rules! $name {
            ($lhs: expr, $rhs: expr) => {{
                cvlr::clog!(stringify!(assert $lhs $pred $rhs));
                $crate::cvlr_log_lineno!();
                cvlr::clog!($lhs, $rhs);
                $crate::cvlr_assert!($lhs $pred $rhs);
            }};
        }   
        pub use $name;
    };
}

impl_bin_assert!(cvlr_assert_eq, ==);
impl_bin_assert!(cvlr_assert_ne, !=);
impl_bin_assert!(cvlr_assert_le, <=);
impl_bin_assert!(cvlr_assert_lt, <);
impl_bin_assert!(cvlr_assert_ge, >=);
impl_bin_assert!(cvlr_assert_gt, >);



