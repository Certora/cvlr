#![no_std]
//! Binary fixed-point numbers for specs: [`NativeFixedU64`] and
//! [`NativeFixedU128`], backed by [`cvlr_mathint::NativeInt`], for modeling
//! fixed-point types of the code under verification.

/// [`NativeFixedU64`] and [`NativeFixedU128`], also exported at the crate
/// root.
pub mod native_fixed;

pub use native_fixed::*;
