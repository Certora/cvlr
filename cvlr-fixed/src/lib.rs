#![no_std]
//! Binary fixed-point numbers for specs: [`NativeFixedU64`] and
//! [`NativeFixedU128`], backed by [`cvlr_mathint::NativeInt`], for modeling
//! fixed-point types of the code under verification; and, with the `fixed`
//! feature, two stand-ins for a program's 128-bit [`fixed`](https://docs.rs/fixed)
//! type: [`exact::ExactFixedU128`], which computes exactly what it computes
//! with its multiplication and division on native integers, and
//! [`narrow::NarrowFixedU128`], which computes at 64 bits.

/// [`NativeFixedU64`] and [`NativeFixedU128`], also exported at the crate
/// root.
pub mod native_fixed;

#[cfg(feature = "fixed")]
pub mod exact;

#[cfg(feature = "fixed")]
pub mod narrow;

pub use native_fixed::*;
