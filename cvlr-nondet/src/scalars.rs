use crate::Nondet;

mod rt_decls {
    #[allow(improper_ctypes)]
    extern "C" {
        // Definition of external functions that represent getting arbitrary values
        pub fn CVT_nondet_u8() -> u8;
        pub fn CVT_nondet_u16() -> u16;
        pub fn CVT_nondet_u32() -> u32;
        pub fn CVT_nondet_u64() -> u64;
        pub fn CVT_nondet_small_u128() -> u128;
        pub fn CVT_nondet_u128() -> u128;
        pub fn CVT_nondet_usize() -> usize;

        pub fn CVT_nondet_i8() -> i8;
        pub fn CVT_nondet_i16() -> i16;
        pub fn CVT_nondet_i32() -> i32;
        pub fn CVT_nondet_i64() -> i64;
        pub fn CVT_nondet_small_i128() -> i128;
        pub fn CVT_nondet_i128() -> i128;
    }
}

#[cfg(feature = "rt")]
#[allow(improper_ctypes_definitions)]
mod rt_impls {
    macro_rules! impl_rt_fn {
        // 1st argument is ignored
        ($name:ident, $c_name:ident, $ty:ident) => {
            #[no_mangle]
            pub extern "C" fn $c_name() -> $ty {
                $ty::default()
            }
        };
    }
    impl_rt_fn!(cvlr_nondet_u8, CVT_nondet_u8, u8);
    impl_rt_fn!(cvlr_nondet_u16, CVT_nondet_u16, u16);
    impl_rt_fn!(cvlr_nondet_u32, CVT_nondet_u32, u32);
    impl_rt_fn!(cvlr_nondet_u64, CVT_nondet_u64, u64);
    impl_rt_fn!(cvlr_nondet_u128, CVT_nondet_u128, u128);
    impl_rt_fn!(cvlr_nondet_usize, CVT_nondet_usize, usize);

    impl_rt_fn!(cvlr_nondet_i8, CVT_nondet_i8, i8);
    impl_rt_fn!(cvlr_nondet_i16, CVT_nondet_i16, i16);
    impl_rt_fn!(cvlr_nondet_i32, CVT_nondet_i32, i32);
    impl_rt_fn!(cvlr_nondet_i64, CVT_nondet_i64, i64);
    impl_rt_fn!(cvlr_nondet_i128, CVT_nondet_i128, i128);

    impl_rt_fn!(cvlr_nondet_small_u128, CVT_nondet_small_u128, u128);
    impl_rt_fn!(cvlr_nondet_small_i128, CVT_nondet_small_i128, i128);
}

use rt_decls::*;

macro_rules! impl_checked_fn {
    ($name:ident, $c_name:ident, $ty:ident) => {
        #[inline(always)]
        pub fn $name() -> $ty {
            unsafe { $c_name() }
        }
    };
}

impl_checked_fn!(cvlr_nondet_u8, CVT_nondet_u8, u8);
impl_checked_fn!(cvlr_nondet_u16, CVT_nondet_u16, u16);
impl_checked_fn!(cvlr_nondet_u32, CVT_nondet_u32, u32);
impl_checked_fn!(cvlr_nondet_u64, CVT_nondet_u64, u64);
impl_checked_fn!(cvlr_nondet_u128, CVT_nondet_u128, u128);
impl_checked_fn!(cvlr_nondet_usize, CVT_nondet_usize, usize);

impl_checked_fn!(cvlr_nondet_i8, CVT_nondet_i8, i8);
impl_checked_fn!(cvlr_nondet_i16, CVT_nondet_i16, i16);
impl_checked_fn!(cvlr_nondet_i32, CVT_nondet_i32, i32);
impl_checked_fn!(cvlr_nondet_i64, CVT_nondet_i64, i64);
impl_checked_fn!(cvlr_nondet_i128, CVT_nondet_i128, i128);

impl_checked_fn!(cvlr_nondet_small_u128, CVT_nondet_small_u128, u128);
impl_checked_fn!(cvlr_nondet_small_i128, CVT_nondet_small_i128, i128);

crate::nondet_impl! { (), (),  "No nondet value for  unit" }
crate::nondet_impl! { bool, cvlr_nondet_u64() > 0, "Nondet for bool"}
crate::nondet_impl! { u8, cvlr_nondet_u8(), "Nondet for u8" }
crate::nondet_impl! { i8, cvlr_nondet_i8(), "Nondet for i8" }
crate::nondet_impl! { u16, cvlr_nondet_u16(), "Nondet for u16" }
crate::nondet_impl! { i16, cvlr_nondet_i16(), "Nondet for i16" }
crate::nondet_impl! { u32, cvlr_nondet_u32(), "Nondet for u32" }
crate::nondet_impl! { i32, cvlr_nondet_i32(), "Nondet for i32" }
crate::nondet_impl! { u64, cvlr_nondet_u64(), "Nondet for u64" }
crate::nondet_impl! { i64, cvlr_nondet_i64(), "Nondet for i64" }
crate::nondet_impl! { u128, cvlr_nondet_u128(), "Nondet for u128" }
crate::nondet_impl! { i128, cvlr_nondet_i128(), "Nondet for i128" }

crate::nondet_impl! { usize, cvlr_nondet_usize(), "Nondet for usize" }
