#![no_std]
//! Logging to the Certora Prover's counterexample call trace.
//!
//! Log values with [`cvlr_log!`] (`clog!` in the `cvlr` prelude); make types
//! loggable by implementing [`CvlrLog`] or deriving it with
//! `#[derive(CvlrLog)]`. [`CvlrLogger`] documents the display rules, and its
//! methods, also available as free functions, are the underlying operations.

mod core;
mod cvt_macros;
mod log;

pub use crate::core::*;
pub use crate::log::*;
