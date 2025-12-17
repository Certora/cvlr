#[rustfmt::skip]
macro_rules! impl_bin_assert {
    ($name: ident, $pred: tt, $dollar: tt) => {
        #[macro_export]
        macro_rules! $name {
        ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let __cvlr_lhs = $lhs;
            let __cvlr_rhs = $rhs;
            cvlr::clog!(stringify!(assert $lhs $pred $rhs) => "_");
            cvlr::clog!(__cvlr_lhs => stringify!($lhs));
            cvlr::clog!(__cvlr_rhs => stringify!($rhs));
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
        #[macro_export]
        macro_rules! $name {
        ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
            let __cvlr_lhs = $lhs;
            let __cvlr_rhs = $rhs;
            cvlr::clog!(stringify!(assume $lhs $pred $rhs) => "_");
            cvlr::clog!(__cvlr_lhs => stringify!($lhs));
            cvlr::clog!(__cvlr_rhs => stringify!($rhs));
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
