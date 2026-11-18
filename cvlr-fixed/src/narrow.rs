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
//! # What the Prover sees
//!
//! The Prover models a bitwise or of two non-constant values only by bounds,
//! and on SBF a `u128` shift by a constant, the overflow flag of a checked
//! `u128` product and the reassembly of a `u128` division's operands all
//! compile to one. So range checks here compare with constants, and the
//! narrowing quotient, multiplication by an integer and fixed-point division
//! are computed with `cvlr-mathint`'s native integers, which the Prover models
//! exactly. Native integers are 64 bits wide on the host, so with the `rt`
//! feature those operations use `u128` arithmetic with the same results.
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

#[cfg(not(feature = "rt"))]
use cvlr_mathint::NativeInt;

/// `x / d` rounded down, where the caller has established that the quotient
/// fits in a `u64`, and `d` is a constant power of two.
///
/// On SBF a `u128` shift by a constant compiles to two 64-bit halves joined by
/// a bitwise or, and the Prover models an or of two non-constant values only
/// by bounds. Its native integers model the same division exactly, and
/// returning a `u64` spares the split back into two halves.
#[cfg(not(feature = "rt"))]
#[inline(always)]
fn div_to_u64(x: u128, d: u128) -> u64 {
    u64::from(NativeInt::from(x) / NativeInt::from(d))
}

/// `x / d` rounded down, where the caller has established that the quotient
/// fits in a `u64`. On the host, where native integers are 64 bits wide, a
/// plain division.
#[cfg(feature = "rt")]
#[inline(always)]
fn div_to_u64(x: u128, d: u128) -> u64 {
    (x / d) as u64
}

/// An exact result in narrow units. The variant fixes the scale the value is
/// held at, so that range checks compare it with constants and only an
/// in-range quotient is computed.
#[derive(Clone, Copy)]
enum Exact {
    /// In narrow units.
    Units(u128),
    /// In narrow units times `2^NFrac`: the product of two narrow values.
    Product(u128),
    /// A wide encoding: narrow units times `2^(Frac - NFrac)`.
    Wide(u128),
    /// In narrow units, as a native integer, for results that need more than
    /// 128 bits or a division the Prover should see whole.
    #[cfg(not(feature = "rt"))]
    Native(NativeInt),
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

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> NarrowFixedU128<Frac, NFrac> {
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

    /// The scales of [`Exact::Product`] and [`Exact::Wide`].
    const PRODUCT_SCALE: u128 = 1 << NFrac::U32;
    const WIDE_SCALE: u128 = 1 << Self::SHIFT;

    /// The largest value, held at `scale`, whose quotient is at most `limit`.
    const fn scaled_limit(limit: u128, scale: u128) -> u128 {
        if limit >= u128::MAX / scale {
            u128::MAX
        } else {
            (limit + 1) * scale - 1
        }
    }

    const PRODUCT_WIDE_MAX: u128 = Self::scaled_limit(Self::WIDE_MAX_NARROW, Self::PRODUCT_SCALE);
    const PRODUCT_NARROW_MAX: u128 = Self::scaled_limit(Self::NARROW_MAX, Self::PRODUCT_SCALE);
    const WIDE_NARROW_MAX: u128 = Self::scaled_limit(Self::NARROW_MAX, Self::WIDE_SCALE);

    /// The wide type wraps modulo `2^128` of its encoding: in narrow units,
    /// modulo `2^(128 - Frac + NFrac)`, which is `WIDE_MAX_NARROW + 1`.
    /// Products wrap modulo that times `2^NFrac`.
    const PRODUCT_WRAP_MASK: u128 = Self::scaled_limit(Self::WIDE_MAX_NARROW, Self::PRODUCT_SCALE);

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
        Self::saturating(Some(Exact::Wide(bits)))
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
        Self::assume_fits(Exact::Wide(wide.to_bits()))
    }

    /// The equal value of the narrow type.
    pub const fn to_narrow(self) -> FixedU64<NFrac> {
        FixedU64::from_bits(self.narrow)
    }

    /// The narrow value `narrow`, which is always representable.
    pub const fn from_narrow_fixed(narrow: FixedU64<NFrac>) -> Self {
        Self::from_narrow(narrow.to_bits())
    }

    /// Whether `r` is within the wide range.
    #[inline(always)]
    fn fits_wide(r: Exact) -> bool {
        match r {
            Exact::Units(x) => x <= Self::WIDE_MAX_NARROW,
            Exact::Product(x) => x <= Self::PRODUCT_WIDE_MAX,
            Exact::Wide(_) => true,
            #[cfg(not(feature = "rt"))]
            Exact::Native(n) => n <= Self::WIDE_MAX_NARROW,
        }
    }

    /// Whether `r` is within the narrow range.
    #[inline(always)]
    fn fits_narrow(r: Exact) -> bool {
        match r {
            Exact::Units(x) => x <= Self::NARROW_MAX,
            Exact::Product(x) => x <= Self::PRODUCT_NARROW_MAX,
            Exact::Wide(x) => x <= Self::WIDE_NARROW_MAX,
            #[cfg(not(feature = "rt"))]
            Exact::Native(n) => n <= Self::NARROW_MAX,
        }
    }

    /// `r` in narrow units, which the caller has established fits the narrow
    /// range.
    #[inline(always)]
    fn quotient(r: Exact) -> u64 {
        match r {
            Exact::Units(x) => x as u64,
            Exact::Product(x) => div_to_u64(x, Self::PRODUCT_SCALE),
            Exact::Wide(x) => div_to_u64(x, Self::WIDE_SCALE),
            #[cfg(not(feature = "rt"))]
            Exact::Native(n) => u64::from(n),
        }
    }

    /// `r` wrapped as the wide type wraps.
    #[inline(always)]
    fn wrapped(r: Exact) -> Exact {
        match r {
            Exact::Units(x) => Exact::Units(x & Self::WIDE_MAX_NARROW),
            Exact::Product(x) => Exact::Product(x & Self::PRODUCT_WRAP_MASK),
            Exact::Wide(x) => Exact::Wide(x),
            #[cfg(not(feature = "rt"))]
            Exact::Native(n) => {
                let modulus = NativeInt::from(Self::WIDE_MAX_NARROW) + 1u64;
                Exact::Native(n - n / modulus * modulus)
            }
        }
    }

    #[inline(always)]
    fn assume_fits(r: Exact) -> Self {
        cvlr_assume!(Self::fits_narrow(r));
        Self::from_narrow(Self::quotient(r))
    }

    /// Classifies an exact result `r`: a wide overflow, or a narrow value under
    /// the assumption that it fits. `None` is a result beyond `u128`, which is
    /// always a wide overflow.
    #[inline(always)]
    fn settle(r: Option<Exact>) -> Outcome {
        match r {
            Some(r) if Self::fits_wide(r) => Outcome::Fits(Self::assume_fits(r).narrow),
            _ => Outcome::WideOverflow,
        }
    }

    #[inline(always)]
    fn checked(r: Option<Exact>) -> Option<Self> {
        match Self::settle(r) {
            Outcome::Fits(n) => Some(Self::from_narrow(n)),
            Outcome::WideOverflow => None,
        }
    }

    #[inline(always)]
    fn saturating(r: Option<Exact>) -> Self {
        match r {
            Some(r) if Self::fits_narrow(r) => Self::from_narrow(Self::quotient(r)),
            _ => Self::MAX,
        }
    }

    #[inline(always)]
    fn wrapping(r: Exact) -> Self {
        Self::assume_fits(Self::wrapped(r))
    }

    #[inline(always)]
    fn unwrapped(r: Option<Exact>) -> Self {
        Self::checked(r).expect("overflow")
    }

    // Exact results in narrow units; `None` when they exceed `u128`.

    #[inline(always)]
    fn add_narrow(self, rhs: Self) -> Option<Exact> {
        Some(Exact::Units(self.narrow as u128 + rhs.narrow as u128))
    }

    #[inline(always)]
    fn sub_narrow(self, rhs: Self) -> Option<Exact> {
        (self.narrow as u128)
            .checked_sub(rhs.narrow as u128)
            .map(Exact::Units)
    }

    #[inline(always)]
    fn mul_narrow(self, rhs: Self) -> Exact {
        Exact::Product((self.narrow as u128) * (rhs.narrow as u128))
    }

    /// `(self << NFrac) / rhs`, as one native division: as a `u128` division
    /// it is a call whose operands the Prover reassembles from 64-bit halves.
    #[cfg(not(feature = "rt"))]
    #[inline(always)]
    fn div_narrow(self, rhs: Self) -> Option<Exact> {
        (rhs.narrow != 0).then(|| {
            Exact::Native(NativeInt::from(self.narrow).muldiv(
                NativeInt::from(Self::PRODUCT_SCALE),
                NativeInt::from(rhs.narrow),
            ))
        })
    }

    #[cfg(feature = "rt")]
    #[inline(always)]
    fn div_narrow(self, rhs: Self) -> Option<Exact> {
        (rhs.narrow != 0)
            .then(|| Exact::Units(((self.narrow as u128) << NFrac::U32) / rhs.narrow as u128))
    }

    /// `self * rhs`, as a native product: as a checked `u128` product its
    /// overflow flag is a bitwise or of two flags, which the Prover models
    /// only by bounds.
    #[cfg(not(feature = "rt"))]
    #[inline(always)]
    fn mul_int_narrow(self, rhs: u128) -> Option<Exact> {
        Some(Exact::Native(
            NativeInt::from(self.narrow) * NativeInt::from(rhs),
        ))
    }

    #[cfg(feature = "rt")]
    #[inline(always)]
    fn mul_int_narrow(self, rhs: u128) -> Option<Exact> {
        (self.narrow as u128).checked_mul(rhs).map(Exact::Units)
    }

    /// `self * rhs` wrapped as the wide type wraps.
    #[cfg(not(feature = "rt"))]
    #[inline(always)]
    fn wrapping_mul_int_narrow(self, rhs: u128) -> Exact {
        Self::wrapped(Exact::Native(
            NativeInt::from(self.narrow) * NativeInt::from(rhs),
        ))
    }

    #[cfg(feature = "rt")]
    #[inline(always)]
    fn wrapping_mul_int_narrow(self, rhs: u128) -> Exact {
        Self::wrapped(Exact::Units((self.narrow as u128).wrapping_mul(rhs)))
    }

    /// The integer value `int` plus one, `None` on wide overflow.
    ///
    /// The sum is a native integer. As a `u64` or `u128` sum, LLVM folds the
    /// range check into a comparison of `int` with a constant near `2^64`,
    /// which it encodes as a negative immediate that the Prover reads as a
    /// 256-bit value.
    #[cfg(not(feature = "rt"))]
    #[inline(always)]
    fn int_plus_one(int: Self) -> Option<Self> {
        Self::checked(Some(Exact::Native(
            NativeInt::from(int.narrow) + Self::ONE.narrow,
        )))
    }

    #[cfg(feature = "rt")]
    #[inline(always)]
    fn int_plus_one(int: Self) -> Option<Self> {
        Self::checked(int.add_narrow(Self::ONE))
    }

    /// The narrow units of a value converted at the narrow precision but with
    /// 128 bits of range, and whether that conversion overflowed: the
    /// conversion rounds as the wide type's does, and overflows only beyond
    /// the wide range.
    #[inline(always)]
    fn from_units((units, overflow): (FixedU128<NFrac>, bool)) -> Option<Exact> {
        (!overflow).then(|| Exact::Units(units.to_bits()))
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

    // Conversions into the type convert at the narrow precision, with 128
    // bits of range: the value rounds down as it does into the wide type, and
    // the result is in narrow units without a wide shift.

    /// Converts as the wide type's `from_num` does, then rounds down onto the
    /// narrow grid, assuming the result fits. A value outside the wide range
    /// wraps as the wide type's does in a release build.
    pub fn from_num<Src: ToFixed>(src: Src) -> Self {
        let (units, overflow) = src.overflowing_to_fixed::<FixedU128<NFrac>>();
        let r = Exact::Units(units.to_bits());
        if !overflow && Self::fits_wide(r) {
            Self::assume_fits(r)
        } else {
            Self::wrapping(r)
        }
    }

    /// Converts as the wide type's `checked_from_num` does: `None` when the
    /// value does not fit the wide type.
    pub fn checked_from_num<Src: ToFixed>(src: Src) -> Option<Self> {
        Self::checked(Self::from_units(src.overflowing_to_fixed()))
    }

    /// Converts, saturating at [`MAX`](Self::MAX).
    pub fn saturating_from_num<Src: ToFixed>(src: Src) -> Self {
        Self::saturating(Self::from_units(src.overflowing_to_fixed()))
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
            Self::int_plus_one(int)
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
            Self::int_plus_one(self.int())
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
        (rhs != 0).then(|| self.unwrapped_div_int(rhs))
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
        Self::wrapping(Exact::Units(self.narrow as u128 + rhs.narrow as u128))
    }

    /// `self - rhs`, wrapping as the wide type does.
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self::wrapping(Exact::Units(
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
        Self::wrapping(self.wrapping_mul_int_narrow(rhs))
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
        // A 64-bit division: a divisor beyond `u64` gives zero either way.
        if rhs > u64::MAX as u128 {
            Self::ZERO
        } else {
            Self::from_narrow(self.narrow / rhs as u64)
        }
    }
}

impl<Frac, NFrac> Clone for NarrowFixedU128<Frac, NFrac> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Frac, NFrac> Copy for NarrowFixedU128<Frac, NFrac> {}

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> Default for NarrowFixedU128<Frac, NFrac> {
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

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> fmt::Debug for NarrowFixedU128<Frac, NFrac> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_wide(), f)
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> fmt::Display for NarrowFixedU128<Frac, NFrac> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_wide(), f)
    }
}

// `fixed` converts these losslessly into every wide type with enough integer
// bits; the narrow value assumes the integer fits its range, and is the
// integer shifted by a 64-bit shift.
macro_rules! from_int {
    ($($Int:ty),*) => { $(
        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> From<$Int> for NarrowFixedU128<Frac, NFrac> {
            #[inline(always)]
            fn from(src: $Int) -> Self {
                let src = u64::from(src);
                cvlr_assume!(src <= u64::MAX >> NFrac::U32);
                Self::from_narrow(src << NFrac::U32)
            }
        }
    )* };
}

from_int!(bool, u8, u16, u32, u64);

// Exact comparisons with primitive integers, in both directions, as `fixed`
// provides them, made through the equal narrow value.
macro_rules! cmp_int {
    ($($Int:ty),*) => { $(
        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> PartialEq<$Int> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn eq(&self, rhs: &$Int) -> bool {
                self.to_narrow() == *rhs
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> PartialEq<NarrowFixedU128<Frac, NFrac>> for $Int {
            #[inline]
            fn eq(&self, rhs: &NarrowFixedU128<Frac, NFrac>) -> bool {
                *self == rhs.to_narrow()
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> PartialOrd<$Int> for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Int) -> Option<Ordering> {
                self.to_narrow().partial_cmp(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> PartialOrd<NarrowFixedU128<Frac, NFrac>> for $Int {
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
impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> FromStr for NarrowFixedU128<Frac, NFrac> {
    type Err = ParseFixedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FixedU128::<Frac>::from_str(s).map(Self::from_wide)
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> cvlr_nondet::Nondet
    for NarrowFixedU128<Frac, NFrac>
{
    fn nondet() -> Self {
        Self::from_narrow(cvlr_nondet::nondet())
    }
}

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> cvlr_log::CvlrLog for NarrowFixedU128<Frac, NFrac> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        logger.log_u64_as_fp(tag, self.narrow, NFrac::U64);
    }
}

/// Converts through the equal narrow value.
impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> ToFixed for NarrowFixedU128<Frac, NFrac> {
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

/// Converts as the wide type does, then rounds down onto the narrow grid,
/// assuming the result fits.
impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> FromFixed for NarrowFixedU128<Frac, NFrac> {
    fn from_fixed<F: Fixed>(src: F) -> Self {
        Self::from_num(src)
    }

    fn checked_from_fixed<F: Fixed>(src: F) -> Option<Self> {
        Self::checked_from_num(src)
    }

    fn saturating_from_fixed<F: Fixed>(src: F) -> Self {
        Self::saturating_from_num(src)
    }

    fn wrapping_from_fixed<F: Fixed>(src: F) -> Self {
        Self::overflowing_from_fixed(src).0
    }

    fn overflowing_from_fixed<F: Fixed>(src: F) -> (Self, bool) {
        let (units, overflow) = src.overflowing_to_fixed::<FixedU128<NFrac>>();
        let r = Exact::Units(units.to_bits());
        (Self::wrapping(r), overflow || !Self::fits_wide(r))
    }
}

macro_rules! binary_op {
    ($Imp:ident, $method:ident, $Assign:ident, $assign:ident, $op:ident) => {
        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp for NarrowFixedU128<Frac, NFrac> {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: Self) -> Self {
                self.$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp<&Self>
            for NarrowFixedU128<Frac, NFrac>
        {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: &Self) -> Self {
                self.$op(*rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp<NarrowFixedU128<Frac, NFrac>>
            for &NarrowFixedU128<Frac, NFrac>
        {
            type Output = NarrowFixedU128<Frac, NFrac>;
            #[inline]
            fn $method(self, rhs: NarrowFixedU128<Frac, NFrac>) -> Self::Output {
                (*self).$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp<&NarrowFixedU128<Frac, NFrac>>
            for &NarrowFixedU128<Frac, NFrac>
        {
            type Output = NarrowFixedU128<Frac, NFrac>;
            #[inline]
            fn $method(self, rhs: &NarrowFixedU128<Frac, NFrac>) -> Self::Output {
                (*self).$op(*rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Assign for NarrowFixedU128<Frac, NFrac> {
            #[inline]
            fn $assign(&mut self, rhs: Self) {
                *self = (*self).$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Assign<&Self>
            for NarrowFixedU128<Frac, NFrac>
        {
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
        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp<u128>
            for NarrowFixedU128<Frac, NFrac>
        {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: u128) -> Self {
                self.$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp<&u128>
            for NarrowFixedU128<Frac, NFrac>
        {
            type Output = Self;
            #[inline]
            fn $method(self, rhs: &u128) -> Self {
                self.$op(*rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Imp<u128>
            for &NarrowFixedU128<Frac, NFrac>
        {
            type Output = NarrowFixedU128<Frac, NFrac>;
            #[inline]
            fn $method(self, rhs: u128) -> Self::Output {
                (*self).$op(rhs)
            }
        }

        impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> $Assign<u128>
            for NarrowFixedU128<Frac, NFrac>
        {
            #[inline]
            fn $assign(&mut self, rhs: u128) {
                *self = (*self).$op(rhs)
            }
        }
    };
}

int_op!(Mul, mul, MulAssign, mul_assign, unwrapped_mul_int);
int_op!(Div, div, DivAssign, div_assign, unwrapped_div_int);

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> Mul<NarrowFixedU128<Frac, NFrac>> for u128 {
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
