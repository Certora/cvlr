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
    fn eq(&self, other: &Self) -> bool {
        unsafe { CVT_nativeint_u64_eq(self.0, other.0) != 0 }
    }
}

impl PartialOrd for NativeIntU64 {
    // We silence these two warnings from clippy: this code should be left as-is
    // for the Certora Prover TAC slicer.
    #[allow(clippy::non_canonical_partial_ord_impl)]
    #[allow(clippy::comparison_chain)]
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
    fn lt(&self, other: &Self) -> bool {
        unsafe { CVT_nativeint_u64_lt(self.0, other.0) != 0 }
    }
    fn le(&self, other: &Self) -> bool {
        unsafe { CVT_nativeint_u64_le(self.0, other.0) != 0 }
    }
    fn gt(&self, other: &Self) -> bool {
        other.lt(self)
    }
    fn ge(&self, other: &Self) -> bool {
        other.le(self)
    }
}

impl Ord for NativeIntU64 {
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

macro_rules! impl_core_traits_for_uint {
    ($uint:ty) => {
        impl core::ops::Add<$uint> for NativeIntU64 {
            type Output = Self;

            fn add(self, rhs: $uint) -> Self::Output {
                self + Self::from(rhs)
            }
        }

        impl core::ops::Mul<$uint> for NativeIntU64 {
            type Output = Self;

            fn mul(self, rhs: $uint) -> Self::Output {
                self * Self::from(rhs)
            }
        }

        impl core::ops::Div<$uint> for NativeIntU64 {
            type Output = Self;

            fn div(self, rhs: $uint) -> Self::Output {
                self / Self::from(rhs)
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

impl_core_traits_for_uint!(u8);
impl_core_traits_for_uint!(u16);
impl_core_traits_for_uint!(u32);
impl_core_traits_for_uint!(u64);
impl_core_traits_for_uint!(u128);

impl From<i32> for NativeIntU64 {
    fn from(value: i32) -> Self {
        value
            .is_positive()
            .then(|| Self::from(value as u32))
            .unwrap_or_else(|| Self::from(0u64) - Self::from((value as i64).unsigned_abs()))
    }
}

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
        cvlr_asserts::cvlr_assume!(value == res.into());
        res
    }
}

impl From<&[u64; 2]> for NativeIntU64 {
    fn from(value: &[u64; 2]) -> Self {
        Self::from_u128(value[0], value[1])
    }
}

impl From<&[u64; 4]> for NativeIntU64 {
    fn from(value: &[u64; 4]) -> Self {
        Self::from_u256(value[0], value[1], value[2], value[3])
    }
}

impl From<&[u8; 32]> for NativeIntU64 {
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
        let x = NativeIntU64(2);
        let y = NativeIntU64(4);
        assert_eq!(x + y, NativeIntU64(6));
        assert_eq!(x + y, 6u64.into());
        assert!(x < 6u64.into());
    }

    #[test]
    fn nondet_test() {
        let x: NativeIntU64 = cvlr_nondet::nondet();
        assert_eq!(x, 0u64.into());
    }
}
