#![no_std]
//! Assertions and assumptions for the Certora Prover.
//!
//! The core macros are [`cvlr_assert!`], [`cvlr_assume!`] and
//! [`cvlr_satisfy!`]. [`asserts`] adds comparison forms that also log their
//! operands (`cvlr_assert_eq!`, `cvlr_assume_le!`, ...) and guarded forms
//! (`cvlr_assert_le_if!`, ...). Normally used through the `cvlr` crate, whose
//! paths some of these macros expand to.

/// Comparison and guarded forms of `cvlr_assert!` and `cvlr_assume!`, which
/// record the values of their operands in the counterexample.
///
/// The macros are also exported at the crate root.
pub mod asserts;
mod core;
/// Attaching source locations to assertions; see [`add_loc!`](crate::add_loc).
pub mod log;

pub use core::*;

/// Deprecated names from the former `cvt` crate: `cvt_assert!`,
/// `cvt_assume!`, `cvt_satisfy!` and `cvt_vacuity_check!`, aliases of the
/// `cvlr_*` macros of the same meaning.
#[deprecated = "Replaced by cvlr"]
pub mod cvt {
    pub use crate::cvlr_assert as cvt_assert;
    pub use crate::cvlr_assume as cvt_assume;
    pub use crate::cvlr_satisfy as cvt_satisfy;
    pub use crate::cvlr_vacuity_check as cvt_vacuity_check;
}
