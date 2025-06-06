#![no_std]

/// ! Mathematical Integers
///
/// This crate provides various representations of integers that behave like
/// mathematical integers that do not overflow. ///
///
/// [NativeInt] is a native symbolic integer of the Certora Prover. Typically,
/// this is a 256 bit integer.
///
/// Use feature `rt` to enable run-time under-approximation, for example, for testing.
pub mod nativeint_u64;

pub use nativeint_u64::NativeIntU64 as NativeInt;

pub use nativeint_u64::{is_u128, is_u16, is_u32, is_u64, is_u8};
