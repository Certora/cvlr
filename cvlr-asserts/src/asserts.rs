#[rustfmt::skip]
macro_rules! impl_bin_assert {
    ($name: ident, $pred: tt, $dollar: tt) => {
        #[macro_export]
        macro_rules! $name {
        ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            cvlr::clog!(stringify!(assert $lhs $pred $rhs));
            cvlr::clog!(lhs => stringify!($lhs));
            cvlr::clog!(rhs => stringify!($rhs));
            $crate::cvlr_assert!(lhs $pred rhs);
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
        #[macro_export]
        macro_rules! $name {
        ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            cvlr::clog!(stringify!(assume $lhs $pred $rhs));
            cvlr::clog!(lhs => stringify!($lhs));
            cvlr::clog!(rhs => stringify!($rhs));
            $crate::cvlr_assume!(lhs $pred rhs);
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
