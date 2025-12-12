#[derive(Eq, Debug, Copy, Clone)]
/// Native Mathematical Integer (represented by u64 number)
///
/// The magic is that symbolically an SBF word is mapped to 256 bit symbolic
/// integer.
pub struct NativeIntU64(u64);

/// Declaration for external library for mathematical integers
///
/// This library is implemented symbolically by Certora Prover
/// Run-time under-approximation is provided in [rt_impls] module
mod rt_decls {
    type BoolU64 = u64;

    extern "C" {
        pub fn CVT_nativeint_u64_eq(_: u64, _: u64) -> BoolU64;
        pub fn CVT_nativeint_u64_lt(_: u64, _: u64) -> BoolU64;
        pub fn CVT_nativeint_u64_le(_: u64, _: u64) -> BoolU64;

        pub fn CVT_nativeint_u64_add(_: u64, _: u64) -> u64;
        pub fn CVT_nativeint_u64_sub(_: u64, _: u64) -> u64;
        pub fn CVT_nativeint_u64_mul(_: u64, _: u64) -> u64;
        pub fn CVT_nativeint_u64_div(_: u64, _: u64) -> u64;
        pub fn CVT_nativeint_u64_div_ceil(_: u64, _: u64) -> u64;
        pub fn CVT_nativeint_u64_muldiv(_: u64, _: u64, _: u64) -> u64;
        pub fn CVT_nativeint_u64_muldiv_ceil(_: u64, _: u64, _: u64) -> u64;

        pub fn CVT_nativeint_u64_nondet() -> u64;

        pub fn CVT_nativeint_u64_from_u128(w0: u64, w1: u64) -> u64;
        pub fn CVT_nativeint_u64_from_u256(w0: u64, w1: u64, w2: u64, w3: u64) -> u64;

        pub fn CVT_nativeint_u64_u64_max() -> u64;
        pub fn CVT_nativeint_u64_u128_max() -> u64;
        pub fn CVT_nativeint_u64_u256_max() -> u64;
    }
}

/// Run-time implementation of the external library
///
/// This implementation is intended as an under-approximation of the symbolic
/// behavior. It is intended to be used for testing.
#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_eq(a: u64, b: u64) -> u64 {
        (a == b).into()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_lt(a: u64, b: u64) -> u64 {
        (a < b).into()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_le(a: u64, b: u64) -> u64 {
        (a <= b).into()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_add(a: u64, b: u64) -> u64 {
        a.checked_add(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_mul(a: u64, b: u64) -> u64 {
        a.checked_mul(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_sub(a: u64, b: u64) -> u64 {
        a.checked_sub(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_div(a: u64, b: u64) -> u64 {
        a.checked_div(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_div_ceil(a: u64, b: u64) -> u64 {
        a.div_ceil(b)
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_muldiv(a: u64, b: u64, c: u64) -> u64 {
        a.checked_mul(b).unwrap().checked_div(c).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_muldiv_ceil(a: u64, b: u64, c: u64) -> u64 {
        a.checked_mul(b).unwrap().div_ceil(c)
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_nondet() -> u64 {
        // -- concrete implementation returns some specific number
        // -- it can, potentially, return a random number instead, or depend on
        // -- run-time of nondet
        0
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_from_u128(w0: u64, w1: u64) -> u64 {
        if w1 != 0 {
            panic!();
        }
        w0
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_from_u256(w0: u64, w1: u64, w2: u64, w3: u64) -> u64 {
        if w1 != 0 || w2 != 0 || w3 != 0 {
            panic!();
        }
        w0
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_u64_max() -> u64 {
        u64::MAX
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_u128_max() -> u64 {
        panic!();
    }

    #[no_mangle]
    pub extern "C" fn CVT_nativeint_u64_u256_max() -> u64 {
        panic!();
    }
}

use rt_decls::*;

impl NativeIntU64 {
    pub fn new<T>(v: T) -> Self
    where
        T: Into<NativeIntU64>,
    {
        v.into()
    }

    pub fn div_ceil(self, rhs: Self) -> Self {
        unsafe { Self(CVT_nativeint_u64_div_ceil(self.0, rhs.0)) }
    }

    pub fn muldiv(self, num: Self, den: Self) -> Self {
        unsafe { Self(CVT_nativeint_u64_muldiv(self.0, num.0, den.0)) }
    }

    pub fn muldiv_ceil(self, num: Self, den: Self) -> Self {
        unsafe { Self(CVT_nativeint_u64_muldiv_ceil(self.0, num.0, den.0)) }
    }

    pub fn from_u128(w0: u64, w1: u64) -> Self {
        unsafe { Self(CVT_nativeint_u64_from_u128(w0, w1)) }
    }

    pub fn from_u256(w0: u64, w1: u64, w2: u64, w3: u64) -> Self {
        unsafe { Self(CVT_nativeint_u64_from_u256(w0, w1, w2, w3)) }
    }

    pub fn u64_max() -> Self {
        unsafe { Self(CVT_nativeint_u64_u64_max()) }
    }

    pub fn u128_max() -> Self {
        unsafe { Self(CVT_nativeint_u64_u128_max()) }
    }

    pub fn u256_max() -> Self {
        unsafe { Self(CVT_nativeint_u64_u256_max()) }
    }

    pub fn is_u8(self) -> bool {
        self <= Self::new(u8::MAX as u64)
    }

    pub fn is_u16(self) -> bool {
        self <= Self::new(u16::MAX as u64)
    }

    pub fn is_u32(self) -> bool {
        self <= Self::new(u32::MAX as u64)
    }

    pub fn is_u64(self) -> bool {
        self <= Self::u64_max()
    }

    pub fn is_u128(self) -> bool {
        self <= Self::u128_max()
    }

    pub fn is_u256(self) -> bool {
        // native ints are 256 bits
        true
    }

    pub fn nondet() -> Self {
        cvlr_nondet::nondet()
    }

    pub fn checked_sub(&self, v: NativeIntU64) -> Self {
        *self - v
    }

    // Expose internal representation. Internal use only.
    pub fn as_internal(&self) -> u64 {
        self.0
    }
}

impl PartialEq for NativeIntU64 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { CVT_nativeint_u64_eq(self.0, other.0) != 0 }
    }
}

impl PartialOrd for NativeIntU64 {
    // We silence these two warnings from clippy: this code should be left as-is
    // for the Certora Prover TAC slicer.
    #[allow(clippy::non_canonical_partial_ord_impl)]
    #[allow(clippy::comparison_chain)]
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let ord = if self.0 == other.0 {
            core::cmp::Ordering::Equal
        } else if self.0 < other.0 {
            core::cmp::Ordering::Less
        } else {
            core::cmp::Ordering::Greater
        };
        Some(ord)
    }
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        unsafe { CVT_nativeint_u64_lt(self.0, other.0) != 0 }
    }
    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        unsafe { CVT_nativeint_u64_le(self.0, other.0) != 0 }
    }
    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        other.lt(self)
    }
    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        other.le(self)
    }
}

impl Ord for NativeIntU64 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.lt(other) {
            core::cmp::Ordering::Less
        } else if self.gt(other) {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    fn max(self, other: Self) -> Self {
        if self.gt(&other) {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self {
        if self.gt(&other) {
            other
        } else {
            self
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self.gt(&max) {
            max
        } else if self.lt(&min) {
            min
        } else {
            self
        }
    }
}

impl core::ops::Add<NativeIntU64> for NativeIntU64 {
    type Output = Self;

    fn add(self, rhs: NativeIntU64) -> Self::Output {
        unsafe { Self(CVT_nativeint_u64_add(self.0, rhs.0)) }
    }
}

impl core::ops::Sub<NativeIntU64> for NativeIntU64 {
    type Output = Self;

    fn sub(self, rhs: NativeIntU64) -> Self::Output {
        unsafe { Self(CVT_nativeint_u64_sub(self.0, rhs.0)) }
    }
}

impl core::ops::Mul<NativeIntU64> for NativeIntU64 {
    type Output = Self;

    fn mul(self, rhs: NativeIntU64) -> Self::Output {
        unsafe { Self(CVT_nativeint_u64_mul(self.0, rhs.0)) }
    }
}

impl core::ops::Div<NativeIntU64> for NativeIntU64 {
    type Output = Self;

    fn div(self, rhs: NativeIntU64) -> Self::Output {
        unsafe { Self(CVT_nativeint_u64_div(self.0, rhs.0)) }
    }
}

macro_rules! impl_from_for_small_uint {
    ($uint:ty) => {
        impl From<$uint> for NativeIntU64 {
            fn from(value: $uint) -> Self {
                Self(value as u64)
            }
        }
    };
}

macro_rules! impl_core_traits_for_num {
    ($num:ty) => {
        impl core::ops::Add<$num> for NativeIntU64 {
            type Output = Self;

            fn add(self, rhs: $num) -> Self::Output {
                self + Self::from(rhs)
            }
        }

        impl core::ops::Mul<$num> for NativeIntU64 {
            type Output = Self;

            fn mul(self, rhs: $num) -> Self::Output {
                self * Self::from(rhs)
            }
        }

        impl core::ops::Div<$num> for NativeIntU64 {
            type Output = Self;

            fn div(self, rhs: $num) -> Self::Output {
                self / Self::from(rhs)
            }
        }

        impl PartialEq<$num> for NativeIntU64 {
            #[inline(always)]
            fn eq(&self, other: &$num) -> bool {
                *self == Self::from(*other)
            }
        }

        impl PartialOrd<$num> for NativeIntU64 {
            #[inline(always)]
            fn partial_cmp(&self, other: &$num) -> Option<core::cmp::Ordering> {
                self.partial_cmp(&Self::from(*other))
            }
            #[inline(always)]
            fn lt(&self, other: &$num) -> bool {
                *self < Self::from(*other)
            }
            #[inline(always)]
            fn le(&self, other: &$num) -> bool {
                *self <= Self::from(*other)
            }
            #[inline(always)]
            fn gt(&self, other: &$num) -> bool {
                *self > Self::from(*other)
            }
            #[inline(always)]
            fn ge(&self, other: &$num) -> bool {
                *self >= Self::from(*other)
            }
        }
    };
}

impl_from_for_small_uint!(u8);
impl_from_for_small_uint!(u16);
impl_from_for_small_uint!(u32);
impl_from_for_small_uint!(u64);

impl From<u128> for NativeIntU64 {
    fn from(value: u128) -> Self {
        // let w0: u64 = (value & 0xffff_ffff_ffff_ffff) as u64;
        let w0: u64 = value as u64;
        let w1: u64 = (value >> 64) as u64;

        Self::from_u128(w0, w1)
    }
}

impl_core_traits_for_num!(u8);
impl_core_traits_for_num!(u16);
impl_core_traits_for_num!(u32);
impl_core_traits_for_num!(u64);
impl_core_traits_for_num!(u128);

impl From<i32> for NativeIntU64 {
    fn from(value: i32) -> Self {
        if value.is_positive() {
            Self::from(value as u64)
        } else {
            Self::from(0u64) - Self::from((value as i64).unsigned_abs())
        }
    }
}
impl_core_traits_for_num!(i32);

impl From<NativeIntU64> for u64 {
    fn from(value: NativeIntU64) -> Self {
        cvlr_asserts::cvlr_assume!(value.is_u64());
        value.as_internal()
    }
}

impl From<NativeIntU64> for u128 {
    fn from(value: NativeIntU64) -> Self {
        cvlr_asserts::cvlr_assume!(value.is_u128());
        let res: u128 = cvlr_nondet::nondet();
        cvlr_asserts::cvlr_assume!(value == res);
        res
    }
}

impl From<&[u64; 2]> for NativeIntU64 {
    #[inline(always)]
    fn from(value: &[u64; 2]) -> Self {
        Self::from_u128(value[0], value[1])
    }
}

impl From<&[u64; 4]> for NativeIntU64 {
    #[inline(always)]
    fn from(value: &[u64; 4]) -> Self {
        Self::from_u256(value[0], value[1], value[2], value[3])
    }
}

impl From<&[u8; 32]> for NativeIntU64 {
    #[inline(always)]
    fn from(value: &[u8; 32]) -> Self {
        let (w0, rest) = value.split_at(8);
        let w0 = u64::from_le_bytes(w0.try_into().unwrap());
        let (w1, rest) = rest.split_at(8);
        let w1 = u64::from_le_bytes(w1.try_into().unwrap());
        let (w2, rest) = rest.split_at(8);
        let w2 = u64::from_le_bytes(w2.try_into().unwrap());
        let w3 = u64::from_le_bytes(rest.try_into().unwrap());
        unsafe { Self(CVT_nativeint_u64_from_u256(w0, w1, w2, w3)) }
    }
}

impl From<&[u8]> for NativeIntU64 {
    #[inline(always)]
    fn from(value: &[u8]) -> Self {
        let v: &[u8; 32] = value.try_into().unwrap();
        Self::from(v)
    }
}

impl cvlr_nondet::Nondet for NativeIntU64 {
    fn nondet() -> NativeIntU64 {
        unsafe { Self(CVT_nativeint_u64_nondet()) }
    }
}

macro_rules! impl_is_uint {
    ($name:ident, $uint:ty, $is_uint:ident) => {
        pub fn $name(v: $uint) -> bool {
            NativeIntU64::from(v).$is_uint()
        }
    };
}

impl_is_uint! { is_u8, u8, is_u8 }
impl_is_uint! { is_u16, u16, is_u16 }
impl_is_uint! { is_u32, u32, is_u32 }
impl_is_uint! { is_u64, u64, is_u64 }
impl_is_uint! { is_u128, u128, is_u128 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x: NativeIntU64 = 2.into();
        let y: NativeIntU64 = 4.into();
        assert_eq!(x + y, 6);
        assert!(x < 6);
    }

    #[test]
    fn nondet_test() {
        let x: NativeIntU64 = cvlr_nondet::nondet();
        assert_eq!(x, 0);
    }

    #[test]
    fn test_arithmetic_operations() {
        let a: NativeIntU64 = 10.into();
        let b: NativeIntU64 = 3.into();

        // Addition
        assert_eq!(a + b, 13);
        assert_eq!(a + 5, 15);

        // Subtraction
        assert_eq!(a - b, 7);
        // Note: b - a would underflow and panic in rt mode

        // Multiplication
        assert_eq!(a * b, 30);
        assert_eq!(a * 2, 20);

        // Division
        assert_eq!(a / b, 3);
        assert_eq!(a / 2, 5);
    }

    #[test]
    fn test_comparison_operations() {
        let a: NativeIntU64 = 5.into();
        let b: NativeIntU64 = 10.into();
        let c: NativeIntU64 = 5.into();

        // Equality
        assert_eq!(a, c);
        assert_ne!(a, b);

        // Less than
        assert!(a < b);
        assert!(!(b < a));
        assert!(!(a < c));

        // Less than or equal
        assert!(a <= b);
        assert!(a <= c);
        assert!(!(b <= a));

        // Greater than
        assert!(b > a);
        assert!(!(a > b));
        assert!(!(a > c));

        // Greater than or equal
        assert!(b >= a);
        assert!(a >= c);
        assert!(!(a >= b));
    }

    #[test]
    fn test_div_ceil() {
        let a: NativeIntU64 = 10.into();
        let b: NativeIntU64 = 3.into();
        assert_eq!(a.div_ceil(b), 4); // 10/3 = 3.33... -> 4

        let c: NativeIntU64 = 9.into();
        let d: NativeIntU64 = 3.into();
        assert_eq!(c.div_ceil(d), 3); // 9/3 = 3 -> 3

        let e: NativeIntU64 = 11.into();
        let f: NativeIntU64 = 5.into();
        assert_eq!(e.div_ceil(f), 3); // 11/5 = 2.2 -> 3
    }

    #[test]
    fn test_muldiv() {
        let a: NativeIntU64 = 10.into();
        let b: NativeIntU64 = 3.into();
        let c: NativeIntU64 = 2.into();
        // (10 * 3) / 2 = 30 / 2 = 15
        assert_eq!(a.muldiv(b, c), 15);

        let d: NativeIntU64 = 100.into();
        let e: NativeIntU64 = 7.into();
        let f: NativeIntU64 = 4.into();
        // (100 * 7) / 4 = 700 / 4 = 175
        assert_eq!(d.muldiv(e, f), 175);
    }

    #[test]
    fn test_muldiv_ceil() {
        let a: NativeIntU64 = 10.into();
        let b: NativeIntU64 = 3.into();
        let c: NativeIntU64 = 4.into();
        // (10 * 3) / 4 = 30 / 4 = 7.5 -> 8
        assert_eq!(a.muldiv_ceil(b, c), 8);

        let d: NativeIntU64 = 10.into();
        let e: NativeIntU64 = 3.into();
        let f: NativeIntU64 = 5.into();
        // (10 * 3) / 5 = 30 / 5 = 6 -> 6
        assert_eq!(d.muldiv_ceil(e, f), 6);
    }

    #[test]
    fn test_from_u128() {
        let val = NativeIntU64::from_u128(42, 0);
        assert_eq!(val, 42);

        let val2 = NativeIntU64::from_u128(0x1234_5678_9abc_def0, 0);
        assert_eq!(val2, 0x1234_5678_9abc_def0u64);
    }

    #[test]
    fn test_from_u256() {
        let val = NativeIntU64::from_u256(100, 0, 0, 0);
        assert_eq!(val, 100);

        let val2 = NativeIntU64::from_u256(0xffff_ffff_ffff_ffff, 0, 0, 0);
        assert_eq!(val2, 0xffff_ffff_ffff_ffffu64);
    }

    #[test]
    fn test_from_primitive_types() {
        // From u8
        let val_u8: NativeIntU64 = 42u8.into();
        assert_eq!(val_u8, 42);

        // From u16
        let val_u16: NativeIntU64 = 1000u16.into();
        assert_eq!(val_u16, 1000);

        // From u32
        let val_u32: NativeIntU64 = 1_000_000u32.into();
        assert_eq!(val_u32, 1_000_000);

        // From u64
        let val_u64: NativeIntU64 = 1_000_000_000u64.into();
        assert_eq!(val_u64, 1_000_000_000);

        // From u128
        let val_u128: NativeIntU64 = 1_000_000_000_000u128.into();
        assert_eq!(val_u128, 1_000_000_000_000u64);
    }

    #[test]
    fn test_from_i32() {
        let val_pos: NativeIntU64 = 42i32.into();
        assert_eq!(val_pos, 42);

        let val_zero: NativeIntU64 = 0i32.into();
        assert_eq!(val_zero, 0);

        // Note: Negative i32 values cause underflow in rt mode when converting
        // (0 - abs(value)), so we only test positive values here
    }

    #[test]
    fn test_from_array_u64_2() {
        let arr = [42u64, 0u64];
        let val: NativeIntU64 = (&arr).into();
        assert_eq!(val, 42);
    }

    #[test]
    fn test_from_array_u64_4() {
        let arr = [100u64, 0u64, 0u64, 0u64];
        let val: NativeIntU64 = (&arr).into();
        assert_eq!(val, 100);
    }

    #[test]
    fn test_from_array_u8_32() {
        let mut arr = [0u8; 32];
        // Set first 8 bytes to represent 0x1234567890abcdef in little-endian
        arr[0..8].copy_from_slice(&0x1234567890abcdefu64.to_le_bytes());
        let val: NativeIntU64 = (&arr).into();
        assert_eq!(val, 0x1234567890abcdefu64);
    }

    #[test]
    fn test_max_functions() {
        let u64_max = NativeIntU64::u64_max();
        assert_eq!(u64_max, u64::MAX);
    }

    #[test]
    fn test_is_uint_functions() {
        // Test is_u8
        assert!(NativeIntU64::from(0u64).is_u8());
        assert!(NativeIntU64::from(255u64).is_u8());
        assert!(!NativeIntU64::from(256u64).is_u8());

        // Test is_u16
        assert!(NativeIntU64::from(0u64).is_u16());
        assert!(NativeIntU64::from(65535u64).is_u16());
        assert!(!NativeIntU64::from(65536u64).is_u16());

        // Test is_u32
        assert!(NativeIntU64::from(0u64).is_u32());
        assert!(NativeIntU64::from(4294967295u32).is_u32());
        assert!(!NativeIntU64::from(4294967296u64).is_u32());

        // Test is_u64
        assert!(NativeIntU64::from(0u64).is_u64());
        assert!(NativeIntU64::from(u64::MAX).is_u64());

        // Test is_u128
        // Note: u128_max() panics in rt mode, so we skip testing is_u128()
        // as it would call u128_max() which panics

        // Test is_u256
        assert!(NativeIntU64::from(0u64).is_u256());
        assert!(NativeIntU64::from(u64::MAX).is_u256());
    }

    #[test]
    fn test_ord_trait_methods() {
        let a: NativeIntU64 = 5.into();
        let b: NativeIntU64 = 10.into();
        let c: NativeIntU64 = 7.into();

        // Test cmp
        assert_eq!(a.cmp(&b), core::cmp::Ordering::Less);
        assert_eq!(b.cmp(&a), core::cmp::Ordering::Greater);
        assert_eq!(a.cmp(&a), core::cmp::Ordering::Equal);

        // Test max
        assert_eq!(a.max(b), b);
        assert_eq!(b.max(a), b);
        assert_eq!(a.max(a), a);

        // Test min
        assert_eq!(a.min(b), a);
        assert_eq!(b.min(a), a);
        assert_eq!(a.min(a), a);

        // Test clamp
        assert_eq!(c.clamp(a, b), c); // c is between a and b
        assert_eq!(a.clamp(c, b), c); // a is less than min
        assert_eq!(b.clamp(a, c), c); // b is greater than max
        assert_eq!(a.clamp(a, b), a); // a is at min
        assert_eq!(b.clamp(a, b), b); // b is at max
    }

    #[test]
    fn test_edge_cases() {
        // Zero
        let zero: NativeIntU64 = 0.into();
        assert_eq!(zero + zero, zero);
        assert_eq!(zero * 100, zero);
        assert_eq!(zero / 1, zero);

        // One
        let one: NativeIntU64 = 1.into();
        assert_eq!(zero + one, one);
        assert_eq!(one * 100, 100);
        assert_eq!(one / one, one);

        // Maximum u64
        let max: NativeIntU64 = u64::MAX.into();
        assert_eq!(max, NativeIntU64::u64_max());
        assert!(max.is_u64());
    }

    #[test]
    fn test_checked_sub() {
        let a: NativeIntU64 = 10.into();
        let b: NativeIntU64 = 3.into();
        assert_eq!(a.checked_sub(b), 7);

        // Note: checked_sub is just an alias for sub, so underflow will panic in rt mode
        // We test the normal case only
    }

    #[test]
    fn test_new_method() {
        let val1 = NativeIntU64::from(42u8);
        assert_eq!(val1, 42u64);

        let val2 = NativeIntU64::from(1000u16);
        assert_eq!(val2, 1000u64);

        let val3 = NativeIntU64::from(1_000_000u32);
        assert_eq!(val3, 1_000_000u64);
    }

    #[test]
    fn test_as_internal() {
        let val = NativeIntU64::from(42);
        assert_eq!(val.as_internal(), 42u64);

        let max = NativeIntU64::from(u64::MAX);
        assert_eq!(max.as_internal(), u64::MAX);
    }

    #[test]
    fn test_arithmetic_with_primitives() {
        let a: NativeIntU64 = 10.into();

        // Addition with primitives
        assert_eq!(a + 5u8, 15);
        assert_eq!(a + 5u16, 15);
        assert_eq!(a + 5u32, 15);
        assert_eq!(a + 5u64, 15);
        assert_eq!(a + 5u128, 15);

        // Multiplication with primitives
        assert_eq!(a * 3u8, 30);
        assert_eq!(a * 3u16, 30);
        assert_eq!(a * 3u32, 30);
        assert_eq!(a * 3u64, 30);
        assert_eq!(a * 3u128, 30);

        // Division with primitives
        assert_eq!(a / 2u8, 5);
        assert_eq!(a / 2u16, 5);
        assert_eq!(a / 2u32, 5);
        assert_eq!(a / 2u64, 5);
        assert_eq!(a / 2u128, 5);
    }
}
