//! A verification-only stand-in for a 128-bit [`fixed`] type that computes at
//! 64 bits.
//!
//! Programs that keep fractions as [`FixedU128`] (`U68F60`, for example) do
//! their multiplications and divisions through 256-bit intermediates, and the
//! Prover pays for every one of those long divisions. [`NarrowFixedU128`] has
//! the API of [`FixedU128<Frac>`] but holds its value as a [`FixedU64<NFrac>`],
//! so the same operations run on 64-bit values with 128-bit intermediates.
//! Swap it in for the program's type under the verification feature, for
//! example by replacing
//!
//! ```ignore
//! pub use fixed::types::U68F60 as Fraction;
//! ```
//!
//! with
//!
//! ```ignore
//! pub type Fraction = cvlr_fixed::narrow::NarrowFixedU128<U60, U14>;
//! ```
//!
//! A literal macro such as `fixed_macro`'s `U68F60!` is replaced with one that
//! expands to [`narrow_lit!`](crate::narrow_lit), which evaluates the literal
//! at compile time as `fixed_macro` does:
//!
//! ```ignore
//! macro_rules! fraction {
//!     ($v:literal) => { cvlr_fixed::narrow_lit!($crate::Fraction, $v) };
//! }
//! ```
//!
//! # What it changes
//!
//! A value is a multiple of `2^-NFrac` below `2^(64 - NFrac)`: the grid of
//! [`FixedU64<NFrac>`]. Every operation returns the wide type's result rounded
//! down to that grid: for every operation except
//! [`round`](NarrowFixedU128::round) and [`ceil`](NarrowFixedU128::ceil), which
//! round to an integer, the result is exactly the wide result with its low
//! `Frac - NFrac` fractional bits cleared. Parsing ([`lit`](NarrowFixedU128::lit)
//! and `FromStr`) parses as the wide type does and then clears those bits. Each operation therefore loses up to `2^-NFrac` of
//! precision, and the errors accumulate across a computation. A rule over this
//! type can say nothing about rounding or precision, and every verdict that
//! depends on it holds under this modeling choice only.
//!
//! # What it keeps
//!
//! * **The stored encoding.** [`to_bits`](NarrowFixedU128::to_bits) and
//!   [`from_bits`](NarrowFixedU128::from_bits) use the wide type's `u128`
//!   encoding with `Frac` fractional bits, and
//!   [`FRAC_NBITS`](NarrowFixedU128::FRAC_NBITS) is `Frac`. Values the program
//!   stores as raw bits, and constants a harness writes in that encoding, keep
//!   their meaning, and code that does its own arithmetic on `to_bits()` still
//!   computes in the scale it was written for.
//! * **The wide type's overflow behavior.** An operation whose wide result would
//!   overflow fails the way the wide operation fails: `checked_*` returns
//!   `None`, `wrapping_*` wraps modulo `2^128` of the wide encoding, and the
//!   operators panic or wrap as `fixed`'s do (see below).
//!
//! # What it assumes
//!
//! A result that fits the wide type but not the narrow one has no faithful
//! narrow value. Operations add a Prover assumption (`cvlr_assume!`) that this
//! does not happen, so executions where it would are dropped from the analysis.
//! In effect every value a rule reaches is below `2^(64 - NFrac)`; choose
//! `NFrac` so that the amounts the property is about fit, and confirm with
//! `cvlr_satisfy!` that the paths of interest stay reachable. On the host (the
//! `rt` feature) such an assumption panics.
//!
//! [`from_bits`](NarrowFixedU128::from_bits) is not a `const fn`, unlike the
//! wide type's: it computes through the Prover's native integers, which model
//! it exactly. A constant the program builds with it uses
//! [`const_from_bits`](NarrowFixedU128::const_from_bits) instead.
//!
//! `from_bits` rounds an encoding that is off the grid down onto it, and saturates one above the narrow range at
//! [`MAX`](NarrowFixedU128::MAX); it does not assume. Programs call it on
//! encodings they compute themselves, from their own arithmetic on `to_bits()`,
//! and those are rarely on the grid: an assumption there would drop real
//! executions without a trace. The consequence is that a stored raw value off
//! the grid reads, through the program's type, as a slightly smaller value
//! than its raw bits say. A rule should read stored values through
//! `from_bits` as the program does rather than compare raw encodings, or state
//! the value is representable with
//! [`is_representable_bits`](NarrowFixedU128::is_representable_bits) or
//! [`from_bits_exact`](NarrowFixedU128::from_bits_exact). A constant the
//! program builds from bits below the grid (an epsilon of a few wide ulps, say)
//! becomes zero.
//!
//! [`MAX`](NarrowFixedU128::MAX) and [`DELTA`](NarrowFixedU128::DELTA) are the
//! narrow type's, not the wide type's: the wide maximum has no narrow
//! representation, and its `to_bits()` is not `u128::MAX`. Saturating
//! operations saturate at the narrow maximum.
//!
//! # Operators
//!
//! `fixed` implements `+`, `-`, `+=`, `-=`, and `*` and `/` by an integer with
//! the primitive integer operators, and `*` and `/` between two fixed-point
//! values with a wrapping operation. With `overflow-checks` on, as in most
//! Solana release profiles, the first group panics on overflow and the second
//! wraps. This type does the same in every profile: with `overflow-checks`
//! off, its `+`, `-` and integer `*` still panic where the wide type's wrap,
//! and in a debug build its fixed-point `*` and `/` wrap where the wide
//! type's `debug_assert!` panics.
//!
//! Only the operators above are provided. A program that uses `%`, shifts,
//! bitwise operators, `sqrt`, `recip`, `mul_add` or comparisons with floats on
//! its fixed-point type does not compile against this one, and needs those
//! uses rewritten under the verification feature.
//!
//! # Layout
//!
//! The value is a `u64`: 8 bytes, aligned to 8, where the wide type is 16
//! bytes aligned to 16. A struct that holds the type itself, rather than its
//! `to_bits()`, has a different layout under the verification feature, so it
//! must not be one the program reads from or writes to account data directly.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;

use cvlr_asserts::cvlr_assume;
use fixed::traits::{Fixed, FromFixed, ToFixed};
use fixed::types::extra::{LeEqU128, LeEqU64};
use fixed::{FixedU128, FixedU64, ParseFixedError};

/// `x / 2^k`, rounded down, for `k <= 64`, where the caller has established
/// that the quotient fits in a `u64`.
///
/// On SBF a `u128` shift by a constant compiles to two 64-bit halves joined by
/// a bitwise or, and the Prover models an or of two non-constant values only
/// by bounds. Its native integers model the same division exactly, and
/// returning a `u64` spares the split back into two halves.
#[cfg(not(feature = "rt"))]
#[inline(always)]
fn shr_to_u64(x: u128, k: u32) -> u64 {
    use cvlr_mathint::NativeInt;
    u64::from(NativeInt::from(x) / NativeInt::from(1u128 << k))
}

/// `x / 2^k`, rounded down, for `k <= 64`, where the caller has established
/// that the quotient fits in a `u64`. On the host, where native integers are
/// 64 bits wide, a plain shift.
#[cfg(feature = "rt")]
#[inline(always)]
fn shr_to_u64(x: u128, k: u32) -> u64 {
    (x >> k) as u64
}

/// An exact result in narrow units, `x / 2^k` rounded down, kept unshifted so
/// that range checks compare `x` with constants and only an in-range quotient
/// is computed.
#[derive(Clone, Copy)]
struct Scaled {
    x: u128,
    k: u32,
}

impl Scaled {
    const fn exact(x: u128) -> Self {
        Self { x, k: 0 }
    }

    /// Whether the quotient is at most `limit`.
    fn at_most(self, limit: u128) -> bool {
        limit >= u128::MAX >> self.k || self.x < (limit + 1) << self.k
    }

    /// The quotient, which the caller has established is at most `u64::MAX`.
    fn quotient(self) -> u64 {
        if self.k == 0 {
            self.x as u64
        } else {
            shr_to_u64(self.x, self.k)
        }
    }

    /// The quotient modulo `2^m`, for `m <= 128`.
    fn wrapped(self, m: u32) -> Self {
        let bits = m + self.k;
        let mask = if bits >= 128 {
            u128::MAX
        } else {
            (1 << bits) - 1
        };
        Self {
            x: self.x & mask,
            k: self.k,
        }
    }
}

/// A [`FixedU128<Frac>`] stand-in that computes as a [`FixedU64<NFrac>`].
///
/// See the [module documentation](self) for what it preserves, what it
/// changes and what it assumes. `NFrac` must be below 64 and must not exceed
/// `Frac`, and the difference must be at most 64.
pub struct NarrowFixedU128<Frac, NFrac> {
    narrow: u64,
    _frac: PhantomData<(Frac, NFrac)>,
}

enum Outcome {
    Fits(u64),
    WideOverflow,
}

impl<Frac: LeEqU128, NFrac: LeEqU64> NarrowFixedU128<Frac, NFrac> {
    const VALID: () = assert!(
        NFrac::U32 < 64 && NFrac::U32 <= Frac::U32 && Frac::U32 - NFrac::U32 <= 64,
        "NarrowFixedU128 needs NFrac < 64 and NFrac <= Frac <= NFrac + 64"
    );

    /// The number of fractional bits the wide type drops.
    const SHIFT: u32 = Frac::U32 - NFrac::U32;

    /// The fractional bits of the narrow representation.
    const FRAC_MASK: u64 = if NFrac::U32 == 0 {
        0
    } else {
        u64::MAX >> (64 - NFrac::U32)
    };

    /// The wide type's largest value, in narrow units (rounded down).
    const WIDE_MAX_NARROW: u128 = u128::MAX >> Self::SHIFT;

    /// The narrow type's largest value, in narrow units.
    const NARROW_MAX: u128 = u64::MAX as u128;

    /// The number of integer bits of the wide type.
    pub const INT_NBITS: u32 = 128 - Frac::U32;
    /// The number of fractional bits of the wide type, which is also the
    /// scale of [`to_bits`](Self::to_bits) and [`from_bits`](Self::from_bits).
    pub const FRAC_NBITS: u32 = Frac::U32;

    /// Zero.
    pub const ZERO: Self = Self::from_narrow(0);
    /// One.
    pub const ONE: Self = Self::from_narrow(1 << NFrac::U32);
    /// The smallest positive value, `2^-NFrac`: the narrow type's, not the
    /// wide type's.
    pub const DELTA: Self = Self::from_narrow(1);
    /// The smallest value, zero.
    pub const MIN: Self = Self::ZERO;
    /// The largest value, `2^(64 - NFrac) - 2^-NFrac`: the narrow type's, not
    /// the wide type's.
    pub const MAX: Self = Self::from_narrow(u64::MAX);

    const fn from_narrow(narrow: u64) -> Self {
        let () = Self::VALID;
        Self {
            narrow,
            _frac: PhantomData,
        }
    }

    /// The number with wide encoding `bits`, i.e. `bits / 2^Frac`, rounded
    /// down onto the narrow grid and saturated at [`MAX`](Self::MAX).
    ///
    /// Use [`from_bits_exact`](Self::from_bits_exact) for a value that must not
    /// change. Unlike the wide type's, this is not a `const fn`, so that the
    /// Prover can model it exactly; in a constant, use
    /// [`const_from_bits`](Self::const_from_bits).
    pub fn from_bits(bits: u128) -> Self {
        Self::saturating(Some(Self::wide_bits(bits)))
    }

    /// [`from_bits`](Self::from_bits) for constants.
    ///
    /// The same value, but computed with a `u128` shift, which the Prover
    /// models only approximately when it is evaluated at run time: use it in
    /// constants only.
    pub const fn const_from_bits(bits: u128) -> Self {
        let () = Self::VALID;
        let narrow = bits >> Self::SHIFT;
        if narrow > u64::MAX as u128 {
            Self::MAX
        } else {
            Self::from_narrow(narrow as u64)
        }
    }

    /// The wide encoding of `self`, i.e. `self * 2^Frac`.
    pub const fn to_bits(self) -> u128 {
        (self.narrow as u128) << Self::SHIFT
    }

    /// Whether the wide encoding `bits` is a value of this type: on the narrow
    /// grid and within its range, so that `from_bits(bits).to_bits() == bits`.
    pub const fn is_representable_bits(bits: u128) -> bool {
        let () = Self::VALID;
        let low_mask = if Self::SHIFT == 0 {
            0
        } else {
            u128::MAX >> (128 - Self::SHIFT)
        };
        bits & low_mask == 0 && bits <= Self::MAX.to_bits()
    }

    /// The number with wide encoding `bits`, assuming it is
    /// [representable](Self::is_representable_bits).
    pub fn from_bits_exact(bits: u128) -> Self {
        cvlr_assume!(Self::is_representable_bits(bits));
        Self::from_bits(bits)
    }

    /// The equal value of the wide type.
    pub const fn to_wide(self) -> FixedU128<Frac> {
        FixedU128::from_bits(self.to_bits())
    }

    /// The wide value `wide`, rounded down onto the narrow grid, assuming it
    /// is within the narrow range.
    pub fn from_wide(wide: FixedU128<Frac>) -> Self {
        Self::assume_fits(Self::wide_bits(wide.to_bits()))
    }

    /// The equal value of the narrow type.
    pub const fn to_narrow(self) -> FixedU64<NFrac> {
        FixedU64::from_bits(self.narrow)
    }

    /// The narrow value `narrow`, which is always representable.
    pub const fn from_narrow_fixed(narrow: FixedU64<NFrac>) -> Self {
        Self::from_narrow(narrow.to_bits())
    }

    /// A wide encoding, in narrow units.
    const fn wide_bits(bits: u128) -> Scaled {
        Scaled {
            x: bits,
            k: Self::SHIFT,
        }
    }

    fn assume_fits(r: Scaled) -> Self {
        cvlr_assume!(r.at_most(Self::NARROW_MAX));
        Self::from_narrow(r.quotient())
    }

    /// Classifies an exact result `r`: a wide overflow, or a narrow value under
    /// the assumption that it fits. `None` is a result beyond `u128`, which is
    /// always a wide overflow.
    fn settle(r: Option<Scaled>) -> Outcome {
        match r {
            Some(r) if r.at_most(Self::WIDE_MAX_NARROW) => {
                cvlr_assume!(r.at_most(Self::NARROW_MAX));
                Outcome::Fits(r.quotient())
            }
            _ => Outcome::WideOverflow,
        }
    }

    fn checked(r: Option<Scaled>) -> Option<Self> {
        match Self::settle(r) {
            Outcome::Fits(n) => Some(Self::from_narrow(n)),
            Outcome::WideOverflow => None,
        }
    }

    fn saturating(r: Option<Scaled>) -> Self {
        match r {
            Some(r) if r.at_most(Self::NARROW_MAX) => Self::from_narrow(r.quotient()),
            _ => Self::MAX,
        }
    }

    /// The wide result wrapped modulo `2^128` of its encoding: in narrow units,
    /// modulo `2^(128 - Frac + NFrac)`.
    fn wrapping(r: Scaled) -> Self {
        Self::assume_fits(r.wrapped(128 - Self::SHIFT))
    }

    fn unwrapped(r: Option<Scaled>) -> Self {
        Self::checked(r).expect("overflow")
    }

    // Exact results in narrow units; `None` when they exceed `u128`.

    fn add_narrow(self, rhs: Self) -> Option<Scaled> {
        Some(Scaled::exact(self.narrow as u128 + rhs.narrow as u128))
    }

    fn sub_narrow(self, rhs: Self) -> Option<Scaled> {
        (self.narrow as u128)
            .checked_sub(rhs.narrow as u128)
            .map(Scaled::exact)
    }

    fn mul_narrow(self, rhs: Self) -> Scaled {
        Scaled {
            x: (self.narrow as u128) * (rhs.narrow as u128),
            k: NFrac::U32,
        }
    }

    fn div_narrow(self, rhs: Self) -> Option<Scaled> {
        if rhs.narrow == 0 {
            None
        } else {
            Some(Scaled::exact(
                ((self.narrow as u128) << NFrac::U32) / rhs.narrow as u128,
            ))
        }
    }

    fn mul_int_narrow(self, rhs: u128) -> Option<Scaled> {
        (self.narrow as u128).checked_mul(rhs).map(Scaled::exact)
    }

    /// The integer `src`. Panics, at compile time in a constant, if it does not
    /// fit the narrow range.
    pub const fn const_from_int(src: u128) -> Self {
        let () = Self::VALID;
        assert!(
            src <= (u64::MAX >> NFrac::U32) as u128,
            "const_from_int: out of range"
        );
        Self::from_narrow((src as u64) << NFrac::U32)
    }

    /// Parses a literal as the wide type does, then rounds it down onto the
    /// narrow grid. Panics, at compile time in a constant, on a malformed
    /// literal or one outside the narrow range.
    ///
    /// Outside a constant this parses at run time, through loops the Prover
    /// has to unroll; [`narrow_lit!`](crate::narrow_lit) always evaluates at
    /// compile time.
    pub const fn lit(src: &str) -> Self {
        let wide = FixedU128::<Frac>::lit(src);
        let narrow = wide.to_bits() >> Self::SHIFT;
        assert!(narrow <= u64::MAX as u128, "lit: out of range");
        Self::from_narrow(narrow as u64)
    }

    /// Converts as the wide type's `from_num` does, then rounds down onto the
    /// narrow grid, assuming the result fits.
    pub fn from_num<Src: ToFixed>(src: Src) -> Self {
        Self::from_wide(FixedU128::<Frac>::from_num(src))
    }

    /// Converts as the wide type's `checked_from_num` does: `None` when the
    /// value does not fit the wide type.
    pub fn checked_from_num<Src: ToFixed>(src: Src) -> Option<Self> {
        FixedU128::<Frac>::checked_from_num(src).map(Self::from_wide)
    }

    /// Converts, saturating at [`MAX`](Self::MAX).
    pub fn saturating_from_num<Src: ToFixed>(src: Src) -> Self {
        let wide = FixedU128::<Frac>::saturating_from_num(src);
        Self::saturating(Some(Self::wide_bits(wide.to_bits())))
    }

    // Conversions out of the type depend only on the value, which the narrow
    // representation holds exactly; going through it avoids the wide type's
    // 128-bit shifts.

    /// Converts as the wide type's `to_num` does.
    pub fn to_num<Dst: FromFixed>(self) -> Dst {
        self.to_narrow().to_num()
    }

    /// Converts as the wide type's `checked_to_num` does.
    pub fn checked_to_num<Dst: FromFixed>(self) -> Option<Dst> {
        self.to_narrow().checked_to_num()
    }

    /// Converts as the wide type's `saturating_to_num` does.
    pub fn saturating_to_num<Dst: FromFixed>(self) -> Dst {
        self.to_narrow().saturating_to_num()
    }

    /// Converts as the wide type's `wrapping_to_num` does.
    pub fn wrapping_to_num<Dst: FromFixed>(self) -> Dst {
        self.to_narrow().wrapping_to_num()
    }

    /// Whether `self` is zero.
    pub const fn is_zero(self) -> bool {
        self.narrow == 0
    }

    /// The integer part.
    pub const fn int(self) -> Self {
        Self::from_narrow(self.narrow & !Self::FRAC_MASK)
    }

    /// The fractional part.
    pub const fn frac(self) -> Self {
        Self::from_narrow(self.narrow - self.int().narrow)
    }

    /// Rounds down to an integer.
    pub const fn floor(self) -> Self {
        self.int()
    }

    /// Rounds toward zero to an integer.
    pub const fn round_to_zero(self) -> Self {
        self.int()
    }

    /// Rounds up to an integer, `None` on wide overflow.
    pub fn checked_ceil(self) -> Option<Self> {
        let int = self.int();
        if self.narrow == int.narrow {
            Some(self)
        } else {
            Self::checked(int.add_narrow(Self::ONE))
        }
    }

    /// Rounds up to an integer, panicking on wide overflow.
    pub fn ceil(self) -> Self {
        self.checked_ceil().expect("overflow")
    }

    /// Rounds to the nearest integer, ties away from zero; `None` on wide
    /// overflow.
    pub fn checked_round(self) -> Option<Self> {
        let half = Self::ONE.narrow >> 1;
        if NFrac::U32 == 0 || self.frac().narrow < half {
            Some(self.int())
        } else {
            Self::checked(self.int().add_narrow(Self::ONE))
        }
    }

    /// Rounds to the nearest integer, ties away from zero, panicking on wide
    /// overflow.
    pub fn round(self) -> Self {
        self.checked_round().expect("overflow")
    }

    /// `|self - rhs|`.
    pub const fn abs_diff(self, rhs: Self) -> Self {
        Self::from_narrow(self.narrow.abs_diff(rhs.narrow))
    }

    /// `|self - rhs|`.
    pub const fn dist(self, rhs: Self) -> Self {
        self.abs_diff(rhs)
    }

    /// `self + rhs`, `None` on wide overflow.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Self::checked(self.add_narrow(rhs))
    }

    /// `self - rhs`, `None` when `rhs > self`.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Self::checked(self.sub_narrow(rhs))
    }

    /// `self * rhs` rounded down, `None` on wide overflow.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        Self::checked(Some(self.mul_narrow(rhs)))
    }

    /// `self / rhs` rounded down, `None` when `rhs` is zero or on wide
    /// overflow.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        Self::checked(self.div_narrow(rhs))
    }

    /// `self * rhs`, `None` on wide overflow.
    pub fn checked_mul_int(self, rhs: u128) -> Option<Self> {
        Self::checked(self.mul_int_narrow(rhs))
    }

    /// `self / rhs` rounded down, `None` when `rhs` is zero.
    pub fn checked_div_int(self, rhs: u128) -> Option<Self> {
        (rhs != 0).then(|| Self::from_narrow((self.narrow as u128 / rhs) as u64))
    }

    /// `self + rhs`, saturating at [`MAX`](Self::MAX).
    pub fn saturating_add(self, rhs: Self) -> Self {
        Self::saturating(self.add_narrow(rhs))
    }

    /// `self - rhs`, saturating at zero.
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self::from_narrow(self.narrow.saturating_sub(rhs.narrow))
    }

    /// `self * rhs` rounded down, saturating at [`MAX`](Self::MAX).
    pub fn saturating_mul(self, rhs: Self) -> Self {
        Self::saturating(Some(self.mul_narrow(rhs)))
    }

    /// `self / rhs` rounded down, saturating at [`MAX`](Self::MAX). Panics
    /// when `rhs` is zero.
    pub fn saturating_div(self, rhs: Self) -> Self {
        Self::saturating(Some(self.div_narrow(rhs).expect("division by zero")))
    }

    /// `self * rhs`, saturating at [`MAX`](Self::MAX).
    pub fn saturating_mul_int(self, rhs: u128) -> Self {
        Self::saturating(self.mul_int_narrow(rhs))
    }

    /// `self + rhs`, wrapping as the wide type does.
    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self::wrapping(Scaled::exact(self.narrow as u128 + rhs.narrow as u128))
    }

    /// `self - rhs`, wrapping as the wide type does.
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping(Scaled::exact(
            (self.narrow as u128).wrapping_sub(rhs.narrow as u128),
        ))
    }

    /// `self * rhs` rounded down, wrapping as the wide type does.
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        Self::wrapping(self.mul_narrow(rhs))
    }

    /// `self / rhs` rounded down, wrapping as the wide type does. Panics when
    /// `rhs` is zero.
    pub fn wrapping_div(self, rhs: Self) -> Self {
        Self::wrapping(self.div_narrow(rhs).expect("division by zero"))
    }

    /// `self * rhs`, wrapping as the wide type does.
    pub fn wrapping_mul_int(self, rhs: u128) -> Self {
        Self::wrapping(Scaled::exact((self.narrow as u128).wrapping_mul(rhs)))
    }

    /// `self / rhs` rounded down. Panics when `rhs` is zero.
    pub fn wrapping_div_int(self, rhs: u128) -> Self {
        self.unwrapped_div_int(rhs)
    }

    /// `self + rhs`, panicking on wide overflow.
    pub fn unwrapped_add(self, rhs: Self) -> Self {
        Self::unwrapped(self.add_narrow(rhs))
    }

    /// `self - rhs`, panicking when `rhs > self`.
    pub fn unwrapped_sub(self, rhs: Self) -> Self {
        Self::unwrapped(self.sub_narrow(rhs))
    }

    /// `self * rhs` rounded down, panicking on wide overflow.
    pub fn unwrapped_mul(self, rhs: Self) -> Self {
        Self::unwrapped(Some(self.mul_narrow(rhs)))
    }

    /// `self / rhs` rounded down, panicking when `rhs` is zero or on wide
    /// overflow.
    pub fn unwrapped_div(self, rhs: Self) -> Self {
        Self::unwrapped(Some(self.div_narrow(rhs).expect("division by zero")))
    }

    /// `self * rhs`, panicking on wide overflow.
    pub fn unwrapped_mul_int(self, rhs: u128) -> Self {
        Self::unwrapped(self.mul_int_narrow(rhs))
    }

    /// `self / rhs` rounded down. Panics when `rhs` is zero.
    pub const fn unwrapped_div_int(self, rhs: u128) -> Self {
        Self::from_narrow((self.narrow as u128 / rhs) as u64)
    }
}

impl<Frac, NFrac> Clone for NarrowFixedU128<Frac, NFrac> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Frac, NFrac> Copy for NarrowFixedU128<Frac, NFrac> {}

impl<Frac: LeEqU128, NFrac: LeEqU64> Default for NarrowFixedU128<Frac, NFrac> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<Frac, NFrac> PartialEq for NarrowFixedU128<Frac, NFrac> {
    fn eq(&self, rhs: &Self) -> bool {
        self.narrow == rhs.narrow
    }
}

impl<Frac, NFrac> Eq for NarrowFixedU128<Frac, NFrac> {}

impl<Frac, NFrac> PartialOrd for NarrowFixedU128<Frac, NFrac> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<Frac, NFrac> Ord for NarrowFixedU128<Frac, NFrac> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.narrow.cmp(&rhs.narrow)
    }
}

impl<Frac, NFrac> Hash for NarrowFixedU128<Frac, NFrac> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.narrow.hash(state)
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64> fmt::Debug for NarrowFixedU128<Frac, NFrac> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_wide(), f)
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64> fmt::Display for NarrowFixedU128<Frac, NFrac> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_wide(), f)
    }
}

// `fixed` converts these losslessly into every wide type with enough integer
// bits; the narrow value assumes the integer fits its range.
macro_rules! from_int {
    ($($Int:ty),*) => { $(
        impl<Frac: LeEqU128, NFrac: LeEqU64> From<$Int> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn from(src: $Int) -> Self {
                Self::from_num(src)
            }
        }
    )* };
}

from_int!(bool, u8, u16, u32, u64);

// Exact comparisons with primitive integers, in both directions, as `fixed`
// provides them, made through the equal narrow value.
macro_rules! cmp_int {
    ($($Int:ty),*) => { $(
        impl<Frac: LeEqU128, NFrac: LeEqU64> PartialEq<$Int> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn eq(&self, rhs: &$Int) -> bool {
                self.to_narrow() == *rhs
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> PartialEq<NarrowFixedU128<Frac, NFrac>> for $Int {
            #[inline]
            fn eq(&self, rhs: &NarrowFixedU128<Frac, NFrac>) -> bool {
                *self == rhs.to_narrow()
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> PartialOrd<$Int> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Int) -> Option<Ordering> {
                self.to_narrow().partial_cmp(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> PartialOrd<NarrowFixedU128<Frac, NFrac>> for $Int {
            #[inline]
            fn partial_cmp(&self, rhs: &NarrowFixedU128<Frac, NFrac>) -> Option<Ordering> {
                self.partial_cmp(&rhs.to_narrow())
            }
        }
    )* };
}

cmp_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// Parses as the wide type does, then rounds down onto the narrow grid,
/// assuming the result fits.
impl<Frac: LeEqU128, NFrac: LeEqU64> FromStr for NarrowFixedU128<Frac, NFrac> {
    type Err = ParseFixedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FixedU128::<Frac>::from_str(s).map(Self::from_wide)
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64> cvlr_nondet::Nondet for NarrowFixedU128<Frac, NFrac> {
    fn nondet() -> Self {
        Self::from_narrow(cvlr_nondet::nondet())
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64> cvlr_log::CvlrLog for NarrowFixedU128<Frac, NFrac> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        logger.log_u64_as_fp(tag, self.narrow, NFrac::U64);
    }
}

/// Converts through the equal narrow value.
impl<Frac: LeEqU128, NFrac: LeEqU64> ToFixed for NarrowFixedU128<Frac, NFrac> {
    fn to_fixed<F: Fixed>(self) -> F {
        self.to_narrow().to_fixed()
    }

    fn checked_to_fixed<F: Fixed>(self) -> Option<F> {
        self.to_narrow().checked_to_fixed()
    }

    fn saturating_to_fixed<F: Fixed>(self) -> F {
        self.to_narrow().saturating_to_fixed()
    }

    fn wrapping_to_fixed<F: Fixed>(self) -> F {
        self.to_narrow().wrapping_to_fixed()
    }

    fn overflowing_to_fixed<F: Fixed>(self) -> (F, bool) {
        self.to_narrow().overflowing_to_fixed()
    }
}

/// Converts through the wide type, then rounds down onto the narrow grid,
/// assuming the result fits.
impl<Frac: LeEqU128, NFrac: LeEqU64> FromFixed for NarrowFixedU128<Frac, NFrac> {
    fn from_fixed<F: Fixed>(src: F) -> Self {
        Self::from_wide(FixedU128::from_fixed(src))
    }

    fn checked_from_fixed<F: Fixed>(src: F) -> Option<Self> {
        FixedU128::checked_from_fixed(src).map(Self::from_wide)
    }

    fn saturating_from_fixed<F: Fixed>(src: F) -> Self {
        let wide = FixedU128::<Frac>::saturating_from_fixed(src);
        Self::saturating(Some(Self::wide_bits(wide.to_bits())))
    }

    fn wrapping_from_fixed<F: Fixed>(src: F) -> Self {
        Self::from_wide(FixedU128::wrapping_from_fixed(src))
    }

    fn overflowing_from_fixed<F: Fixed>(src: F) -> (Self, bool) {
        let (wide, overflow) = FixedU128::overflowing_from_fixed(src);
        (Self::from_wide(wide), overflow)
    }
}

macro_rules! binary_op {
    ($Imp:ident, $method:ident, $Assign:ident, $assign:ident, $op:ident) => {
        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp for NarrowFixedU128<Frac, NFrac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: Self) -> Self {
                self.$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp<&Self> for NarrowFixedU128<Frac, NFrac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: &Self) -> Self {
                self.$op(*rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp<NarrowFixedU128<Frac, NFrac>>
            for &NarrowFixedU128<Frac, NFrac>
        {
            type Output = NarrowFixedU128<Frac, NFrac>;
            #[inline]
            fn $method(self, rhs: NarrowFixedU128<Frac, NFrac>) -> Self::Output {
                (*self).$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp<&NarrowFixedU128<Frac, NFrac>>
            for &NarrowFixedU128<Frac, NFrac>
        {
            type Output = NarrowFixedU128<Frac, NFrac>;
            #[inline]
            fn $method(self, rhs: &NarrowFixedU128<Frac, NFrac>) -> Self::Output {
                (*self).$op(*rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Assign for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn $assign(&mut self, rhs: Self) {
                *self = (*self).$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Assign<&Self> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn $assign(&mut self, rhs: &Self) {
                *self = (*self).$op(*rhs)
            }
        }
    };
}

binary_op!(Add, add, AddAssign, add_assign, unwrapped_add);
binary_op!(Sub, sub, SubAssign, sub_assign, unwrapped_sub);
binary_op!(Mul, mul, MulAssign, mul_assign, wrapping_mul);
binary_op!(Div, div, DivAssign, div_assign, wrapping_div);

macro_rules! int_op {
    ($Imp:ident, $method:ident, $Assign:ident, $assign:ident, $op:ident) => {
        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp<u128> for NarrowFixedU128<Frac, NFrac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: u128) -> Self {
                self.$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp<&u128> for NarrowFixedU128<Frac, NFrac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: &u128) -> Self {
                self.$op(*rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Imp<u128> for &NarrowFixedU128<Frac, NFrac> {
            type Output = NarrowFixedU128<Frac, NFrac>;
            #[inline]
            fn $method(self, rhs: u128) -> Self::Output {
                (*self).$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64> $Assign<u128> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn $assign(&mut self, rhs: u128) {
                *self = (*self).$op(rhs)
            }
        }
    };
}

int_op!(Mul, mul, MulAssign, mul_assign, unwrapped_mul_int);
int_op!(Div, div, DivAssign, div_assign, unwrapped_div_int);

impl<Frac: LeEqU128, NFrac: LeEqU64> Mul<NarrowFixedU128<Frac, NFrac>> for u128 {
    type Output = NarrowFixedU128<Frac, NFrac>;
    #[inline]
    fn mul(self, rhs: NarrowFixedU128<Frac, NFrac>) -> Self::Output {
        rhs.unwrapped_mul_int(self)
    }
}

/// The literal `$v` (an integer or decimal literal, as `fixed_macro` accepts)
/// as a value of the [`NarrowFixedU128`] type `$T`, evaluated at compile time.
///
/// This is the stand-in for `fixed_macro`'s literal macros. Calling
/// [`NarrowFixedU128::lit`] outside a constant parses at run time instead.
#[macro_export]
macro_rules! narrow_lit {
    ($T:ty, $v:literal) => {{
        const VALUE: $T = <$T>::lit(::core::stringify!($v));
        VALUE
    }};
}
