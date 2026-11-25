//! A verification-only stand-in for a 128-bit [`fixed`] type that computes
//! exactly what the type computes, with its multiplication and division done
//! on the Prover's native integers.
//!
//! Programs that keep fractions as [`FixedU128`] (`U68F60`, for example)
//! multiply and divide through 256-bit intermediates: a product is a 256-bit
//! multiplication and a shift, a quotient a 256-by-128-bit long division. The
//! Prover pays for every bit of those. [`ExactFixedU128`] wraps the program's
//! own [`FixedU128<Frac>`] and delegates every operation to it, except
//! fixed-point multiplication and division, which it computes as one
//! `cvlr-mathint` `muldiv` that the Prover models exactly. Swap it in for the
//! program's type under the verification feature, for example by replacing
//!
//! ```ignore
//! pub use fixed::types::U68F60 as Fraction;
//! ```
//!
//! with
//!
//! ```ignore
//! pub type Fraction = cvlr_fixed::exact::ExactFixedU128<fixed::types::extra::U60>;
//! ```
//!
//! A literal macro such as `fixed_macro`'s `U68F60!` is replaced with one that
//! expands to [`exact_lit!`](crate::exact_lit), which evaluates the literal at
//! compile time as `fixed_macro` does:
//!
//! ```ignore
//! macro_rules! fraction {
//!     ($v:literal) => { cvlr_fixed::exact_lit!($crate::Fraction, $v) };
//! }
//! ```
//!
//! # What it changes
//!
//! Nothing about any value. Every operation returns what the wide type's
//! returns, fails where it fails and panics with its message. The value is a
//! [`FixedU128<Frac>`], so [`from_bits`](ExactFixedU128::from_bits) and
//! [`to_bits`](ExactFixedU128::to_bits) are the wide type's, `const` included,
//! every constant of the program keeps its value, and a struct holding the
//! type has the wide type's layout.
//!
//! Fixed-point `*` and `/`, and their `checked_`, `saturating_`, `wrapping_`,
//! `unwrapped_` and `overflowing_` forms, compute `floor(a * b / 2^Frac)` and
//! `floor(a * 2^Frac / b)` on the encodings, as `fixed` does, and wrap modulo
//! `2^128` where `fixed` wraps. Division by zero panics with "division by
//! zero" in every form but `checked_div`, which returns `None`.
//!
//! # Operators
//!
//! `fixed`'s `*` and `/` between two fixed-point values wrap on overflow and
//! `debug_assert!` that they did not, so they panic or wrap depending on
//! whether `fixed` itself is built with debug assertions. Many Solana programs
//! turn them on for `fixed` in release (`[profile.release.package.fixed]
//! debug-assertions = true`). This type's operators make the same choice:
//! on an overflow they evaluate the wide type's own `*` on two constants whose
//! product overflows, so its `debug_assert!`, compiled with `fixed`'s profile,
//! decides between the panic and the wrapped result.
//!
//! Every other operator, and every method not listed above, is the wide type's
//! own code.
//!
//! Only the operations listed on [`ExactFixedU128`] are provided. A program
//! that uses another method of its fixed-point type (`sqrt`, `recip`,
//! `mul_add`, `%`, shifts, bitwise operators, comparisons with floats) does not
//! compile against this one.
//!
//! # What the Prover sees
//!
//! A product or quotient is one native-integer `muldiv` and two range checks
//! against constants, where the wide type's is a 256-bit computation of a few
//! hundred instructions with a long-division loop. The rest of the arithmetic
//! is unchanged. On the host (the `rt` feature), where native integers are 64
//! bits wide, every operation is the wide type's.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;

use fixed::traits::{Fixed, FromFixed, ToFixed};
use fixed::types::extra::{IsLessOrEqual, LeEqU128, True, U127};
use fixed::{FixedU128, ParseFixedError};

#[cfg(not(feature = "rt"))]
use cvlr_mathint::NativeInt;

/// `2^k` for a constant `k <= 128`, as a product of factors below `2^63`, so
/// that no constant is encoded as a negative immediate.
#[cfg(not(feature = "rt"))]
#[inline(always)]
fn native_pow2(k: u32) -> NativeInt {
    let a = k / 3;
    let b = (k - a) / 2;
    let c = k - a - b;
    NativeInt::from(1u64 << a) * NativeInt::from(1u64 << b) * NativeInt::from(1u64 << c)
}

/// `floor(x * y / z)` modulo `2^128`, and whether it is `2^128` or more.
#[cfg(not(feature = "rt"))]
#[inline(always)]
fn native_muldiv(x: NativeInt, y: NativeInt, z: NativeInt) -> (u128, bool) {
    let q = x.muldiv(y, z);
    (q.mask(128).into_u128(), !q.is_u128())
}

/// A [`FixedU128<Frac>`] whose fixed-point multiplication and division the
/// Prover computes exactly on native integers.
///
/// See the [module documentation](self).
#[repr(transparent)]
pub struct ExactFixedU128<Frac>(FixedU128<Frac>);

impl<Frac: LeEqU128> ExactFixedU128<Frac> {
    /// The number of integer bits.
    pub const INT_NBITS: u32 = FixedU128::<Frac>::INT_NBITS;
    /// The number of fractional bits.
    pub const FRAC_NBITS: u32 = FixedU128::<Frac>::FRAC_NBITS;
    /// Zero.
    pub const ZERO: Self = Self(FixedU128::ZERO);
    /// The smallest positive value, `2^-Frac`.
    pub const DELTA: Self = Self(FixedU128::DELTA);
    /// The smallest value, zero.
    pub const MIN: Self = Self(FixedU128::MIN);
    /// The largest value.
    pub const MAX: Self = Self(FixedU128::MAX);

    /// The number with encoding `bits`, i.e. `bits / 2^Frac`.
    pub const fn from_bits(bits: u128) -> Self {
        Self(FixedU128::from_bits(bits))
    }

    /// The encoding of `self`, i.e. `self * 2^Frac`.
    pub const fn to_bits(self) -> u128 {
        self.0.to_bits()
    }

    /// The equal value of the wide type.
    pub const fn to_wide(self) -> FixedU128<Frac> {
        self.0
    }

    /// The equal value of this type.
    pub const fn from_wide(wide: FixedU128<Frac>) -> Self {
        Self(wide)
    }

    /// Parses `src` at compile time, as the wide type's `lit` does.
    pub const fn lit(src: &str) -> Self {
        Self(FixedU128::lit(src))
    }

    /// The integer `src`, as the wide type's `const_from_int` converts it.
    /// Panics, at compile time in a constant, if it does not fit.
    pub const fn const_from_int(src: u128) -> Self {
        Self(FixedU128::const_from_int(src))
    }

    /// Converts as the wide type's `from_num` does.
    pub fn from_num<Src: ToFixed>(src: Src) -> Self {
        Self(FixedU128::from_num(src))
    }

    /// Converts as the wide type's `checked_from_num` does.
    pub fn checked_from_num<Src: ToFixed>(src: Src) -> Option<Self> {
        FixedU128::checked_from_num(src).map(Self)
    }

    /// Converts as the wide type's `saturating_from_num` does.
    pub fn saturating_from_num<Src: ToFixed>(src: Src) -> Self {
        Self(FixedU128::saturating_from_num(src))
    }

    /// Converts as the wide type's `to_num` does.
    pub fn to_num<Dst: FromFixed>(self) -> Dst {
        self.0.to_num()
    }

    /// Converts as the wide type's `checked_to_num` does.
    pub fn checked_to_num<Dst: FromFixed>(self) -> Option<Dst> {
        self.0.checked_to_num()
    }

    /// Converts as the wide type's `saturating_to_num` does.
    pub fn saturating_to_num<Dst: FromFixed>(self) -> Dst {
        self.0.saturating_to_num()
    }

    /// Converts as the wide type's `wrapping_to_num` does.
    pub fn wrapping_to_num<Dst: FromFixed>(self) -> Dst {
        self.0.wrapping_to_num()
    }

    /// Whether `self` is zero.
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// The integer part.
    pub const fn int(self) -> Self {
        Self(self.0.int())
    }

    /// The fractional part.
    pub const fn frac(self) -> Self {
        Self(self.0.frac())
    }

    /// Rounds down to an integer.
    pub const fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Rounds toward zero to an integer.
    pub const fn round_to_zero(self) -> Self {
        Self(self.0.round_to_zero())
    }

    /// Rounds up to an integer, `None` on overflow.
    pub const fn checked_ceil(self) -> Option<Self> {
        match self.0.checked_ceil() {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Rounds up to an integer, as the wide type's `ceil` does.
    pub const fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Rounds to the nearest integer, ties away from zero; `None` on overflow.
    pub const fn checked_round(self) -> Option<Self> {
        match self.0.checked_round() {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Rounds to the nearest integer, as the wide type's `round` does.
    pub const fn round(self) -> Self {
        Self(self.0.round())
    }

    /// `|self - rhs|`.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self(self.0.abs_diff(rhs.0))
    }

    /// `|self - rhs|`.
    pub const fn dist(self, rhs: Self) -> Self {
        Self(self.0.dist(rhs.0))
    }

    /// `self + rhs`, `None` on overflow.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// `self - rhs`, `None` when `rhs > self`.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// `self * rhs`, `None` on overflow.
    pub const fn checked_mul_int(self, rhs: u128) -> Option<Self> {
        match self.0.checked_mul_int(rhs) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// `self / rhs` rounded down, `None` when `rhs` is zero.
    pub const fn checked_div_int(self, rhs: u128) -> Option<Self> {
        match self.0.checked_div_int(rhs) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// `self + rhs`, saturating at [`MAX`](Self::MAX).
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// `self - rhs`, saturating at zero.
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// `self * rhs`, saturating at [`MAX`](Self::MAX).
    pub const fn saturating_mul_int(self, rhs: u128) -> Self {
        Self(self.0.saturating_mul_int(rhs))
    }

    /// `self + rhs`, wrapping.
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    /// `self - rhs`, wrapping.
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// `self * rhs`, wrapping.
    pub const fn wrapping_mul_int(self, rhs: u128) -> Self {
        Self(self.0.wrapping_mul_int(rhs))
    }

    /// `self / rhs` rounded down. Panics when `rhs` is zero.
    pub const fn wrapping_div_int(self, rhs: u128) -> Self {
        Self(self.0.wrapping_div_int(rhs))
    }

    /// `self + rhs`, panicking on overflow.
    pub const fn unwrapped_add(self, rhs: Self) -> Self {
        Self(self.0.unwrapped_add(rhs.0))
    }

    /// `self - rhs`, panicking when `rhs > self`.
    pub const fn unwrapped_sub(self, rhs: Self) -> Self {
        Self(self.0.unwrapped_sub(rhs.0))
    }

    /// `self * rhs`, panicking on overflow.
    pub const fn unwrapped_mul_int(self, rhs: u128) -> Self {
        Self(self.0.unwrapped_mul_int(rhs))
    }

    /// `self / rhs` rounded down. Panics when `rhs` is zero.
    pub const fn unwrapped_div_int(self, rhs: u128) -> Self {
        Self(self.0.unwrapped_div_int(rhs))
    }

    /// `self * rhs` rounded down, wrapping, and whether it overflowed.
    #[cfg(not(feature = "rt"))]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (bits, overflow) = native_muldiv(
            NativeInt::from(self.to_bits()),
            NativeInt::from(rhs.to_bits()),
            native_pow2(Frac::U32),
        );
        (Self::from_bits(bits), overflow)
    }

    /// `self * rhs` rounded down, wrapping, and whether it overflowed.
    #[cfg(feature = "rt")]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (v, overflow) = self.0.overflowing_mul(rhs.0);
        (Self(v), overflow)
    }

    /// `self / rhs` rounded down, wrapping, and whether it overflowed. Panics
    /// when `rhs` is zero.
    #[cfg(not(feature = "rt"))]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            panic!("division by zero");
        }
        let (bits, overflow) = native_muldiv(
            NativeInt::from(self.to_bits()),
            native_pow2(Frac::U32),
            NativeInt::from(rhs.to_bits()),
        );
        (Self::from_bits(bits), overflow)
    }

    /// `self / rhs` rounded down, wrapping, and whether it overflowed. Panics
    /// when `rhs` is zero.
    #[cfg(feature = "rt")]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        let (v, overflow) = self.0.overflowing_div(rhs.0);
        (Self(v), overflow)
    }

    /// `self * rhs` rounded down, `None` on overflow.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (v, false) => Some(v),
            (_, true) => None,
        }
    }

    /// `self / rhs` rounded down, `None` when `rhs` is zero or on overflow.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            return None;
        }
        match self.overflowing_div(rhs) {
            (v, false) => Some(v),
            (_, true) => None,
        }
    }

    /// `self * rhs` rounded down, saturating at [`MAX`](Self::MAX).
    pub fn saturating_mul(self, rhs: Self) -> Self {
        match self.overflowing_mul(rhs) {
            (v, false) => v,
            (_, true) => Self::MAX,
        }
    }

    /// `self / rhs` rounded down, saturating at [`MAX`](Self::MAX). Panics
    /// when `rhs` is zero.
    pub fn saturating_div(self, rhs: Self) -> Self {
        match self.overflowing_div(rhs) {
            (v, false) => v,
            (_, true) => Self::MAX,
        }
    }

    /// `self * rhs` rounded down, wrapping.
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// `self / rhs` rounded down, wrapping. Panics when `rhs` is zero.
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    /// `self * rhs` rounded down, panicking on overflow.
    pub fn unwrapped_mul(self, rhs: Self) -> Self {
        match self.overflowing_mul(rhs) {
            (v, false) => v,
            (_, true) => panic!("overflow"),
        }
    }

    /// `self / rhs` rounded down, panicking when `rhs` is zero or on overflow.
    pub fn unwrapped_div(self, rhs: Self) -> Self {
        match self.overflowing_div(rhs) {
            (v, false) => v,
            (_, true) => panic!("overflow"),
        }
    }

    /// The `*` and `/` operators: the wrapped result, after `fixed`'s own
    /// `debug_assert!` has had its say on an overflow.
    fn operator_result((v, overflow): (Self, bool)) -> Self {
        if overflow {
            Self::fixed_overflow_check();
        }
        v
    }

    /// Evaluates the wide type's `*` on two constants whose product overflows
    /// it (or, with no integer bits, its `/` on two whose quotient does), so
    /// that `fixed`'s `debug_assert!`, built with `fixed`'s profile, panics
    /// exactly when the wide type's operator would have.
    fn fixed_overflow_check() {
        if Frac::U32 < 128 {
            let _ = FixedU128::<Frac>::MAX * FixedU128::<Frac>::MAX;
        } else {
            let _ = FixedU128::<Frac>::MAX / FixedU128::<Frac>::DELTA;
        }
    }
}

impl<Frac: LeEqU128> ExactFixedU128<Frac>
where
    Frac: IsLessOrEqual<U127, Output = True>,
{
    /// One.
    pub const ONE: Self = Self(FixedU128::ONE);
}

impl<Frac> Clone for ExactFixedU128<Frac> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Frac> Copy for ExactFixedU128<Frac> {}

impl<Frac: LeEqU128> Default for ExactFixedU128<Frac> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<Frac> PartialEq for ExactFixedU128<Frac> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.to_bits() == rhs.0.to_bits()
    }
}

impl<Frac> Eq for ExactFixedU128<Frac> {}

impl<Frac> PartialOrd for ExactFixedU128<Frac> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<Frac> Ord for ExactFixedU128<Frac> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.to_bits().cmp(&rhs.0.to_bits())
    }
}

impl<Frac> Hash for ExactFixedU128<Frac> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl<Frac: LeEqU128> fmt::Debug for ExactFixedU128<Frac> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<Frac: LeEqU128> fmt::Display for ExactFixedU128<Frac> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

macro_rules! from_int {
    ($($Int:ty),*) => { $(
        impl<Frac: LeEqU128> From<$Int> for ExactFixedU128<Frac>
        where
            FixedU128<Frac>: From<$Int>,
        {
            #[inline(always)]
            fn from(src: $Int) -> Self {
                Self(FixedU128::from(src))
            }
        }
    )* };
}

from_int!(bool, u8, u16, u32, u64);

macro_rules! cmp_int {
    ($($Int:ty),*) => { $(
        impl<Frac: LeEqU128> PartialEq<$Int> for ExactFixedU128<Frac> {
            #[inline]
            fn eq(&self, rhs: &$Int) -> bool {
                self.0 == *rhs
            }
        }

        impl<Frac: LeEqU128> PartialEq<ExactFixedU128<Frac>> for $Int {
            #[inline]
            fn eq(&self, rhs: &ExactFixedU128<Frac>) -> bool {
                *self == rhs.0
            }
        }

        impl<Frac: LeEqU128> PartialOrd<$Int> for ExactFixedU128<Frac> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Int) -> Option<Ordering> {
                self.0.partial_cmp(rhs)
            }
        }

        impl<Frac: LeEqU128> PartialOrd<ExactFixedU128<Frac>> for $Int {
            #[inline]
            fn partial_cmp(&self, rhs: &ExactFixedU128<Frac>) -> Option<Ordering> {
                self.partial_cmp(&rhs.0)
            }
        }
    )* };
}

cmp_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<Frac: LeEqU128> FromStr for ExactFixedU128<Frac> {
    type Err = ParseFixedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FixedU128::<Frac>::from_str(s).map(Self)
    }
}

impl<Frac: LeEqU128> cvlr_nondet::Nondet for ExactFixedU128<Frac> {
    fn nondet() -> Self {
        Self::from_bits(cvlr_nondet::nondet())
    }
}

impl<Frac: LeEqU128> cvlr_log::CvlrLog for ExactFixedU128<Frac> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        logger.log_u128(tag, self.to_bits());
    }
}

impl<Frac: LeEqU128> ToFixed for ExactFixedU128<Frac> {
    fn to_fixed<F: Fixed>(self) -> F {
        self.0.to_fixed()
    }

    fn checked_to_fixed<F: Fixed>(self) -> Option<F> {
        self.0.checked_to_fixed()
    }

    fn saturating_to_fixed<F: Fixed>(self) -> F {
        self.0.saturating_to_fixed()
    }

    fn wrapping_to_fixed<F: Fixed>(self) -> F {
        self.0.wrapping_to_fixed()
    }

    fn overflowing_to_fixed<F: Fixed>(self) -> (F, bool) {
        self.0.overflowing_to_fixed()
    }
}

impl<Frac: LeEqU128> FromFixed for ExactFixedU128<Frac> {
    fn from_fixed<F: Fixed>(src: F) -> Self {
        Self(FixedU128::from_fixed(src))
    }

    fn checked_from_fixed<F: Fixed>(src: F) -> Option<Self> {
        FixedU128::checked_from_fixed(src).map(Self)
    }

    fn saturating_from_fixed<F: Fixed>(src: F) -> Self {
        Self(FixedU128::saturating_from_fixed(src))
    }

    fn wrapping_from_fixed<F: Fixed>(src: F) -> Self {
        Self(FixedU128::wrapping_from_fixed(src))
    }

    fn overflowing_from_fixed<F: Fixed>(src: F) -> (Self, bool) {
        let (v, overflow) = FixedU128::overflowing_from_fixed(src);
        (Self(v), overflow)
    }
}

macro_rules! binary_op {
    ($Imp:ident, $method:ident, $Assign:ident, $assign:ident, |$a:ident, $b:ident| $body:expr) => {
        impl<Frac: LeEqU128> $Imp for ExactFixedU128<Frac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: Self) -> Self {
                let ($a, $b) = (self, rhs);
                $body
            }
        }

        impl<Frac: LeEqU128> $Imp<&Self> for ExactFixedU128<Frac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: &Self) -> Self {
                $Imp::$method(self, *rhs)
            }
        }

        impl<Frac: LeEqU128> $Imp<ExactFixedU128<Frac>> for &ExactFixedU128<Frac> {
            type Output = ExactFixedU128<Frac>;
            #[inline]
            fn $method(self, rhs: ExactFixedU128<Frac>) -> Self::Output {
                $Imp::$method(*self, rhs)
            }
        }

        impl<Frac: LeEqU128> $Imp<&ExactFixedU128<Frac>> for &ExactFixedU128<Frac> {
            type Output = ExactFixedU128<Frac>;
            #[inline]
            fn $method(self, rhs: &ExactFixedU128<Frac>) -> Self::Output {
                $Imp::$method(*self, *rhs)
            }
        }

        impl<Frac: LeEqU128> $Assign for ExactFixedU128<Frac> {
            #[inline]
            fn $assign(&mut self, rhs: Self) {
                *self = $Imp::$method(*self, rhs)
            }
        }

        impl<Frac: LeEqU128> $Assign<&Self> for ExactFixedU128<Frac> {
            #[inline]
            fn $assign(&mut self, rhs: &Self) {
                *self = $Imp::$method(*self, *rhs)
            }
        }
    };
}

binary_op!(Add, add, AddAssign, add_assign, |a, b| ExactFixedU128(a.0 + b.0));
binary_op!(Sub, sub, SubAssign, sub_assign, |a, b| ExactFixedU128(a.0 - b.0));
binary_op!(Mul, mul, MulAssign, mul_assign, |a, b| Self::operator_result(a.overflowing_mul(b)));
binary_op!(Div, div, DivAssign, div_assign, |a, b| Self::operator_result(a.overflowing_div(b)));

macro_rules! int_op {
    ($Imp:ident, $method:ident, $Assign:ident, $assign:ident) => {
        impl<Frac: LeEqU128> $Imp<u128> for ExactFixedU128<Frac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: u128) -> Self {
                Self($Imp::$method(self.0, rhs))
            }
        }

        impl<Frac: LeEqU128> $Imp<&u128> for ExactFixedU128<Frac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: &u128) -> Self {
                Self($Imp::$method(self.0, *rhs))
            }
        }

        impl<Frac: LeEqU128> $Imp<u128> for &ExactFixedU128<Frac> {
            type Output = ExactFixedU128<Frac>;
            #[inline]
            fn $method(self, rhs: u128) -> Self::Output {
                ExactFixedU128($Imp::$method(self.0, rhs))
            }
        }

        impl<Frac: LeEqU128> $Assign<u128> for ExactFixedU128<Frac> {
            #[inline]
            fn $assign(&mut self, rhs: u128) {
                *self = ExactFixedU128($Imp::$method(self.0, rhs))
            }
        }
    };
}

int_op!(Mul, mul, MulAssign, mul_assign);
int_op!(Div, div, DivAssign, div_assign);

impl<Frac: LeEqU128> Mul<ExactFixedU128<Frac>> for u128 {
    type Output = ExactFixedU128<Frac>;
    #[inline]
    fn mul(self, rhs: ExactFixedU128<Frac>) -> Self::Output {
        ExactFixedU128(self * rhs.0)
    }
}

/// The literal `$v` (an integer or decimal literal, as `fixed_macro` accepts)
/// as a value of the [`ExactFixedU128`] type `$T`, evaluated at compile time.
///
/// This is the stand-in for `fixed_macro`'s literal macros; like them, a
/// literal the type cannot hold fails the build.
#[macro_export]
macro_rules! exact_lit {
    ($T:ty, $v:literal) => {{
        const VALUE: $T = <$T>::lit(::core::stringify!($v));
        VALUE
    }};
}
