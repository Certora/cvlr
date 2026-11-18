//! Prover rules for `cvlr_fixed::narrow::NarrowFixedU128`, run on the code the
//! Prover analyses: the native-integer paths, which the host tests in
//! `narrow_equiv.rs` cannot reach.
//!
//! The rules are generic over the type and instantiated for three
//! configurations: `<U60, U14>`, a typical one, with a band of results that
//! fit the wide type but not the narrow one; `<U64, U0>`, with a whole-word
//! shift and no fraction; and `<U127, U63>`, the widest fraction, where one
//! is `2^63`. In the last two the narrow and wide ranges coincide.
//!
//! Each `check_*` rule states an operation's result in native integers: the
//! exact result in narrow units, `None` beyond the wide range, and nothing
//! (the execution is assumed away) between the narrow and the wide range. The
//! reference is not `fixed` itself, whose 128-bit shifts the Prover models
//! only approximately.
//!
//! A `check_*` rule passes vacuously on a branch the implementation assumes
//! away, and `rule_sanity` only checks that the rule as a whole is reachable.
//! So each `reach_*` rule witnesses, with `cvlr_satisfy!`, one outcome a
//! `check_*` rule depends on: a `None` result, a saturated one, a wrap where
//! the modulus bites, and an in-range result with the top bit of the narrow
//! word set. A configuration instantiates only the witnesses its ranges make
//! reachable.
//!
//! Constants of `2^63` or more are built from small factors: LLVM encodes
//! them as negative immediates, which the Prover reads as 256-bit values
//! unless `-solanaTACSoundSignedMath` is on.
//!
//! Run with `narrow_prover.conf` and `narrow_prover_sound_signed.conf`; see
//! their header.

use core::marker::PhantomData;

use cvlr::mathint::NativeInt;
use cvlr::prelude::*;
use cvlr_fixed::narrow::NarrowFixedU128;
use fixed::types::extra::{LeEqU128, LeEqU64, U0, U127, U14, U32, U60, U63, U64};
use fixed::{FixedU128, FixedU64};

/// `2^k` for `k <= 128`, as a product of factors below `2^63`.
fn pow2(k: u32) -> NativeInt {
    let a = k / 3;
    let b = (k - a) / 2;
    let c = k - a - b;
    NativeInt::from(1u64 << a) * NativeInt::from(1u64 << b) * NativeInt::from(1u64 << c)
}

fn native(x: u64) -> NativeInt {
    NativeInt::from(x)
}

fn narrow_max() -> NativeInt {
    NativeInt::u64_max()
}

fn top_bit_set(x: u64) -> bool {
    native(x) >= pow2(63)
}

fn pair() -> (u64, u64) {
    (nondet(), nondet())
}

struct Rules<Frac, NFrac>(PhantomData<(Frac, NFrac)>);

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> Rules<Frac, NFrac> {
    const SHIFT: u32 = Frac::U32 - NFrac::U32;
    const ONE_UNITS: u64 = 1 << NFrac::U32;

    fn one() -> NativeInt {
        pow2(NFrac::U32)
    }

    /// The wide type's range in narrow units: one more than its largest value.
    fn wide_limit() -> NativeInt {
        pow2(128 - Self::SHIFT)
    }

    fn value(units: u64) -> NarrowFixedU128<Frac, NFrac> {
        NarrowFixedU128::from_narrow_fixed(FixedU64::from_bits(units))
    }

    fn units(x: NarrowFixedU128<Frac, NFrac>) -> u64 {
        x.to_narrow().to_bits()
    }

    fn native_units(x: NarrowFixedU128<Frac, NFrac>) -> NativeInt {
        native(Self::units(x))
    }

    /// A `checked_*` result against the exact result `e`.
    fn check_checked(r: Option<NarrowFixedU128<Frac, NFrac>>, e: NativeInt) {
        if e >= Self::wide_limit() {
            cvlr_assert!(r.is_none());
        } else {
            cvlr_assert!(r.is_some());
            if let Some(r) = r {
                cvlr_assert!(Self::native_units(r) == e);
            }
        }
    }

    /// A `saturating_*` result against the exact result `e`.
    fn check_saturating(r: NarrowFixedU128<Frac, NFrac>, e: NativeInt) {
        if e > narrow_max() {
            cvlr_assert!(Self::native_units(r) == narrow_max());
        } else {
            cvlr_assert!(Self::native_units(r) == e);
        }
    }

    fn wrapped(e: NativeInt) -> NativeInt {
        let modulus = Self::wide_limit();
        e - e / modulus * modulus
    }

    /// A wrapping result against the exact result `e`: `e` modulo the wide
    /// range, which must then fit the narrow one.
    fn check_wrapping(r: NarrowFixedU128<Frac, NFrac>, e: NativeInt) {
        cvlr_assert!(Self::native_units(r) == Self::wrapped(e));
    }

    fn mul_exact(a: u64, b: u64) -> NativeInt {
        native(a).muldiv(native(b), Self::one())
    }

    fn div_exact(a: u64, b: u64) -> NativeInt {
        native(a).muldiv(Self::one(), native(b))
    }

    fn add() {
        let (a, b) = pair();
        let (x, y) = (Self::value(a), Self::value(b));
        let e = native(a) + native(b);
        Self::check_checked(x.checked_add(y), e);
        Self::check_saturating(x.saturating_add(y), e);
        Self::check_wrapping(x.wrapping_add(y), e);
    }

    fn sub() {
        let (a, b) = pair();
        let (x, y) = (Self::value(a), Self::value(b));
        let r = x.checked_sub(y);
        Self::check_wrapping(
            x.wrapping_sub(y),
            native(a) + Self::wide_limit() - native(b),
        );
        if a < b {
            cvlr_assert!(r.is_none());
            cvlr_assert!(Self::units(x.saturating_sub(y)) == 0);
        } else {
            cvlr_assert!(r.is_some() && Self::units(r.unwrap()) == a - b);
            cvlr_assert!(Self::units(x.saturating_sub(y)) == a - b);
        }
    }

    fn mul_checked() {
        let (a, b) = pair();
        Self::check_checked(
            Self::value(a).checked_mul(Self::value(b)),
            Self::mul_exact(a, b),
        );
    }

    fn mul_saturating() {
        let (a, b) = pair();
        Self::check_saturating(
            Self::value(a).saturating_mul(Self::value(b)),
            Self::mul_exact(a, b),
        );
    }

    fn mul_operator() {
        let (a, b) = pair();
        Self::check_wrapping(Self::value(a) * Self::value(b), Self::mul_exact(a, b));
    }

    fn div_checked() {
        let (a, b) = pair();
        let r = Self::value(a).checked_div(Self::value(b));
        if b == 0 {
            cvlr_assert!(r.is_none());
        } else {
            Self::check_checked(r, Self::div_exact(a, b));
        }
    }

    fn div_saturating() {
        let (a, b) = pair();
        cvlr_assume!(b != 0);
        Self::check_saturating(
            Self::value(a).saturating_div(Self::value(b)),
            Self::div_exact(a, b),
        );
    }

    fn div_operator() {
        let (a, b) = pair();
        cvlr_assume!(b != 0);
        Self::check_wrapping(Self::value(a) / Self::value(b), Self::div_exact(a, b));
    }

    fn mul_int() {
        let a: u64 = nondet();
        let n: u128 = nondet();
        let x = Self::value(a);
        let e = native(a) * NativeInt::from(n);
        Self::check_checked(x.checked_mul_int(n), e);
        Self::check_saturating(x.saturating_mul_int(n), e);
        Self::check_wrapping(x.wrapping_mul_int(n), e);
    }

    fn div_int() {
        let a: u64 = nondet();
        let n: u128 = nondet();
        let x = Self::value(a);
        let r = x.checked_div_int(n);
        if n == 0 {
            cvlr_assert!(r.is_none());
        } else {
            cvlr_assert!(r.is_some());
            let r = r.unwrap();
            cvlr_assert!(Self::native_units(r) == native(a) / NativeInt::from(n));
            cvlr_assert!(Self::units(x / n) == Self::units(r));
        }
    }

    fn from_bits() {
        let bits: u128 = nondet();
        let r = NarrowFixedU128::<Frac, NFrac>::from_bits(bits);
        Self::check_saturating(r, NativeInt::from(bits) / pow2(Self::SHIFT));
    }

    fn to_bits() {
        let a: u64 = nondet();
        let bits = Self::value(a).to_bits();
        cvlr_assert!(NativeInt::from(bits) == native(a) * pow2(Self::SHIFT));
        cvlr_assert!(Self::units(NarrowFixedU128::from_bits(bits)) == a);
        cvlr_assert!(NarrowFixedU128::<Frac, NFrac>::is_representable_bits(bits));
    }

    fn ceil_round() {
        let a: u64 = nondet();
        let frac = a % Self::ONE_UNITS;
        let int = a - frac;
        let up = native(int) + Self::one();
        let x = Self::value(a);
        Self::check_checked(x.checked_ceil(), if frac == 0 { native(a) } else { up });
        let rounds_down = NFrac::U32 == 0 || frac < Self::ONE_UNITS / 2;
        Self::check_checked(
            x.checked_round(),
            if rounds_down { native(int) } else { up },
        );
    }

    fn from_int() {
        let v: u64 = nondet();
        let e = native(v) * Self::one();
        Self::check_checked(NarrowFixedU128::checked_from_num(v), e);
        Self::check_saturating(NarrowFixedU128::saturating_from_num(v), e);
        Self::check_wrapping(NarrowFixedU128::from_num(v), e);
        cvlr_assert!(Self::native_units(NarrowFixedU128::from(v)) == e);
    }

    fn from_signed() {
        let v: i64 = nondet();
        cvlr_assume!(v < 0);
        cvlr_assert!(NarrowFixedU128::<Frac, NFrac>::checked_from_num(v).is_none());
        cvlr_assert!(Self::units(NarrowFixedU128::saturating_from_num(v)) == 0);
    }

    fn from_fixed() {
        let x: u64 = nondet();
        let src = FixedU64::<U32>::from_bits(x);
        // `x / 2^32` in narrow units, rounded down.
        let e = native(x).muldiv(Self::one(), pow2(32));
        Self::check_checked(NarrowFixedU128::checked_from_num(src), e);
        Self::check_saturating(NarrowFixedU128::saturating_from_num(src), e);
    }

    fn to_num_and_compare() {
        let a: u64 = nondet();
        let v: u64 = nondet();
        let x = Self::value(a);
        cvlr_assert!(native(x.to_num::<u64>()) == native(a) / Self::one());
        let scaled = native(v) * Self::one();
        cvlr_assert!((x < v) == (native(a) < scaled));
        cvlr_assert!((x == v) == (native(a) == scaled));
    }

    fn to_wide() {
        let a: u64 = nondet();
        let wide: FixedU128<Frac> = Self::value(a).to_wide();
        cvlr_assert!(NativeInt::from(wide.to_bits()) == native(a) * pow2(Self::SHIFT));
    }

    // Witnesses.

    fn reach_add_large() {
        let (a, b) = pair();
        let r = Self::value(a).checked_add(Self::value(b));
        cvlr_satisfy!(r.is_some_and(|r| top_bit_set(Self::units(r))));
    }

    fn reach_add_saturated() {
        let (a, b) = pair();
        cvlr_assume!(native(a) + native(b) > narrow_max());
        let r = Self::value(a).saturating_add(Self::value(b));
        cvlr_satisfy!(Self::native_units(r) == narrow_max());
    }

    fn reach_add_none() {
        let (a, b) = pair();
        cvlr_satisfy!(Self::value(a).checked_add(Self::value(b)).is_none());
    }

    fn reach_add_wraps() {
        let (a, b) = pair();
        let e = native(a) + native(b);
        cvlr_assume!(e >= Self::wide_limit());
        let r = Self::value(a).wrapping_add(Self::value(b));
        cvlr_satisfy!(Self::native_units(r) == Self::wrapped(e));
    }

    fn reach_sub_wraps() {
        let (a, b) = pair();
        cvlr_assume!(a < b);
        let e = native(a) + Self::wide_limit() - native(b);
        let r = Self::value(a).wrapping_sub(Self::value(b));
        cvlr_satisfy!(Self::native_units(r) == Self::wrapped(e));
    }

    fn reach_mul_large() {
        let (a, b) = pair();
        let r = Self::value(a).checked_mul(Self::value(b));
        cvlr_satisfy!(r.is_some_and(|r| top_bit_set(Self::units(r))));
    }

    fn reach_mul_saturated() {
        let (a, b) = pair();
        cvlr_assume!(Self::mul_exact(a, b) > narrow_max());
        let r = Self::value(a).saturating_mul(Self::value(b));
        cvlr_satisfy!(Self::native_units(r) == narrow_max());
    }

    fn reach_mul_none() {
        let (a, b) = pair();
        cvlr_satisfy!(Self::value(a).checked_mul(Self::value(b)).is_none());
    }

    fn reach_mul_wraps() {
        let (a, b) = pair();
        let e = Self::mul_exact(a, b);
        cvlr_assume!(e >= Self::wide_limit());
        let r = Self::value(a) * Self::value(b);
        cvlr_satisfy!(Self::native_units(r) == Self::wrapped(e));
    }

    fn reach_div_large() {
        let (a, b) = pair();
        let r = Self::value(a).checked_div(Self::value(b));
        cvlr_satisfy!(r.is_some_and(|r| top_bit_set(Self::units(r))));
    }

    fn reach_div_saturated() {
        let (a, b) = pair();
        cvlr_assume!(b != 0 && Self::div_exact(a, b) > narrow_max());
        let r = Self::value(a).saturating_div(Self::value(b));
        cvlr_satisfy!(Self::native_units(r) == narrow_max());
    }

    fn reach_div_none() {
        let (a, b) = pair();
        cvlr_assume!(b != 0);
        cvlr_satisfy!(Self::value(a).checked_div(Self::value(b)).is_none());
    }

    fn reach_div_wraps() {
        let (a, b) = pair();
        cvlr_assume!(b != 0);
        let e = Self::div_exact(a, b);
        cvlr_assume!(e >= Self::wide_limit());
        let r = Self::value(a) / Self::value(b);
        cvlr_satisfy!(Self::native_units(r) == Self::wrapped(e));
    }

    fn reach_mul_int_large() {
        let a: u64 = nondet();
        let n: u128 = nondet();
        let r = Self::value(a).checked_mul_int(n);
        cvlr_satisfy!(r.is_some_and(|r| top_bit_set(Self::units(r))));
    }

    fn reach_mul_int_saturated() {
        let a: u64 = nondet();
        let n: u128 = nondet();
        cvlr_assume!(native(a) * NativeInt::from(n) > narrow_max());
        let r = Self::value(a).saturating_mul_int(n);
        cvlr_satisfy!(Self::native_units(r) == narrow_max());
    }

    fn reach_mul_int_none() {
        let a: u64 = nondet();
        let n: u128 = nondet();
        cvlr_satisfy!(Self::value(a).checked_mul_int(n).is_none());
    }

    fn reach_mul_int_wraps() {
        let a: u64 = nondet();
        let n: u128 = nondet();
        let e = native(a) * NativeInt::from(n);
        cvlr_assume!(e >= Self::wide_limit());
        let r = Self::value(a).wrapping_mul_int(n);
        cvlr_satisfy!(Self::native_units(r) == Self::wrapped(e));
    }

    fn reach_from_bits_large() {
        let bits: u128 = nondet();
        let r = NarrowFixedU128::<Frac, NFrac>::from_bits(bits);
        cvlr_satisfy!(top_bit_set(Self::units(r)));
    }

    fn reach_from_bits_saturated() {
        let bits: u128 = nondet();
        cvlr_assume!(NativeInt::from(bits) / pow2(Self::SHIFT) > narrow_max());
        let r = NarrowFixedU128::<Frac, NFrac>::from_bits(bits);
        cvlr_satisfy!(Self::native_units(r) == narrow_max());
    }

    fn reach_ceil_large() {
        let a: u64 = nondet();
        let r = Self::value(a).checked_ceil();
        cvlr_satisfy!(r.is_some_and(|r| top_bit_set(Self::units(r))));
    }

    fn reach_ceil_none() {
        let a: u64 = nondet();
        cvlr_satisfy!(Self::value(a).checked_ceil().is_none());
    }

    fn reach_from_int_saturated() {
        let v: u64 = nondet();
        cvlr_assume!(native(v) * Self::one() > narrow_max());
        let r = NarrowFixedU128::<Frac, NFrac>::saturating_from_num(v);
        cvlr_satisfy!(Self::native_units(r) == narrow_max());
    }

    fn reach_from_int_none() {
        let v: u64 = nondet();
        cvlr_satisfy!(NarrowFixedU128::<Frac, NFrac>::checked_from_num(v).is_none());
    }
}

/// Instantiates `Rules<$Frac, $NFrac>`: each `$rule => $method` is a rule
/// named `$rule` that runs `Rules::$method`.
macro_rules! instantiate {
    ($Frac:ty, $NFrac:ty { $($rule:ident => $method:ident),* $(,)? }) => {
        $(
            #[rule]
            pub fn $rule() {
                Rules::<$Frac, $NFrac>::$method();
            }
        )*
    };
}

instantiate!(U60, U14 {
    check_add_u60_u14 => add,
    check_sub_u60_u14 => sub,
    check_mul_checked_u60_u14 => mul_checked,
    check_mul_saturating_u60_u14 => mul_saturating,
    check_mul_operator_u60_u14 => mul_operator,
    check_div_checked_u60_u14 => div_checked,
    check_div_saturating_u60_u14 => div_saturating,
    check_div_operator_u60_u14 => div_operator,
    check_mul_int_u60_u14 => mul_int,
    check_div_int_u60_u14 => div_int,
    check_from_bits_u60_u14 => from_bits,
    check_to_bits_u60_u14 => to_bits,
    check_ceil_round_u60_u14 => ceil_round,
    check_from_int_u60_u14 => from_int,
    check_from_signed_u60_u14 => from_signed,
    check_from_fixed_u60_u14 => from_fixed,
    check_to_num_and_compare_u60_u14 => to_num_and_compare,
    check_to_wide_u60_u14 => to_wide,
    // A sum or quotient of two narrow values, a ceiling and an integer are
    // all within the wide range, so those never fail or wrap; a wrapped
    // difference is beyond the narrow range.
    reach_add_large_u60_u14 => reach_add_large,
    reach_add_saturated_u60_u14 => reach_add_saturated,
    reach_mul_large_u60_u14 => reach_mul_large,
    reach_mul_saturated_u60_u14 => reach_mul_saturated,
    reach_mul_none_u60_u14 => reach_mul_none,
    reach_mul_wraps_u60_u14 => reach_mul_wraps,
    reach_div_large_u60_u14 => reach_div_large,
    reach_div_saturated_u60_u14 => reach_div_saturated,
    reach_mul_int_large_u60_u14 => reach_mul_int_large,
    reach_mul_int_saturated_u60_u14 => reach_mul_int_saturated,
    reach_mul_int_none_u60_u14 => reach_mul_int_none,
    reach_mul_int_wraps_u60_u14 => reach_mul_int_wraps,
    reach_from_bits_large_u60_u14 => reach_from_bits_large,
    reach_from_bits_saturated_u60_u14 => reach_from_bits_saturated,
    reach_ceil_large_u60_u14 => reach_ceil_large,
    reach_from_int_saturated_u60_u14 => reach_from_int_saturated,
});

instantiate!(U64, U0 {
    check_add_u64_u0 => add,
    check_sub_u64_u0 => sub,
    check_mul_checked_u64_u0 => mul_checked,
    check_mul_saturating_u64_u0 => mul_saturating,
    check_mul_operator_u64_u0 => mul_operator,
    check_div_checked_u64_u0 => div_checked,
    check_div_saturating_u64_u0 => div_saturating,
    check_div_operator_u64_u0 => div_operator,
    check_mul_int_u64_u0 => mul_int,
    check_div_int_u64_u0 => div_int,
    check_from_bits_u64_u0 => from_bits,
    check_to_bits_u64_u0 => to_bits,
    check_ceil_round_u64_u0 => ceil_round,
    check_from_int_u64_u0 => from_int,
    check_from_signed_u64_u0 => from_signed,
    check_from_fixed_u64_u0 => from_fixed,
    check_to_num_and_compare_u64_u0 => to_num_and_compare,
    check_to_wide_u64_u0 => to_wide,
    // The wide and narrow ranges coincide, so a result that saturates also
    // fails or wraps. A quotient of integers, the ceiling of an integer, an
    // integer, and the integer part of a wide encoding all fit.
    reach_add_large_u64_u0 => reach_add_large,
    reach_add_saturated_u64_u0 => reach_add_saturated,
    reach_add_none_u64_u0 => reach_add_none,
    reach_add_wraps_u64_u0 => reach_add_wraps,
    reach_sub_wraps_u64_u0 => reach_sub_wraps,
    reach_mul_large_u64_u0 => reach_mul_large,
    reach_mul_saturated_u64_u0 => reach_mul_saturated,
    reach_mul_none_u64_u0 => reach_mul_none,
    reach_mul_wraps_u64_u0 => reach_mul_wraps,
    reach_div_large_u64_u0 => reach_div_large,
    reach_mul_int_large_u64_u0 => reach_mul_int_large,
    reach_mul_int_saturated_u64_u0 => reach_mul_int_saturated,
    reach_mul_int_none_u64_u0 => reach_mul_int_none,
    reach_mul_int_wraps_u64_u0 => reach_mul_int_wraps,
    reach_from_bits_large_u64_u0 => reach_from_bits_large,
    reach_ceil_large_u64_u0 => reach_ceil_large,
});

instantiate!(U127, U63 {
    check_add_u127_u63 => add,
    check_sub_u127_u63 => sub,
    check_mul_checked_u127_u63 => mul_checked,
    check_mul_saturating_u127_u63 => mul_saturating,
    check_mul_operator_u127_u63 => mul_operator,
    check_div_checked_u127_u63 => div_checked,
    check_div_saturating_u127_u63 => div_saturating,
    check_div_operator_u127_u63 => div_operator,
    check_mul_int_u127_u63 => mul_int,
    check_div_int_u127_u63 => div_int,
    check_from_bits_u127_u63 => from_bits,
    check_to_bits_u127_u63 => to_bits,
    check_ceil_round_u127_u63 => ceil_round,
    check_from_int_u127_u63 => from_int,
    check_from_signed_u127_u63 => from_signed,
    check_from_fixed_u127_u63 => from_fixed,
    check_to_num_and_compare_u127_u63 => to_num_and_compare,
    check_to_wide_u127_u63 => to_wide,
    // The wide and narrow ranges coincide; the integer part of a wide
    // encoding fits.
    reach_add_large_u127_u63 => reach_add_large,
    reach_add_saturated_u127_u63 => reach_add_saturated,
    reach_add_none_u127_u63 => reach_add_none,
    reach_add_wraps_u127_u63 => reach_add_wraps,
    reach_sub_wraps_u127_u63 => reach_sub_wraps,
    reach_mul_large_u127_u63 => reach_mul_large,
    reach_mul_saturated_u127_u63 => reach_mul_saturated,
    reach_mul_none_u127_u63 => reach_mul_none,
    reach_mul_wraps_u127_u63 => reach_mul_wraps,
    reach_div_large_u127_u63 => reach_div_large,
    reach_div_saturated_u127_u63 => reach_div_saturated,
    reach_div_none_u127_u63 => reach_div_none,
    reach_div_wraps_u127_u63 => reach_div_wraps,
    reach_mul_int_large_u127_u63 => reach_mul_int_large,
    reach_mul_int_saturated_u127_u63 => reach_mul_int_saturated,
    reach_mul_int_none_u127_u63 => reach_mul_int_none,
    reach_mul_int_wraps_u127_u63 => reach_mul_int_wraps,
    reach_from_bits_large_u127_u63 => reach_from_bits_large,
    reach_ceil_large_u127_u63 => reach_ceil_large,
    reach_ceil_none_u127_u63 => reach_ceil_none,
    reach_from_int_saturated_u127_u63 => reach_from_int_saturated,
    reach_from_int_none_u127_u63 => reach_from_int_none,
});
