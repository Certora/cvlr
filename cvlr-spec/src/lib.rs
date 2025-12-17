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
//! - [`cvlr_implies`] - Logical implication (A → B)
//! - [`cvlr_true`] - Constant true expression
//!
//! ## State Pairs
//!
//! Postconditions use [`eval_with_states`](CvlrBoolExpr::eval_with_states) to evaluate
//! over both pre-state and post-state contexts, allowing you to express postconditions
//! that compare states before and after operations.
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
//! use cvlr_spec::{cvlr_spec, cvlr_true};
//!
//! struct Counter {
//!     value: i32,
//! }
//!
//! // Define a simple spec - cvlr_true uses eval_with_states for ensures
//! let spec = cvlr_spec(cvlr_true::<Counter>(), cvlr_true::<Counter>());
//! ```

mod bool_expr;
mod combinators;
mod macros;
pub mod spec;

#[doc(hidden)]
pub mod __macro_support {
    pub use cvlr_asserts::*;
    pub use cvlr_macros::*;
}

// Re-export core types and traits
pub use bool_expr::{cvlr_true, CvlrBoolExpr};
pub use combinators::{cvlr_and, cvlr_implies, CvlrAnd, CvlrImplies};
pub use spec::{cvlr_invar_spec, cvlr_spec, CvlrInvarSpec, CvlrPropImpl, CvlrSpec};
