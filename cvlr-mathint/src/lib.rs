#![no_std]
//! Mathematical Integers
//!
//! This crate provides various representations of integers that behave like
//! mathematical integers that do not overflow at machine-word widths (they are
//! 256 bits wide; see [NativeInt] for the exact semantics).
//!
//! [NativeInt] is a native symbolic integer of the Certora Prover: a 256-bit
//! integer whose operations the Prover interprets directly. Use it to state the
//! mathematically intended result of a computation and compare it with what the
//! code computes in machine arithmetic.
//!
//! Use feature `rt` to enable run-time under-approximation, for example, for testing.

/// [`NativeIntU64`](nativeint_u64::NativeIntU64), exported at the crate root
/// as [`NativeInt`], and the `is_u*` helpers.
pub mod nativeint_u64;

pub use nativeint_u64::NativeIntU64 as NativeInt;

pub use nativeint_u64::{is_u128, is_u16, is_u32, is_u64, is_u8};
