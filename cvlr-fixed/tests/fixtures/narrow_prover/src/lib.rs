//! Prover rules for `cvlr_fixed::narrow::NarrowFixedU128`, run on the code the
//! Prover analyses: the native-integer paths, which the host tests in
//! `narrow_equiv.rs` cannot reach.
//!
//! Each rule states an operation's result in native integers: the exact
//! result in narrow units, `None` beyond the wide range, and nothing (the
//! execution is assumed away) between the narrow and the wide range. The
//! reference is not `fixed` itself, whose 128-bit shifts the Prover models
//! only approximately. Run with `narrow_prover.conf`; see its header.

use cvlr::mathint::NativeInt;
use cvlr::prelude::*;
use cvlr_fixed::narrow::NarrowFixedU128;
use fixed::types::extra::{U14, U32, U60};
use fixed::{FixedU128, FixedU64};

type N = NarrowFixedU128<U60, U14>;

/// The wide type's largest value in narrow units: `2^(128 - 46) - 1`.
const WIDE_MAX_NARROW: u128 = (1 << 82) - 1;
const ONE: u64 = 1 << 14;

fn native(x: u64) -> NativeInt {
    NativeInt::from(x)
}

/// `u64::MAX`, exactly. As a literal, LLVM materializes it as the immediate
/// `-1`, which the Prover reads as `2^256 - 1`.
fn narrow_max() -> NativeInt {
    NativeInt::u64_max()
}

fn value(units: u64) -> N {
    N::from_narrow_fixed(FixedU64::from_bits(units))
}

fn units(x: N) -> u64 {
    x.to_narrow().to_bits()
}

fn pair() -> (u64, u64) {
    (nondet(), nondet())
}

/// A `checked_*` result against the exact result `e`.
fn check_checked(r: Option<N>, e: NativeInt) {
    if e > WIDE_MAX_NARROW {
        cvlr_assert!(r.is_none());
    } else {
        cvlr_assert!(r.is_some());
        if let Some(r) = r {
            cvlr_assert!(native(units(r)) == e);
        }
    }
}

/// A `saturating_*` result against the exact result `e`.
fn check_saturating(r: N, e: NativeInt) {
    if e > narrow_max() {
        cvlr_assert!(native(units(r)) == narrow_max());
    } else {
        cvlr_assert!(native(units(r)) == e);
    }
}

/// A wrapping result against the exact result `e`: `e` modulo the wide
/// range, which must then fit the narrow one.
fn check_wrapping(r: N, e: NativeInt) {
    let modulus = NativeInt::from(WIDE_MAX_NARROW) + 1u64;
    let wrapped = e - e / modulus * modulus;
    cvlr_assert!(native(units(r)) == wrapped);
}

#[rule]
pub fn narrow_add() {
    let (a, b) = pair();
    let e = native(a) + native(b);
    check_checked(value(a).checked_add(value(b)), e);
    check_saturating(value(a).saturating_add(value(b)), e);
    check_wrapping(value(a).wrapping_add(value(b)), e);
}

#[rule]
pub fn narrow_sub() {
    let (a, b) = pair();
    let r = value(a).checked_sub(value(b));
    if a < b {
        cvlr_assert!(r.is_none());
        cvlr_assert!(units(value(a).saturating_sub(value(b))) == 0);
    } else {
        cvlr_assert!(r.is_some() && units(r.unwrap()) == a - b);
        cvlr_assert!(units(value(a).saturating_sub(value(b))) == a - b);
    }
}

#[rule]
pub fn narrow_mul_checked() {
    let (a, b) = pair();
    let e = native(a).muldiv(native(b), native(ONE));
    check_checked(value(a).checked_mul(value(b)), e);
}

#[rule]
pub fn narrow_mul_saturating() {
    let (a, b) = pair();
    let e = native(a).muldiv(native(b), native(ONE));
    check_saturating(value(a).saturating_mul(value(b)), e);
}

#[rule]
pub fn narrow_mul_operator() {
    let (a, b) = pair();
    let e = native(a).muldiv(native(b), native(ONE));
    check_wrapping(value(a) * value(b), e);
}

#[rule]
pub fn narrow_div_checked() {
    let (a, b) = pair();
    let r = value(a).checked_div(value(b));
    if b == 0 {
        cvlr_assert!(r.is_none());
    } else {
        check_checked(r, native(a).muldiv(native(ONE), native(b)));
    }
}

#[rule]
pub fn narrow_div_operator() {
    let (a, b) = pair();
    cvlr_assume!(b != 0);
    check_wrapping(
        value(a) / value(b),
        native(a).muldiv(native(ONE), native(b)),
    );
}

#[rule]
pub fn narrow_mul_int() {
    let a: u64 = nondet();
    let n: u128 = nondet();
    let e = native(a) * NativeInt::from(n);
    check_checked(value(a).checked_mul_int(n), e);
    check_saturating(value(a).saturating_mul_int(n), e);
    check_wrapping(value(a).wrapping_mul_int(n), e);
}

#[rule]
pub fn narrow_div_int() {
    let a: u64 = nondet();
    let n: u128 = nondet();
    let r = value(a).checked_div_int(n);
    if n == 0 {
        cvlr_assert!(r.is_none());
    } else {
        cvlr_assert!(r.is_some());
        cvlr_assert!(native(units(r.unwrap())) == native(a) / NativeInt::from(n));
        cvlr_assert!(units(value(a) / n) == units(r.unwrap()));
    }
}

#[rule]
pub fn narrow_from_bits() {
    let bits: u128 = nondet();
    let r = units(N::from_bits(bits));
    let e = NativeInt::from(bits) / NativeInt::from(1u128 << 46);
    if e > narrow_max() {
        cvlr_assert!(native(r) == narrow_max());
    } else {
        cvlr_assert!(native(r) == e);
    }
}

#[rule]
pub fn narrow_to_bits() {
    let a: u64 = nondet();
    let bits = value(a).to_bits();
    cvlr_assert!(NativeInt::from(bits) == native(a) * NativeInt::from(1u128 << 46));
    cvlr_assert!(units(N::from_bits(bits)) == a);
    cvlr_assert!(N::is_representable_bits(bits));
}

#[rule]
pub fn narrow_ceil_round() {
    let a: u64 = nondet();
    let frac = a % ONE;
    let int = a - frac;
    let ceil = value(a).checked_ceil();
    let round = value(a).checked_round();
    // Neither can leave the wide range, so both are `Some` or assumed away.
    cvlr_assert!(ceil.is_some() && round.is_some());
    let up = native(int) + ONE;
    if frac == 0 {
        cvlr_assert!(units(ceil.unwrap()) == a);
    } else {
        cvlr_assert!(native(units(ceil.unwrap())) == up);
    }
    if frac < ONE / 2 {
        cvlr_assert!(units(round.unwrap()) == int);
    } else {
        cvlr_assert!(native(units(round.unwrap())) == up);
    }
}

#[rule]
pub fn narrow_from_int() {
    let v: u64 = nondet();
    let e = native(v) * ONE;
    // The wide type holds every u64, so none of these fail.
    let checked = N::checked_from_num(v);
    cvlr_assert!(checked.is_some());
    cvlr_assert!(native(units(checked.unwrap())) == e);
    cvlr_assert!(native(units(N::from_num(v))) == e);
    cvlr_assert!(native(units(N::from(v))) == e);
    check_saturating(N::saturating_from_num(v), e);
}

#[rule]
pub fn narrow_from_fixed() {
    let x: u64 = nondet();
    let r = N::checked_from_num(FixedU64::<U32>::from_bits(x));
    // `x / 2^32` in narrow units is `x / 2^18`, rounded down.
    cvlr_assert!(r.is_some());
    cvlr_assert!(units(r.unwrap()) == x >> 18);
}

#[rule]
pub fn narrow_to_num_and_compare() {
    let a: u64 = nondet();
    let v: u64 = nondet();
    cvlr_assert!(native(value(a).to_num::<u64>()) == native(a) / ONE);
    let scaled = native(v) * ONE;
    cvlr_assert!((value(a) < v) == (native(a) < scaled));
    cvlr_assert!((value(a) == v) == (native(a) == scaled));
}

#[rule]
pub fn narrow_to_wide() {
    let a: u64 = nondet();
    let wide: FixedU128<U60> = value(a).to_wide();
    cvlr_assert!(NativeInt::from(wide.to_bits()) == native(a) * NativeInt::from(1u128 << 46));
}
