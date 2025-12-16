//! Specification language for CVL (Certora Verification Language) in Rust.
//!
//! This module provides a framework for writing specifications with preconditions
//! (requires) and postconditions (ensures) that can be used for formal verification.
//!
//! # Core Concepts
//!
//! ## Boolean Expressions
//!
//! The [`CvlrBoolExpr`] trait represents boolean expressions that can be:
//! - Evaluated to a boolean value
//! - Asserted (checked for correctness)
//! - Assumed (taken as preconditions)
//!
//! ## Composing Expressions
//!
//! Boolean expressions can be composed using:
//! - [`cvlr_and`] - Logical AND
//! - [`cvlr_impl`] - Logical implication (A → B)
//! - [`CvlrTrue`] - Constant true expression
//!
//! ## State Pairs
//!
//! [`StatePair`] represents a pair of contexts (pre-state and post-state) that
//! allows expressing postconditions that compare states before and after operations.
//!
//! ## Specifications
//!
//! The [`CvlrSpec`] trait represents a complete specification with:
//! - Preconditions (requires) - conditions that must hold before an operation
//! - Postconditions (ensures) - conditions that must hold after an operation
//!
//! Use [`cvlr_spec`] to create a specification from requires and ensures clauses,
//! or [`cvlr_invar_spec`] for specifications with invariants.
//!
//! ## Lemmas
//!
//! The [`CvlrLemma`](spec::CvlrLemma) trait represents a lemma: a logical statement where if the
//! preconditions (requires) hold, then the postconditions (ensures) must also hold.
//! Use [`cvlr_lemma!`] to define lemmas, or [`cvlr_predicate!`] to create anonymous
//! predicates for use in lemmas.
//!
//! # Examples
//!
//! ```ignore
//! use cvlr_spec::{cvlr_spec, CvlrTrue};
//!
//! struct Counter {
//!     value: i32,
//! }
//!
//! // Define a simple spec - CvlrTrue works for both requires and ensures
//! let spec = cvlr_spec(CvlrTrue, CvlrTrue);
//! ```

mod bool_expr;
mod combinators;
mod macros;
pub mod spec;
mod state_pair;

#[doc(hidden)]
pub mod __macro_support {
    pub use cvlr_macros::*;
}

// Re-export core types and traits
pub use bool_expr::{CvlrBoolExpr, CvlrTrue};
pub use combinators::{
    cvlr_and, cvlr_and_pair, cvlr_impl, cvlr_impl_pair, CvlrAnd, CvlrAndStatePair, CvlrImpl,
    CvlrImplStatePair,
};
pub use spec::{cvlr_invar_spec, cvlr_spec, CvlrInvarSpec, CvlrPropImpl, CvlrSpec};
pub use state_pair::StatePair;
