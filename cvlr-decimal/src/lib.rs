#![no_std]
//! Decimal numbers for specs: [`NativeDecimal`], a [`cvlr_mathint::NativeInt`]
//! interpreted with a fixed number of fractional decimal digits, such as token
//! amounts with 6 or 9 decimals.

/// [`NativeDecimal`] and [`AsDecimal`], also exported at the crate root.
pub mod native_decimal;

pub use native_decimal::*;
