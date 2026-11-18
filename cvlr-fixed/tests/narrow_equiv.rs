//! `NarrowFixedU128<Frac, NFrac>` against `fixed::FixedU128<Frac>` itself.
//!
//! On values of the narrow grid, every operation must return the wide result
//! rounded down onto the grid, and `None` exactly where the wide operation
//! returns `None`. Where the wide result fits the wide type but not the narrow
//! one the operation assumes the case away; under `rt` an assumption aborts
//! rather than unwinds, so those cases are skipped.

use std::panic::{catch_unwind, AssertUnwindSafe};

use cvlr_fixed::narrow::NarrowFixedU128;
use fixed::traits::FromFixed;
use fixed::types::extra::{LeEqU128, LeEqU64, U0, U127, U14, U20, U32, U60, U63, U64};
use fixed::{FixedU128, FixedU64};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A u64 whose magnitude is spread over every bit width, so that small
    /// values, values near one and values near the top of the range all occur.
    fn spread(&mut self) -> u64 {
        let width = self.next() % 65;
        if width == 0 {
            0
        } else {
            self.next() >> (64 - width)
        }
    }
}

const CASES: usize = 20_000;

fn panics<T>(f: impl FnOnce() -> T) -> bool {
    catch_unwind(AssertUnwindSafe(f)).is_err()
}

struct Check<Frac, NFrac>(core::marker::PhantomData<(Frac, NFrac)>);

impl<Frac: LeEqU128, NFrac: LeEqU64 + LeEqU128> Check<Frac, NFrac> {
    const SHIFT: u32 = Frac::U32 - NFrac::U32;

    fn narrow(bits: u64) -> NarrowFixedU128<Frac, NFrac> {
        NarrowFixedU128::from_bits_exact((bits as u128) << Self::SHIFT)
    }

    fn wide(n: NarrowFixedU128<Frac, NFrac>) -> FixedU128<Frac> {
        FixedU128::from_bits(n.to_bits())
    }

    /// The wide result `w`, rounded down onto the narrow grid, or `None` when
    /// it lies above the narrow range.
    fn truncate(w: FixedU128<Frac>) -> Option<u128> {
        let narrow = w.to_bits() >> Self::SHIFT;
        (narrow <= u64::MAX as u128).then_some(narrow << Self::SHIFT)
    }

    /// Compares an operation that can fail on the wide type.
    fn option(
        what: &str,
        a: u64,
        b: u128,
        wide: Option<FixedU128<Frac>>,
        narrow: impl FnOnce() -> Option<NarrowFixedU128<Frac, NFrac>>,
    ) {
        match wide {
            None => assert_eq!(
                narrow().map(|n| n.to_bits()),
                None,
                "{what}({a:#x}, {b:#x}): the wide operation fails"
            ),
            Some(w) => Self::value(what, a, b, w, || {
                narrow().expect("narrow failed where wide did not")
            }),
        }
    }

    /// Compares an operation whose wide result is `w`.
    fn value(
        what: &str,
        a: u64,
        b: u128,
        w: FixedU128<Frac>,
        narrow: impl FnOnce() -> NarrowFixedU128<Frac, NFrac>,
    ) {
        match Self::truncate(w) {
            Some(expected) => assert_eq!(
                narrow().to_bits(),
                expected,
                "{what}({a:#x}, {b:#x}): wide {w}"
            ),
            // Above the narrow range the operation assumes the case away, and
            // under `rt` an assumption aborts the process rather than unwinding.
            None => {}
        }
    }

    fn arithmetic(seed: u64) {
        let mut rng = Rng(seed);
        for _ in 0..CASES {
            let (a, b) = (rng.spread(), rng.spread());
            let (na, nb) = (Self::narrow(a), Self::narrow(b));
            let (wa, wb) = (Self::wide(na), Self::wide(nb));
            let bb = b as u128;

            Self::option("checked_add", a, bb, wa.checked_add(wb), || {
                na.checked_add(nb)
            });
            Self::option("checked_sub", a, bb, wa.checked_sub(wb), || {
                na.checked_sub(nb)
            });
            Self::option("checked_mul", a, bb, wa.checked_mul(wb), || {
                na.checked_mul(nb)
            });
            Self::option("checked_div", a, bb, wa.checked_div(wb), || {
                na.checked_div(nb)
            });

            let int = rng.spread() as u128 >> (rng.next() % 64);
            Self::option("checked_mul_int", a, int, wa.checked_mul_int(int), || {
                na.checked_mul_int(int)
            });
            Self::option("checked_div_int", a, int, wa.checked_div_int(int), || {
                na.checked_div_int(int)
            });

            Self::value("wrapping_add", a, bb, wa.wrapping_add(wb), || {
                na.wrapping_add(nb)
            });
            Self::value("wrapping_sub", a, bb, wa.wrapping_sub(wb), || {
                na.wrapping_sub(nb)
            });
            Self::value("wrapping_mul", a, bb, wa.wrapping_mul(wb), || {
                na.wrapping_mul(nb)
            });
            Self::value("wrapping_mul_int", a, int, wa.wrapping_mul_int(int), || {
                na.wrapping_mul_int(int)
            });
            Self::value("mul", a, bb, wa.wrapping_mul(wb), || na * nb);
            if b != 0 {
                Self::value("wrapping_div", a, bb, wa.wrapping_div(wb), || {
                    na.wrapping_div(nb)
                });
                Self::value("div", a, bb, wa.wrapping_div(wb), || na / nb);
            }
            if int != 0 {
                Self::value("div_int", a, int, wa / int, || na / int);
                Self::value(
                    "unwrapped_div_int",
                    a,
                    int,
                    wa.checked_div_int(int).unwrap(),
                    || na.unwrapped_div_int(int),
                );
            }

            match wa.checked_add(wb) {
                Some(w) => Self::value("add", a, bb, w, || na + nb),
                None => assert!(panics(|| na + nb), "add({a:#x}, {b:#x}) must panic"),
            }
            match wa.checked_sub(wb) {
                Some(w) => Self::value("sub", a, bb, w, || na - nb),
                None => assert!(panics(|| na - nb), "sub({a:#x}, {b:#x}) must panic"),
            }
            match wa.checked_mul_int(int) {
                Some(w) => Self::value("mul_int", a, int, w, || na * int),
                None => assert!(panics(|| na * int), "mul_int({a:#x}, {int:#x}) must panic"),
            }

            assert_eq!(na.cmp(&nb), wa.cmp(&wb), "cmp({a:#x}, {b:#x})");
            assert_eq!(na.abs_diff(nb).to_bits(), wa.abs_diff(wb).to_bits());
            assert_eq!(
                na.saturating_sub(nb).to_bits(),
                wa.saturating_sub(wb).to_bits()
            );
            for (what, s, w) in [
                (
                    "saturating_add",
                    na.saturating_add(nb),
                    wa.saturating_add(wb),
                ),
                (
                    "saturating_mul",
                    na.saturating_mul(nb),
                    wa.saturating_mul(wb),
                ),
            ] {
                let expected =
                    Self::truncate(w).unwrap_or(NarrowFixedU128::<Frac, NFrac>::MAX.to_bits());
                assert_eq!(s.to_bits(), expected, "{what}({a:#x}, {b:#x})");
            }
        }
    }

    fn rounding(seed: u64) {
        let mut rng = Rng(seed);
        for _ in 0..CASES {
            let a = rng.spread();
            let na = Self::narrow(a);
            let wa = Self::wide(na);
            Self::value("int", a, 0, wa.int(), || na.int());
            Self::value("frac", a, 0, wa.frac(), || na.frac());
            Self::value("floor", a, 0, wa.floor(), || na.floor());
            Self::value("round_to_zero", a, 0, wa.round_to_zero(), || {
                na.round_to_zero()
            });
            Self::option("checked_ceil", a, 0, wa.checked_ceil(), || {
                na.checked_ceil()
            });
            Self::option("checked_round", a, 0, wa.checked_round(), || {
                na.checked_round()
            });
            assert_eq!(na.to_num::<u128>(), wa.to_num::<u128>());
            assert_eq!(na.checked_to_num::<u64>(), wa.checked_to_num::<u64>());
            assert_eq!(na.saturating_to_num::<u32>(), wa.saturating_to_num::<u32>());
            assert_eq!(na.is_zero(), wa == 0);
            assert_eq!(format!("{na}"), format!("{wa}"));

            let int = rng.spread() >> (rng.next() % 64);
            assert_eq!(na == int, wa == int, "{a:#x} == {int:#x}");
            assert_eq!(
                na.partial_cmp(&int),
                wa.partial_cmp(&int),
                "{a:#x} cmp {int:#x}"
            );
            assert_eq!(
                int.partial_cmp(&na),
                int.partial_cmp(&wa),
                "{int:#x} cmp {a:#x}"
            );
            let signed = int as i64;
            assert_eq!(na.partial_cmp(&signed), wa.partial_cmp(&signed));
            assert_eq!(
                na.partial_cmp(&(int as u128)),
                wa.partial_cmp(&(int as u128))
            );
        }
    }

    fn conversions(seed: u64) {
        let mut rng = Rng(seed);
        for _ in 0..CASES {
            let n = rng.spread();
            if let Some(w) = FixedU128::<Frac>::checked_from_num(n) {
                Self::value("from_num", n, 0, w, || NarrowFixedU128::from_num(n));
                Self::value("from", n, 0, w, || NarrowFixedU128::from(n));
            }
            Self::option(
                "checked_from_num",
                n,
                0,
                FixedU128::<Frac>::checked_from_num(n),
                || NarrowFixedU128::checked_from_num(n),
            );

            // From another fixed-point type, through every overflow mode.
            let other = FixedU64::<U32>::from_bits(rng.spread());
            Self::option(
                "checked_from_fixed",
                other.to_bits(),
                0,
                FixedU128::<Frac>::checked_from_fixed(other),
                || NarrowFixedU128::checked_from_fixed(other),
            );
            let (wide, wide_overflow) = FixedU128::<Frac>::overflowing_from_fixed(other);
            if !wide_overflow {
                Self::value("from_fixed", other.to_bits(), 0, wide, || {
                    NarrowFixedU128::from_fixed(other)
                });
            }
            let saturated = NarrowFixedU128::<Frac, NFrac>::saturating_from_num(other);
            match FixedU128::<Frac>::checked_from_fixed(other).and_then(Self::truncate) {
                Some(expected) => assert_eq!(saturated.to_bits(), expected),
                None => assert_eq!(saturated, NarrowFixedU128::MAX),
            }

            let bits = ((rng.next() as u128) << 64) | rng.next() as u128;
            let bits = bits >> (rng.next() % 128);
            let from_bits = NarrowFixedU128::<Frac, NFrac>::from_bits(bits);
            assert_eq!(
                NarrowFixedU128::<Frac, NFrac>::const_from_bits(bits),
                from_bits
            );
            match Self::truncate(FixedU128::from_bits(bits)) {
                Some(expected) => assert_eq!(from_bits.to_bits(), expected),
                None => assert_eq!(from_bits, NarrowFixedU128::MAX),
            }
            assert_eq!(
                NarrowFixedU128::<Frac, NFrac>::is_representable_bits(bits),
                from_bits.to_bits() == bits
            );
        }
    }
}

#[test]
fn constants() {
    type N = NarrowFixedU128<U60, U14>;
    type W = FixedU128<U60>;
    assert_eq!(N::ONE.to_bits(), W::ONE.to_bits());
    assert_eq!(N::ZERO.to_bits(), 0);
    assert_eq!(N::FRAC_NBITS, W::FRAC_NBITS);
    assert_eq!(N::INT_NBITS, W::INT_NBITS);
    assert_eq!(N::DELTA.to_bits(), 1 << 46);
    assert_eq!(N::MAX.to_bits(), (u64::MAX as u128) << 46);
    assert_eq!(
        N::const_from_int(10_000).to_bits(),
        W::const_from_int(10_000).to_bits()
    );
    assert_eq!(
        N::const_from_int(3).unwrapped_div_int(10_000).to_bits(),
        (W::const_from_int(3).unwrapped_div_int(10_000).to_bits() >> 46) << 46
    );
    assert_eq!(N::lit("0.75").to_bits(), W::lit("0.75").to_bits());
    assert_eq!(cvlr_fixed::narrow_lit!(N, 0.75), N::lit("0.75"));
    assert_eq!(
        cvlr_fixed::narrow_lit!(N, 10_000),
        N::const_from_int(10_000)
    );
    assert_eq!(
        "2.5".parse::<N>().unwrap().to_bits(),
        W::lit("2.5").to_bits()
    );
    assert_eq!(N::from_bits(1_000_000), N::ZERO);
    assert_eq!(N::from_bits(u128::MAX), N::MAX);
    assert!(!N::is_representable_bits(1));
}

#[test]
fn arithmetic_u60_u14() {
    Check::<U60, U14>::arithmetic(0x9e37_79b9_7f4a_7c15);
}

#[test]
fn arithmetic_u60_u20() {
    Check::<U60, U20>::arithmetic(0xd1b5_4a32_d192_ed03);
}

#[test]
fn arithmetic_same_scale() {
    Check::<U14, U14>::arithmetic(0x2545_f491_4f6c_dd1d);
}

#[test]
fn rounding_u60_u14() {
    Check::<U60, U14>::rounding(0x94d0_49bb_1331_11eb);
}

#[test]
fn rounding_same_scale() {
    Check::<U14, U14>::rounding(0xbf58_476d_1ce4_e5b9);
}

#[test]
fn conversions_u60_u14() {
    Check::<U60, U14>::conversions(0x6a09_e667_f3bc_c909);
}

#[test]
fn conversions_same_scale() {
    Check::<U14, U14>::conversions(0xbb67_ae85_84ca_a73b);
}

#[test]
fn edges_whole_word_shift() {
    Check::<U64, U0>::arithmetic(0x3c6e_f372_fe94_f82b);
    Check::<U64, U0>::rounding(0xa54f_f53a_5f1d_36f1);
    Check::<U64, U0>::conversions(0x510e_527f_ade6_82d1);
}

#[test]
fn edges_no_fraction() {
    Check::<U14, U0>::arithmetic(0x9b05_688c_2b3e_6c1f);
    Check::<U14, U0>::rounding(0x1f83_d9ab_fb41_bd6b);
    Check::<U14, U0>::conversions(0x5be0_cd19_137e_2179);
}

#[test]
fn edges_widest_narrow() {
    Check::<U127, U63>::arithmetic(0xcbbb_9d5d_c105_9ed8);
    Check::<U127, U63>::rounding(0x629a_292a_367c_d507);
    Check::<U127, U63>::conversions(0x9159_015a_3070_dd17);
}
