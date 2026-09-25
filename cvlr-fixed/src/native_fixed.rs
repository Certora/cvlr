use cvlr_asserts::cvlr_assume;
use cvlr_mathint::NativeInt;
use cvlr_nondet::nondet;

macro_rules! native_fixed {
    ($NativeFixed:ident, $uint:ty, $is_uint:ident) => {
        #[derive(Copy, Clone, Eq, Debug)]
        /// Native Fixed point numbers with F bits of precision
        ///
        /// A non-negative binary fixed-point number whose value is
        /// `bits / 2^F`, where `bits` is a [`NativeInt`] that is always assumed
        /// to fit in
        #[doc = concat!("`", stringify!($uint), "`. `F` must be less than 64.")]
        ///
        /// Intended for modeling fixed-point types in the code under
        /// verification (e.g. a Q64.64 value stored as a
        #[doc = concat!("`", stringify!($uint), "`) and reasoning about them")]
        /// with [`NativeInt`] arithmetic. Intermediate results are computed in
        /// 256 bits, so products of two representable values (and the other
        /// intermediates below) are exact and do not wrap.
        ///
        /// # Out-of-range results are assumed away
        ///
        /// No operation returns an error or panics on overflow or underflow.
        /// Instead, operations that can leave the representable range add a
        /// Prover assumption (`cvlr_assume!`) that the result is in range, so
        /// executions where it would not be are silently dropped from the
        /// analysis. In particular the `checked_*` methods do not return
        /// `Option`; "checked" means "assumed not to overflow". If the code
        /// under verification can overflow, check that explicitly (e.g. by
        /// comparing against the bound with `cvlr_assert!`) rather than relying
        /// on these operations to report it, and use `cvlr_satisfy!` to confirm
        /// that the interesting paths remain reachable.
        ///
        /// # Operators
        ///
        /// * `a + b`, `a - b`, `a * b`, `a / b` with both operands of this type
        ///   are [`checked_add`](Self::checked_add),
        ///   [`checked_sub`](Self::checked_sub),
        ///   [`checked_mul`](Self::checked_mul) and
        ///   [`checked_div`](Self::checked_div).
        /// * `a * n` and `a / n` with `n: impl Into<NativeInt>` are
        ///   [`mul_by_int`](Self::mul_by_int) and
        ///   [`div_by_int`](Self::div_by_int).
        /// * `==`, `<`, `<=`, `>`, `>=`, `min`, `max`, `clamp` between two
        ///   values of this type compare the exact values.
        /// * `From<T: Into<NativeInt>>` is [`new`](Self::new): it converts an
        ///   integer, not raw bits.
        ///
        /// [`Nondet`](cvlr_nondet::Nondet) produces an arbitrary representable
        /// value, and [`CvlrLog`](cvlr_log::CvlrLog) shows the value in the
        /// counterexample as a fixed-point number with `F` fractional bits.
        pub struct $NativeFixed<const F: u32> {
            val: NativeInt,
        }

        impl<const F: u32> $NativeFixed<F> {
            const FRAC: u32 = F;
            const BASE: u64 = 2u64.pow(Self::FRAC);

            /// The fixed-point number equal to the integer `v`.
            ///
            #[doc = concat!("Assumes `v * 2^F` fits in `", stringify!($uint), "`.")]
            /// To construct from a raw representation instead, use
            /// [`from_bits`](Self::from_bits).
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

            /// The value `1` (raw bits `2^F`).
            pub fn one() -> Self {
                Self::from_bits(Self::BASE as $uint)
            }

            /// The raw representation, i.e. `self * 2^F`.
            ///
            /// This is the inverse of [`from_bits`](Self::from_bits); use it to
            /// compare with the stored integer of the fixed-point type being
            /// modeled.
            pub fn to_bits(&self) -> $uint {
                cvlr_assume!(self.val.$is_uint());
                self.val.into()
            }

            /// The number whose raw representation is `bits`, i.e.
            /// `bits / 2^F`.
            ///
            /// Use this to reinterpret a fixed-point value read from program
            /// state. To convert an integer, use [`new`](Self::new).
            pub fn from_bits(bits: $uint) -> Self {
                Self { val: bits.into() }
            }

            /// `self * v`.
            ///
            /// Assumes the result is representable. The product is computed as
            /// a [`NativeInt`], so it is exact as long as it is below `2^256`.
            pub fn mul_by_int(&self, v: NativeInt) -> Self {
                Self::from_val(self.val * v)
            }

            /// `self / v`, rounded down to a multiple of `2^-F`.
            ///
            /// `v` is not checked: division by zero follows [`NativeInt`]
            /// division, which yields zero, and is not assumed away.
            pub fn div_by_int(&self, v: NativeInt) -> Self {
                Self::from_val(self.val / v)
            }

            /// `self * v`, rounded down to a multiple of `2^-F`.
            ///
            /// The raw product is computed exactly before rounding, so no
            /// precision is lost to intermediate overflow. Assumes the result
            /// is representable; does not return `Option`.
            pub fn checked_mul(&self, v: Self) -> Self {
                Self::from_val((self.val * v.val) / Self::BASE)
            }

            /// `self + v`.
            ///
            /// Assumes the result is representable; does not return `Option`.
            pub fn checked_add(&self, v: Self) -> Self {
                Self::from_val(self.val + v.val)
            }

            /// `self / v`, rounded down to a multiple of `2^-F`.
            ///
            /// Assumes `v` is non-zero and that the result is representable;
            /// does not return `Option`. Executions where `v` is zero are
            /// dropped from the analysis.
            pub fn checked_div(&self, v: Self) -> Self {
                cvlr_assume!(v.val > 0u64);
                Self::from_val(self.val * Self::BASE / v.val)
            }

            /// `self - v`, or zero if `v >= self`.
            pub fn saturating_sub(&self, v: Self) -> Self {
                let val = if self.val <= v.val {
                    0u64.into()
                } else {
                    self.val - v.val
                };
                Self::from_val(val)
            }

            /// `self - v`.
            ///
            /// Assumes `self >= v`; does not return `Option`. Executions where
            /// the subtraction would underflow are dropped from the analysis.
            /// Use [`saturating_sub`](Self::saturating_sub) to clamp at zero
            /// instead.
            pub fn checked_sub(&self, v: Self) -> Self {
                cvlr_assume!(self.val >= v.val);
                let val = self.val - v.val;
                Self { val }
            }

            /// `floor(self) >= v`.
            ///
            /// Because `v` is an integer, this equals the exact comparison
            /// `self >= v`.
            pub fn ge(&self, v: NativeInt) -> bool {
                self.to_floor() >= v
            }

            /// `floor(self) > v`.
            ///
            /// Compares the integer part only, so this is **not** the exact
            /// comparison `self > v`: for the value 2.5, `gt(2)` is `false`. To
            /// compare exactly, use `*self > Self::new(v)`.
            pub fn gt(&self, v: NativeInt) -> bool {
                self.to_floor() > v
            }

            /// `floor(self) <= v`.
            ///
            /// Compares the integer part only, so this is **not** the exact
            /// comparison `self <= v`: for the value 2.5, `le(2)` is `true`. To
            /// compare exactly, use `*self <= Self::new(v)`.
            pub fn le(&self, v: NativeInt) -> bool {
                self.to_floor() <= v
            }

            /// `floor(self) < v`.
            ///
            /// Because `v` is an integer, this equals the exact comparison
            /// `self < v`.
            pub fn lt(&self, v: NativeInt) -> bool {
                self.to_floor() < v
            }

            /// The largest integer not greater than `self`.
            pub fn to_floor(&self) -> NativeInt {
                self.val / Self::BASE
            }

            /// [`to_floor`](Self::to_floor) as a fixed-point number.
            pub fn floor(&self) -> Self {
                self.to_floor().into()
            }

            /// The smallest integer not less than `self`.
            pub fn to_ceil(&self) -> NativeInt {
                let floor = self.to_floor();
                let rem = *self - Self::new(floor);

                if rem.val > 0 {
                    floor + 1
                } else {
                    floor
                }
            }

            /// [`to_ceil`](Self::to_ceil) as a fixed-point number.
            ///
            /// Assumes the result is representable.
            pub fn ceil(&self) -> Self {
                self.to_ceil().into()
            }
        }

        impl<const F: u32> cvlr_nondet::Nondet for $NativeFixed<F> {
            fn nondet() -> Self {
                Self::from_val(nondet())
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
                logger.log_u64_as_fp(tag, self.val.as_internal(), F as u64);
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

        impl<const F: u32> core::ops::Div<$NativeFixed<F>> for $NativeFixed<F> {
            type Output = Self;

            fn div(self, v: Self) -> Self::Output {
                self.checked_div(v)
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
