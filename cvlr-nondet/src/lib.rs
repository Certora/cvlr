#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod core;
mod option;
mod scalars;

#[cfg(feature = "std")]
pub mod havoc;

pub use core::{Nondet, nondet, nondet_with};

pub use scalars::{cvlr_nondet_small_u128, cvlr_nondet_small_i128}; 
pub use option::nondet_option;

#[cfg(feature = "derive")]
pub use derive_nondet::*;
