#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod core;
pub mod option;
pub mod scalars;

#[cfg(feature = "std")]
pub mod havoc;

pub use core::{nondet, nondet_with, Nondet};

pub use option::nondet_option;
pub use scalars::{cvlr_nondet_small_i128, cvlr_nondet_small_u128};
