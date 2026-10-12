#![no_std]
//! Nondeterministic values for the Certora Prover.
//!
//! A nondeterministic value is one the Prover chooses freely, so a rule is
//! checked for every possible choice. Draw one with [`nondet()`], constrain it
//! with [`nondet_with`] or `cvlr_assume!`, and make your own types
//! nondeterministic by implementing [`Nondet`] or deriving it with
//! `#[derive(Nondet)]`. With the `std` feature (on by default), [`havoc`]
//! allocates objects with arbitrary contents.

#[cfg(feature = "std")]
extern crate std;

mod core;
mod option;
mod scalars;

/// Heap objects with arbitrary contents, for types without a
/// [`Nondet`] implementation.
#[cfg(feature = "std")]
pub mod havoc;

pub use core::{nondet, nondet_with, Nondet};

pub use option::nondet_option;
pub use scalars::{
    cvlr_nondet_small_i128, cvlr_nondet_small_u128, cvlr_nondet_u32, cvlr_nondet_u64,
};
