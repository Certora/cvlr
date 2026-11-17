#![no_std]
//! Binary fixed-point numbers for specs: [`NativeFixedU64`] and
//! [`NativeFixedU128`], backed by [`cvlr_mathint::NativeInt`], for modeling
//! fixed-point types of the code under verification; and, with the `fixed`
//! feature, [`narrow::NarrowFixedU128`], a stand-in that replaces a program's
//! 128-bit [`fixed`](https://docs.rs/fixed) type with a 64-bit one.

/// [`NativeFixedU64`] and [`NativeFixedU128`], also exported at the crate
/// root.
pub mod native_fixed;

#[cfg(feature = "fixed")]
pub mod narrow;

pub use native_fixed::*;
