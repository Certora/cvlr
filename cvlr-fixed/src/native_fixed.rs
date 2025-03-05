use cvlr_asserts::cvlr_assume;
use cvlr_mathint::NativeInt;
use cvlr_nondet::nondet;

macro_rules! native_fixed {
    ($NativeFixed:ident, $uint:ty, $is_uint:ident) => {
        #[derive(Copy, Clone, Eq, Debug)]
        /// Native Fixed point numbers with F bits of precision
        pub struct $NativeFixed<const F: u32> {
            val: NativeInt,
        }

        impl<const F: u32> $NativeFixed<F> {
            const FRAC: u32 = F;
            const BASE: u64 = 2u64.pow(Self::FRAC);

            pub fn new(v: NativeInt) -> Self {
                let val = v * Self::BASE;
                cvlr_assume!(val.$is_uint());
                Self { val }
            }

            #[inline(always)]
            fn from_val(val: NativeInt) -> Self {
                cvlr_assume!(val.$is_uint());
                Self { val }
            }

            pub fn one() -> Self {
                Self::from_bits(Self::BASE as $uint)
            }

            pub fn to_bits(&self) -> $uint {
                cvlr_assume!(self.val.is_u64());
                self.val.into()
            }

            pub fn from_bits(bits: $uint) -> Self {
                Self { val: bits.into() }
            }

            pub fn mul_by_int(&self, v: NativeInt) -> Self {
                Self::from_val(self.val * v)
            }

            pub fn div_by_int(&self, v: NativeInt) -> Self {
                Self::from_val(self.val / v)
            }

            pub fn checked_mul(&self, v: Self) -> Self {
                Self::from_val((self.val * v.val) / Self::BASE)
            }

            pub fn checked_add(&self, v: Self) -> Self {
                Self::from_val(self.val + v.val)
            }

            pub fn saturating_sub(&self, v: Self) -> Self {
                let val = if self.val <= v.val {
                    0u64.into()
                } else {
                    self.val - v.val
                };
                Self::from_val(val)
            }

            pub fn checked_sub(&self, v: Self) -> Self {
                cvlr_assume!(self.val >= v.val);
                let val = self.val - v.val;
                Self { val }
            }

            pub fn ge(&self, v: NativeInt) -> bool {
                self.to_floor() >= v
            }

            pub fn gt(&self, v: NativeInt) -> bool {
                self.to_floor() > v
            }

            pub fn le(&self, v: NativeInt) -> bool {
                self.to_floor() <= v
            }

            pub fn lt(&self, v: NativeInt) -> bool {
                self.to_floor() < v
            }

            pub fn to_floor(&self) -> NativeInt {
                self.val / Self::BASE
            }

            pub fn floor(&self) -> Self {
                self.to_floor().into()
            }
        }

        impl<const F: u32> cvlr_nondet::Nondet for $NativeFixed<F> {
            fn nondet() -> Self {
                Self::new(nondet())
            }
        }

        impl<const F: u32, T: Into<NativeInt>> From<T> for $NativeFixed<F> {
            fn from(value: T) -> Self {
                Self::new(value.into())
            }
        }

        impl<const F: u32> cvlr_log::CvlrLog for $NativeFixed<F> {
            #[inline(always)]
            fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
                cvlr_log::cvlr_log_with(tag, &self.to_floor(), logger);
                cvlr_log::cvlr_log_with(&"\tbits", &self.val, logger);
            }
        }

        impl<const F: u32> core::ops::Add<$NativeFixed<F>> for $NativeFixed<F> {
            type Output = Self;

            fn add(self, v: Self) -> Self::Output {
                self.checked_add(v)
            }
        }

        impl<const F: u32> core::ops::Sub<$NativeFixed<F>> for $NativeFixed<F> {
            type Output = Self;

            fn sub(self, v: Self) -> Self::Output {
                self.checked_sub(v)
            }
        }

        impl<const F: u32> core::ops::Mul<$NativeFixed<F>> for $NativeFixed<F> {
            type Output = Self;

            fn mul(self, v: Self) -> Self::Output {
                self.checked_mul(v)
            }
        }

        impl<const F: u32, T: Into<NativeInt>> core::ops::Mul<T> for $NativeFixed<F> {
            type Output = Self;

            fn mul(self, v: T) -> Self::Output {
                self.mul_by_int(v.into())
            }
        }

        impl<const F: u32, T: Into<NativeInt>> core::ops::Div<T> for $NativeFixed<F> {
            type Output = Self;

            fn div(self, v: T) -> Self::Output {
                self.div_by_int(v.into())
            }
        }

        impl<const F: u32> core::cmp::PartialEq for $NativeFixed<F> {
            fn eq(&self, other: &Self) -> bool {
                self.val == other.val
            }
        }

        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl<const F: u32> core::cmp::PartialOrd for $NativeFixed<F> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.val.partial_cmp(&other.val)
            }
            fn lt(&self, other: &Self) -> bool {
                self.val.lt(&other.val)
            }
            fn le(&self, other: &Self) -> bool {
                self.val.le(&other.val)
            }
            fn gt(&self, other: &Self) -> bool {
                self.val.gt(&other.val)
            }
            fn ge(&self, other: &Self) -> bool {
                self.val.ge(&other.val)
            }
        }

        impl<const F: u32> core::cmp::Ord for $NativeFixed<F> {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.val.cmp(&other.val)
            }

            fn max(self, other: Self) -> Self {
                if self > other {
                    self
                } else {
                    other
                }
            }

            fn min(self, other: Self) -> Self {
                if self > other {
                    other
                } else {
                    self
                }
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                if self > max {
                    max
                } else if self < min {
                    min
                } else {
                    self
                }
            }
        }
    };
}

native_fixed! { NativeFixedU64, u64, is_u64 }
native_fixed! { NativeFixedU128, u128, is_u128 }
