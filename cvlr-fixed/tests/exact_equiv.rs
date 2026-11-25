//! `ExactFixedU128<Frac>` against `fixed::FixedU128<Frac>` itself, on the
//! host.
//!
//! Under `rt` every operation is `fixed`'s own, so these tests check the
//! wiring: each method reaches the wide type's counterpart, the forms of
//! multiplication and division derive from `overflowing_*` as `fixed`'s do,
//! and the operators panic exactly where `fixed`'s do in this (debug) build.
//! The native-integer paths are checked by the Prover rules in
//! `fixtures/exact_prover`.

use std::panic::{catch_unwind, AssertUnwindSafe};

use cvlr_fixed::exact::ExactFixedU128;
use fixed::types::extra::{LeEqU128, Unsigned, U0, U127, U60};
use fixed::FixedU128;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A u128 whose magnitude is spread over every bit width.
    fn spread(&mut self) -> u128 {
        let width = self.next() % 129;
        let v = (u128::from(self.next()) << 64) | u128::from(self.next());
        if width == 0 {
            0
        } else {
            v >> (128 - width)
        }
    }
}

const CASES: usize = 20_000;

fn panics<T>(f: impl FnOnce() -> T) -> Option<T> {
    catch_unwind(AssertUnwindSafe(f)).ok()
}

fn bits<Frac>(v: Option<ExactFixedU128<Frac>>) -> Option<u128>
where
    Frac: LeEqU128,
{
    v.map(ExactFixedU128::to_bits)
}

fn wide_bits<Frac>(v: Option<FixedU128<Frac>>) -> Option<u128>
where
    Frac: LeEqU128,
{
    v.map(FixedU128::to_bits)
}

fn agree<Frac: LeEqU128>(seed: u64) {
    let mut rng = Rng(seed);
    for _ in 0..CASES {
        let (a, b) = (rng.spread(), rng.spread());
        let (x, y) = (
            ExactFixedU128::<Frac>::from_bits(a),
            ExactFixedU128::<Frac>::from_bits(b),
        );
        let (w, v) = (FixedU128::<Frac>::from_bits(a), FixedU128::<Frac>::from_bits(b));

        let (r, o) = x.overflowing_mul(y);
        let (wr, wo) = w.overflowing_mul(v);
        assert_eq!((r.to_bits(), o), (wr.to_bits(), wo), "overflowing_mul {a} {b}");
        assert_eq!(bits(x.checked_mul(y)), wide_bits(w.checked_mul(v)));
        assert_eq!(x.saturating_mul(y).to_bits(), w.saturating_mul(v).to_bits());
        assert_eq!(x.wrapping_mul(y).to_bits(), w.wrapping_mul(v).to_bits());
        assert_eq!(
            panics(|| x.unwrapped_mul(y).to_bits()),
            panics(|| w.unwrapped_mul(v).to_bits())
        );
        assert_eq!(panics(|| (x * y).to_bits()), panics(|| (w * v).to_bits()), "* {a} {b}");

        assert_eq!(bits(x.checked_div(y)), wide_bits(w.checked_div(v)), "checked_div {a} {b}");
        assert_eq!(
            panics(|| x.overflowing_div(y)).map(|(r, o)| (r.to_bits(), o)),
            panics(|| w.overflowing_div(v)).map(|(r, o)| (r.to_bits(), o))
        );
        assert_eq!(
            panics(|| x.saturating_div(y).to_bits()),
            panics(|| w.saturating_div(v).to_bits())
        );
        assert_eq!(
            panics(|| x.wrapping_div(y).to_bits()),
            panics(|| w.wrapping_div(v).to_bits())
        );
        assert_eq!(
            panics(|| x.unwrapped_div(y).to_bits()),
            panics(|| w.unwrapped_div(v).to_bits())
        );
        assert_eq!(panics(|| (x / y).to_bits()), panics(|| (w / v).to_bits()), "/ {a} {b}");

        assert_eq!(bits(x.checked_add(y)), wide_bits(w.checked_add(v)));
        assert_eq!(bits(x.checked_sub(y)), wide_bits(w.checked_sub(v)));
        assert_eq!(panics(|| (x + y).to_bits()), panics(|| (w + v).to_bits()));
        assert_eq!(panics(|| (x - y).to_bits()), panics(|| (w - v).to_bits()));
        assert_eq!(x.saturating_sub(y).to_bits(), w.saturating_sub(v).to_bits());
        assert_eq!(x.cmp(&y), w.cmp(&v));

        let n = rng.spread() >> (rng.next() % 128);
        assert_eq!(bits(x.checked_mul_int(n)), wide_bits(w.checked_mul_int(n)));
        assert_eq!(bits(x.checked_div_int(n)), wide_bits(w.checked_div_int(n)));
        assert_eq!(panics(|| (x * n).to_bits()), panics(|| (w * n).to_bits()));
        assert_eq!(panics(|| (x / n).to_bits()), panics(|| (w / n).to_bits()));

        assert_eq!(bits(x.checked_ceil()), wide_bits(w.checked_ceil()));
        assert_eq!(bits(x.checked_round()), wide_bits(w.checked_round()));
        assert_eq!(x.floor().to_bits(), w.floor().to_bits());
        assert_eq!(x.to_num::<f64>(), w.to_num::<f64>());
        assert_eq!(x.checked_to_num::<u64>(), w.checked_to_num::<u64>());
    }
}

#[test]
fn agrees_with_fixed_u60() {
    agree::<U60>(0x9e37_79b9_7f4a_7c15);
}

#[test]
fn agrees_with_fixed_u0() {
    agree::<U0>(0xd1b5_4a32_d192_ed03);
}

#[test]
fn agrees_with_fixed_u127() {
    agree::<U127>(0x2545_f491_4f6c_dd1d);
}

#[test]
fn constants_and_literals_are_the_wide_types() {
    type F = ExactFixedU128<U60>;
    type W = FixedU128<U60>;
    assert_eq!(F::ONE.to_bits(), W::ONE.to_bits());
    assert_eq!(F::MAX.to_bits(), W::MAX.to_bits());
    assert_eq!(F::DELTA.to_bits(), W::DELTA.to_bits());
    assert_eq!(cvlr_fixed::exact_lit!(F, 0.000001).to_bits(), W::lit("0.000001").to_bits());
    assert_eq!(cvlr_fixed::exact_lit!(F, 1000000000000000).to_bits(), W::lit("1000000000000000").to_bits());
    const EPS: F = F::from_bits(1_000_000);
    assert_eq!(EPS.to_bits(), 1_000_000);
    const PCT: F = F::const_from_int(5).unwrapped_div_int(100);
    assert_eq!(PCT.to_bits(), W::const_from_int(5).unwrapped_div_int(100).to_bits());
}

/// `a * b` as a 256-bit `(hi, lo)`.
fn wide_mul(a: u128, b: u128) -> (u128, u128) {
    let (a1, a0) = (a >> 64, a & u128::from(u64::MAX));
    let (b1, b0) = (b >> 64, b & u128::from(u64::MAX));
    let (p00, p01, p10, p11) = (a0 * b0, a0 * b1, a1 * b0, a1 * b1);
    let mid = (p00 >> 64) + (p01 & u128::from(u64::MAX)) + (p10 & u128::from(u64::MAX));
    let lo = (p00 & u128::from(u64::MAX)) | (mid << 64);
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (hi, lo)
}

/// `(hi, lo) >> k` for `k <= 128`, as `(quotient mod 2^128, quotient >= 2^128)`.
fn shr(hi: u128, lo: u128, k: u32) -> (u128, bool) {
    if k == 0 {
        (lo, hi != 0)
    } else if k == 128 {
        (hi, false)
    } else {
        ((lo >> k) | (hi << (128 - k)), hi >> k != 0)
    }
}

/// `(hi, lo) / d` by shift and subtract, as `(quotient mod 2^128, quotient >= 2^128)`.
fn wide_div(hi: u128, lo: u128, d: u128) -> (u128, bool) {
    let (mut q_hi, mut q_lo, mut rem) = (0u128, 0u128, 0u128);
    for i in (0..256).rev() {
        let bit = if i >= 128 { (hi >> (i - 128)) & 1 } else { (lo >> i) & 1 };
        let carry = rem >> 127;
        rem = (rem << 1) | bit;
        if carry == 1 || rem >= d {
            rem = rem.wrapping_sub(d);
            if i >= 128 {
                q_hi |= 1 << (i - 128);
            } else {
                q_lo |= 1 << i;
            }
        }
    }
    (q_lo, q_hi != 0)
}

/// `fixed`'s own results are the floor quotients the Prover rules specify:
/// with the `spec_*` rules, this ties the native paths to `fixed` itself.
fn fixed_is_floor<Frac: LeEqU128>(seed: u64) {
    let mut rng = Rng(seed);
    let k = Frac::U32;
    for _ in 0..CASES {
        let (a, b) = (rng.spread(), rng.spread());
        let (w, v) = (FixedU128::<Frac>::from_bits(a), FixedU128::<Frac>::from_bits(b));
        let (hi, lo) = wide_mul(a, b);
        let (r, o) = w.overflowing_mul(v);
        assert_eq!((r.to_bits(), o), shr(hi, lo, k), "mul {a} {b}");
        if b != 0 {
            let (nhi, nlo) = if k == 0 { (0, a) } else if k == 128 { (a, 0) } else { (a >> (128 - k), a << k) };
            let (r, o) = w.overflowing_div(v);
            assert_eq!((r.to_bits(), o), wide_div(nhi, nlo, b), "div {a} {b}");
        }
    }
}

#[test]
fn fixed_is_floor_u60() {
    fixed_is_floor::<U60>(0x0123_4567_89ab_cdef);
}

#[test]
fn fixed_is_floor_u0() {
    fixed_is_floor::<U0>(0xfedc_ba98_7654_3210);
}

#[test]
fn fixed_is_floor_u127() {
    fixed_is_floor::<U127>(0x0f0f_0f0f_f0f0_f0f0);
}
