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

pub mod u128_arith;

pub mod asserts {
    pub use cvlr_asserts::*;
}

pub mod mathint {
    pub use cvlr_mathint::*;
}

pub mod nondet {
    pub use cvlr_nondet::*;
}

pub mod log {
    pub use cvlr_log::*;
}

pub mod macros {
    pub use cvlr_macros::*;
}

pub mod derive {
    pub use cvlr_derive::*;
}

pub mod spec {
    pub use cvlr_spec::*;
}

pub mod fixed {
    pub use cvlr_fixed::*;
}

pub mod decimal {
    pub use cvlr_decimal::*;
}

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
