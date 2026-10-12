#![no_std]
//! Certora Verification Language for Rust (CVLR): write specifications for the
//! Certora Prover in Rust.
//!
//! A specification is a set of *rules*: ordinary Rust functions marked with
//! `#[rule]` that call the code under verification on nondeterministic inputs
//! and state what must hold. The Prover checks each rule for every possible
//! value of those inputs.
//!
//! ```ignore
//! use cvlr::prelude::*;
//!
//! #[rule]
//! fn withdraw_never_increases_balance() {
//!     let balance: u64 = nondet();
//!     let amount: u64 = nondet();
//!     cvlr_assume!(amount <= balance);
//!     let new_balance = withdraw(balance, amount);
//!     cvlr_assert_le!(new_balance, balance);
//! }
//! ```
//!
//! `use cvlr::prelude::*` (or `use cvlr::*`) brings in everything commonly
//! needed. Solana-specific helpers (nondeterministic accounts, account logging,
//! CPI models) are in the separate `cvlr-solana` family of crates.
//!
//! # Where things are
//!
//! | task | items | module |
//! | --- | --- | --- |
//! | define a rule | `#[rule]` | [`macros`] |
//! | check, assume, reach | `cvlr_assert!`, `cvlr_assume!`, `cvlr_satisfy!`, `cvlr_assert_eq!`/`_le!`/..., `cvlr_assume_eq!`/..., `cvlr_assert_*_if!` | [`asserts`] |
//! | check several conditions, DSL | `cvlr_assert_that!`, `cvlr_assert_all!`, `cvlr_assume_that!`, `cvlr_assume_all!` | [`macros`] |
//! | arbitrary inputs | `nondet()`, `nondet_with`, `nondet_option`, `#[derive(Nondet)]`, `havoc::alloc_ref_havoced` | [`nondet`](mod@nondet), [`derive`](mod@derive) |
//! | readable counterexamples | `clog!`, `CvlrLog`, `#[derive(CvlrLog)]`, `CvlrLogger` | [`log`], [`derive`](mod@derive) |
//! | exact arithmetic | `NativeInt` (256-bit), `is_u64` and friends | [`mathint`] |
//! | fixed-point / decimal values | `NativeFixedU64`, `NativeFixedU128`, `NativeDecimal` | [`fixed`], [`decimal`] |
//! | reusable pre/postconditions | `#[predicate]`, `cvlr_def_predicate!`, `cvlr_spec!`, `cvlr_invar_spec!`, `cvlr_rules!`, `cvlr_lemma!` | [`spec`], [`macros`] |
//! | adapt the code under verification | `#[mock_fn]`, `#[hook_on_entry]`, `#[hook_on_exit]`, `#[early_panic]` | prelude |
//!
//! # Features
//!
//! * `rt` — run-time implementations of the Prover builtins, so specs and
//!   CVLR itself can be compiled and run as ordinary tests (nondeterministic
//!   values become fixed defaults such as `0`). Not for verification builds.
//! * `vacuity` — makes the check `#[rule]` appends to every rule a
//!   `cvlr_assert!(false)` (see `cvlr_vacuity_check!`).
//! * `no-loc` — replaces recorded source locations with placeholders, for
//!   stable macro-expansion snapshots.

/// `u128` comparison and ceiling division through Prover builtins that
/// current Prover releases no longer recognize; use the ordinary operators.
pub mod u128_arith;

/// Assertions, assumptions and reachability checks: `cvlr_assert!`,
/// `cvlr_assume!`, `cvlr_satisfy!` and their comparison forms
/// (`cvlr_assert_eq!`, `cvlr_assume_le!`, `cvlr_assert_lt_if!`, ...).
/// Re-exports [`cvlr_asserts`](mod@cvlr_asserts).
pub mod asserts {
    pub use cvlr_asserts::*;
}

/// `NativeInt`, the Prover's 256-bit integer, for exact arithmetic in specs,
/// and the `is_u*` range checks. Re-exports [`cvlr_mathint`](mod@cvlr_mathint).
pub mod mathint {
    pub use cvlr_mathint::*;
}

/// Nondeterministic values: `nondet()`, `nondet_with`, `nondet_option`, the
/// `Nondet` trait, and heap allocation with arbitrary contents (`havoc`).
/// Re-exports [`cvlr_nondet`](mod@cvlr_nondet).
pub mod nondet {
    pub use cvlr_nondet::*;
}

/// Logging to the counterexample call trace: `clog!` / `cvlr_log!`, the
/// `CvlrLog` trait and `CvlrLogger`. Re-exports [`cvlr_log`](mod@cvlr_log).
pub mod log {
    pub use cvlr_log::*;
}

/// Procedural macros: `#[rule]`, `#[mock_fn]`, `#[cvlr_predicate]`, the
/// condition DSL (`cvlr_assert_that!`, `cvlr_assert_all!`, `cvlr_assume_that!`,
/// `cvlr_eval_that!`, ...) and `cvlr_rule_for_spec!`. Re-exports
/// [`cvlr_macros`](mod@cvlr_macros).
pub mod macros {
    pub use cvlr_macros::*;
}

/// `#[derive(Nondet)]` and `#[derive(CvlrLog)]`. Re-exports
/// [`cvlr_derive`](mod@cvlr_derive).
pub mod derive {
    pub use cvlr_derive::*;
}

/// Reusable specifications: formulas and predicates (`CvlrFormula`,
/// `cvlr_def_predicate!`, `cvlr_and!`, `cvlr_implies!`), pre/postcondition
/// specs (`cvlr_spec!`, `cvlr_invar_spec!`), rule generation (`cvlr_rules!`)
/// and lemmas (`cvlr_lemma!`). Re-exports [`cvlr_spec`](mod@cvlr_spec).
pub mod spec {
    pub use cvlr_spec::*;
}

/// Binary fixed-point numbers backed by `NativeInt`: `NativeFixedU64` and
/// `NativeFixedU128`. Re-exports [`cvlr_fixed`](mod@cvlr_fixed).
pub mod fixed {
    pub use cvlr_fixed::*;
}

/// Decimal numbers with a fixed number of fractional digits, backed by
/// `NativeInt`: `NativeDecimal`. Re-exports [`cvlr_decimal`](mod@cvlr_decimal).
pub mod decimal {
    pub use cvlr_decimal::*;
}

/// Everything commonly needed in a spec: `use cvlr::prelude::*;`.
///
/// Contains all of [`asserts`] and [`spec`]; `clog!`; `nondet` (also as
/// `cvlr_nondet`); the attributes `rule` (also `cvlr_rule`), `mock_fn`,
/// `early_panic` (also `cvlr_early_panic`), `hook_on_entry` and
/// `hook_on_exit` (also `cvlr_hook_on_entry` / `cvlr_hook_on_exit`); the
/// condition DSL macros; and the `Nondet` and `CvlrLog` derives. The crate
/// root re-exports the prelude, so `use cvlr::*;` works as well.
pub mod prelude {
    pub use super::asserts::*;

    pub use super::log::cvlr_log as clog;
    pub use super::nondet::nondet;
    pub use super::nondet::nondet as cvlr_nondet;

    pub use __macro_support::rule as cvlr_rule;
    pub use cvlr_early_panic::early_panic as cvlr_early_panic;
    pub use cvlr_hook::cvlr_hook_on_entry;
    pub use cvlr_hook::cvlr_hook_on_exit;

    pub use __macro_support::mock_fn;
    pub use __macro_support::rule;
    pub use cvlr_early_panic::early_panic;
    pub use cvlr_hook::cvlr_hook_on_entry as hook_on_entry;
    pub use cvlr_hook::cvlr_hook_on_exit as hook_on_exit;

    pub use super::macros::{
        cvlr_assert_all, cvlr_assert_that, cvlr_assume_all, cvlr_assume_that, cvlr_eval_all,
        cvlr_eval_that,
    };

    pub use super::derive::{CvlrLog, Nondet};

    pub use super::spec::*;
}

pub use prelude::*;

pub use crate::mathint::{is_u128, is_u16, is_u32, is_u64, is_u8};
pub use macros::cvlr_pif as pif;
pub use macros::cvlr_predicate as predicate;
