//! Prover rules for `cvlr_fixed::exact::ExactFixedU128`, run on the code the
//! Prover analyses: the native-integer multiplication, division, rounding,
//! integer comparisons and conversions from integers, which the host tests
//! cannot reach (on the host they are `fixed`'s own, or computed from its
//! `int` and `frac`). `spec_to_num` and `spec_from_num` check two of the
//! conversions that remain `fixed`'s code.
//!
//! The rules are generic over the type and instantiated for three fractional
//! widths: `U60`, a typical one; `U0`, with no fraction; and `U127`, the
//! widest that has a one.
//!
//! Each `spec_*` rule states an operation's result in native integers,
//! characterised rather than computed the way the implementation computes it:
//! the quotient `q` of a product is the one with `q * 2^Frac <= a * b <
//! (q + 1) * 2^Frac`, an overflow is exactly a quotient of `2^128` or more,
//! and an overflowing result is the quotient modulo `2^128`. Each `reach_*`
//! rule witnesses, with `cvlr_satisfy!`, an outcome a `spec_*` rule depends
//! on. That `fixed` computes the same quotients is checked on the host
//! (`fixed_is_floor_*` in `exact_equiv.rs`): a rule comparing with `fixed`
//! directly gets spurious counterexamples, because the Prover models the
//! bitwise operations in its 256-bit arithmetic only approximately.
//!
//! The fixture builds `fixed` with debug assertions on, so its `*`, `/`,
//! `ceil` and `round` panic on overflow. Each `spec_*_overflow_panics` rule
//! asserts `false` after an overflowing call: its assertions verify and its
//! sanity check fails, because no execution gets past the call, which is the
//! claim. Division
//! by zero is not a rule: the compiler sees that it always panics and removes
//! the assertion after it.
//!
//! Constants of `2^63` or more are built from small factors: LLVM encodes
//! them as negative immediates, which the Prover reads as 256-bit values
//! unless `-solanaTACSoundSignedMath` is on. The same reading breaks
//! `Ordering::reverse`, which negates an `i8`, so the implementation compares
//! an integer with a fixed-point value without it.
//!
//! Run with `exact_prover.conf`; see its header.

use core::marker::PhantomData;

use cvlr::mathint::NativeInt;
use cvlr::prelude::*;
use cvlr_fixed::exact::ExactFixedU128;
use fixed::types::extra::{LeEqU128, U0, U127, U60};

/// `2^k` for `k <= 256`, as a product of factors of at most `2^62`.
fn pow2(k: u32) -> NativeInt {
    let factor = |n: u32| NativeInt::from(1u64 << n);
    let a = k.min(62);
    let b = (k - a).min(62);
    let c = (k - a - b).min(62);
    let d = (k - a - b - c).min(62);
    let e = k - a - b - c - d;
    factor(a) * factor(b) * factor(c) * factor(d) * factor(e)
}

fn native(x: u128) -> NativeInt {
    NativeInt::from(x)
}

fn native_u64(x: u64) -> NativeInt {
    NativeInt::from(x)
}

/// `x` modulo `2^128`.
fn mod_2_128(x: NativeInt) -> NativeInt {
    x - pow2(128) * (x / pow2(128))
}

struct Rules<Frac>(PhantomData<Frac>);

impl<Frac: LeEqU128> Rules<Frac> {
    fn scale() -> NativeInt {
        pow2(Frac::U32)
    }

    fn value(bits: u128) -> ExactFixedU128<Frac> {
        ExactFixedU128::from_bits(bits)
    }

    fn pair() -> (u128, u128) {
        (nondet(), nondet())
    }

    /// The exact product's floor quotient, and whether it overflows.
    fn product(a: u128, b: u128) -> (NativeInt, bool) {
        let ab = native(a) * native(b);
        let q = ab / Self::scale();
        (q, ab >= pow2(128) * Self::scale())
    }

    /// The exact quotient's floor, and whether it overflows; `b` non-zero.
    fn quotient(a: u128, b: u128) -> (NativeInt, bool) {
        let num = native(a) * Self::scale();
        let q = num / native(b);
        (q, num >= pow2(128) * native(b))
    }

    pub fn spec_mul() {
        let (a, b) = Self::pair();
        let (r, overflow) = Self::value(a).overflowing_mul(Self::value(b));
        let ab = native(a) * native(b);
        let (q, wide) = Self::product(a, b);
        cvlr_assert!(overflow == wide);
        cvlr_assert!(q * Self::scale() <= ab);
        cvlr_assert!(ab < (q + NativeInt::from(1u64)) * Self::scale());
        cvlr_assert!(native(r.to_bits()) == mod_2_128(q));
    }

    pub fn spec_div() {
        let (a, b) = Self::pair();
        cvlr_assume!(b != 0);
        let (r, overflow) = Self::value(a).overflowing_div(Self::value(b));
        let num = native(a) * Self::scale();
        let (q, wide) = Self::quotient(a, b);
        cvlr_assert!(overflow == wide);
        cvlr_assert!(q * native(b) <= num);
        cvlr_assert!(num < (q + NativeInt::from(1u64)) * native(b));
        cvlr_assert!(native(r.to_bits()) == mod_2_128(q));
    }

    pub fn spec_mul_forms() {
        let (a, b) = Self::pair();
        let (x, y) = (Self::value(a), Self::value(b));
        let (r, overflow) = x.overflowing_mul(y);
        cvlr_assert!(x.checked_mul(y) == if overflow { None } else { Some(r) });
        cvlr_assert!(x.saturating_mul(y) == if overflow { ExactFixedU128::MAX } else { r });
        cvlr_assert!(x.wrapping_mul(y) == r);
    }

    pub fn spec_div_forms() {
        let (a, b) = Self::pair();
        cvlr_assume!(b != 0);
        let (x, y) = (Self::value(a), Self::value(b));
        let (r, overflow) = x.overflowing_div(y);
        cvlr_assert!(x.checked_div(y) == if overflow { None } else { Some(r) });
        cvlr_assert!(x.saturating_div(y) == if overflow { ExactFixedU128::MAX } else { r });
        cvlr_assert!(x.wrapping_div(y) == r);
    }

    pub fn spec_checked_div_by_zero() {
        let a: u128 = nondet();
        cvlr_assert!(Self::value(a).checked_div(ExactFixedU128::ZERO).is_none());
    }

    /// `*` returns the product where it fits.
    pub fn spec_mul_operator() {
        let (a, b) = Self::pair();
        let (x, y) = (Self::value(a), Self::value(b));
        let expected = x.checked_mul(y);
        cvlr_assume!(expected.is_some());
        cvlr_assert!(Some(x * y) == expected);
    }

    /// `*` panics where it does not, as `fixed`'s does with debug assertions.
    pub fn spec_mul_operator_overflow_panics() {
        let (a, b) = Self::pair();
        let (x, y) = (Self::value(a), Self::value(b));
        cvlr_assume!(x.checked_mul(y).is_none());
        let _ = x * y;
        cvlr_assert!(false);
    }

    pub fn spec_div_operator() {
        let (a, b) = Self::pair();
        let (x, y) = (Self::value(a), Self::value(b));
        let expected = x.checked_div(y);
        cvlr_assume!(expected.is_some());
        cvlr_assert!(Some(x / y) == expected);
    }

    pub fn spec_div_operator_overflow_panics() {
        let (a, b) = Self::pair();
        cvlr_assume!(b != 0);
        let (x, y) = (Self::value(a), Self::value(b));
        cvlr_assume!(x.checked_div(y).is_none());
        let _ = x / y;
        cvlr_assert!(false);
    }

    pub fn spec_to_num() {
        let a: u128 = nondet();
        let q = native(a) / Self::scale();
        match Self::value(a).checked_to_num::<u64>() {
            Some(v) => cvlr_assert!(q.is_u64() && q == v),
            None => cvlr_assert!(!q.is_u64()),
        }
    }

    pub fn spec_from_num() {
        let v: u64 = nondet();
        clog!(v);
        let scaled = native_u64(v) * Self::scale();
        match ExactFixedU128::<Frac>::checked_from_num(v) {
            Some(x) => cvlr_assert!(scaled.is_u128() && native(x.to_bits()) == scaled),
            None => cvlr_assert!(!scaled.is_u128()),
        }
    }

    pub fn spec_cmp_u64() {
        let (a, v): (u128, u64) = (nondet(), nondet());
        clog!(a, v);
        let (x, n, scaled) = (Self::value(a), native(a), native_u64(v) * Self::scale());
        cvlr_assert!((x < v) == (n < scaled));
        cvlr_assert!((x == v) == (n == scaled));
        cvlr_assert!((v < x) == (scaled < n));
    }

    pub fn spec_cmp_u128() {
        let (a, v) = Self::pair();
        let (x, n, scaled) = (Self::value(a), native(a), native(v) * Self::scale());
        cvlr_assert!((x < v) == (n < scaled));
        cvlr_assert!((x == v) == (n == scaled));
        cvlr_assert!((v < x) == (scaled < n));
    }

    pub fn spec_rounding() {
        let a: u128 = nondet();
        clog!(a);
        let (x, n) = (Self::value(a), native(a));
        let frac = n - Self::scale() * (n / Self::scale());
        let floor = n - frac;
        cvlr_assert!(native(x.frac().to_bits()) == frac);
        cvlr_assert!(native(x.floor().to_bits()) == floor);
        cvlr_assert!(x.int() == x.floor());
        let ceil = if frac == NativeInt::from(0u64) {
            n
        } else {
            floor + Self::scale()
        };
        match x.checked_ceil() {
            Some(c) => cvlr_assert!(ceil.is_u128() && native(c.to_bits()) == ceil),
            None => cvlr_assert!(!ceil.is_u128()),
        }
        let round = if Frac::U32 > 0 && frac >= pow2(Frac::U32 - 1) {
            floor + Self::scale()
        } else {
            floor
        };
        match x.checked_round() {
            Some(r) => cvlr_assert!(round.is_u128() && native(r.to_bits()) == round),
            None => cvlr_assert!(!round.is_u128()),
        }
    }

    pub fn spec_ceil_overflow_panics() {
        let x = Self::value(nondet());
        cvlr_assume!(x.checked_ceil().is_none());
        let _ = x.ceil();
        cvlr_assert!(false);
    }

    pub fn spec_round_overflow_panics() {
        let x = Self::value(nondet());
        cvlr_assume!(x.checked_round().is_none());
        let _ = x.round();
        cvlr_assert!(false);
    }

    /// `From<u64>`, which exists where every `u64` fits.
    pub fn spec_from_u64()
    where
        ExactFixedU128<Frac>: From<u64>,
    {
        let v: u64 = nondet();
        let x = ExactFixedU128::<Frac>::from(v);
        cvlr_assert!(native(x.to_bits()) == native_u64(v) * Self::scale());
    }

    pub fn reach_mul_fits_large() {
        let (a, b) = Self::pair();
        let (r, overflow) = Self::value(a).overflowing_mul(Self::value(b));
        cvlr_satisfy!(!overflow && native(r.to_bits()) >= pow2(127));
    }

    pub fn reach_mul_overflows() {
        let (a, b) = Self::pair();
        let (r, overflow) = Self::value(a).overflowing_mul(Self::value(b));
        cvlr_satisfy!(overflow && r.to_bits() != 0);
    }

    pub fn reach_div_fits_large() {
        let (a, b) = Self::pair();
        cvlr_assume!(b != 0);
        let (r, overflow) = Self::value(a).overflowing_div(Self::value(b));
        cvlr_satisfy!(!overflow && native(r.to_bits()) >= pow2(127));
    }

    pub fn reach_div_overflows() {
        let (a, b) = Self::pair();
        cvlr_assume!(b != 0);
        let (r, overflow) = Self::value(a).overflowing_div(Self::value(b));
        cvlr_satisfy!(overflow && r.to_bits() != 0);
    }
}

macro_rules! instantiate {
    ($Frac:ty { $($rule:ident => $method:ident),* $(,)? }) => {
        $(
            #[rule]
            pub fn $rule() {
                Rules::<$Frac>::$method();
            }
        )*
    };
}

instantiate!(U60 {
    spec_mul_u60 => spec_mul,
    spec_to_num_u60 => spec_to_num,
    spec_from_num_u60 => spec_from_num,
    spec_from_u64_u60 => spec_from_u64,
    spec_cmp_u64_u60 => spec_cmp_u64,
    spec_cmp_u128_u60 => spec_cmp_u128,
    spec_rounding_u60 => spec_rounding,
    spec_ceil_overflow_panics_u60 => spec_ceil_overflow_panics,
    spec_round_overflow_panics_u60 => spec_round_overflow_panics,
    spec_div_u60 => spec_div,
    spec_mul_forms_u60 => spec_mul_forms,
    spec_div_forms_u60 => spec_div_forms,
    spec_checked_div_by_zero_u60 => spec_checked_div_by_zero,
    spec_mul_operator_u60 => spec_mul_operator,
    spec_mul_operator_overflow_panics_u60 => spec_mul_operator_overflow_panics,
    spec_div_operator_u60 => spec_div_operator,
    spec_div_operator_overflow_panics_u60 => spec_div_operator_overflow_panics,
    reach_mul_fits_large_u60 => reach_mul_fits_large,
    reach_mul_overflows_u60 => reach_mul_overflows,
    reach_div_fits_large_u60 => reach_div_fits_large,
    reach_div_overflows_u60 => reach_div_overflows,
});

// With no fraction, a quotient never exceeds its dividend and rounding never
// overflows.
instantiate!(U0 {
    spec_mul_u0 => spec_mul,
    spec_to_num_u0 => spec_to_num,
    spec_from_num_u0 => spec_from_num,
    spec_from_u64_u0 => spec_from_u64,
    spec_cmp_u64_u0 => spec_cmp_u64,
    spec_cmp_u128_u0 => spec_cmp_u128,
    spec_rounding_u0 => spec_rounding,
    spec_div_u0 => spec_div,
    spec_mul_forms_u0 => spec_mul_forms,
    spec_div_forms_u0 => spec_div_forms,
    spec_checked_div_by_zero_u0 => spec_checked_div_by_zero,
    spec_mul_operator_u0 => spec_mul_operator,
    spec_mul_operator_overflow_panics_u0 => spec_mul_operator_overflow_panics,
    spec_div_operator_u0 => spec_div_operator,
    reach_mul_fits_large_u0 => reach_mul_fits_large,
    reach_mul_overflows_u0 => reach_mul_overflows,
    reach_div_fits_large_u0 => reach_div_fits_large,
});

// No `spec_from_num`: `from_num` is `fixed`'s own code, and at this width the
// Prover finds a spurious counterexample to it. No `From<u64>` either: not
// every `u64` fits.
instantiate!(U127 {
    spec_mul_u127 => spec_mul,
    spec_to_num_u127 => spec_to_num,
    spec_cmp_u64_u127 => spec_cmp_u64,
    spec_cmp_u128_u127 => spec_cmp_u128,
    spec_rounding_u127 => spec_rounding,
    spec_ceil_overflow_panics_u127 => spec_ceil_overflow_panics,
    spec_round_overflow_panics_u127 => spec_round_overflow_panics,
    spec_div_u127 => spec_div,
    spec_mul_forms_u127 => spec_mul_forms,
    spec_div_forms_u127 => spec_div_forms,
    spec_checked_div_by_zero_u127 => spec_checked_div_by_zero,
    spec_mul_operator_u127 => spec_mul_operator,
    spec_mul_operator_overflow_panics_u127 => spec_mul_operator_overflow_panics,
    spec_div_operator_u127 => spec_div_operator,
    spec_div_operator_overflow_panics_u127 => spec_div_operator_overflow_panics,
    reach_mul_fits_large_u127 => reach_mul_fits_large,
    reach_mul_overflows_u127 => reach_mul_overflows,
    reach_div_fits_large_u127 => reach_div_fits_large,
    reach_div_overflows_u127 => reach_div_overflows,
});
