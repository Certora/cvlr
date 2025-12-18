//! Specification types and traits.

use crate::bool_expr::CvlrFormula;

/// A specification that defines preconditions (requires) and postconditions (ensures).
///
/// This trait represents a contract specification for operations. Implementations
/// should:
/// - Assume preconditions hold before an operation (via [`assume_requires`](CvlrSpec::assume_requires))
/// - Check that postconditions hold after an operation (via [`check_ensures`](CvlrSpec::check_ensures))
///
/// # Associated Types
///
/// * [`Context`](CvlrSpec::Context) - The context type representing the state
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{CvlrSpec, cvlr_true, cvlr_spec};
///
/// struct MyContext {
///     value: i32,
/// }
///
/// // Create a simple spec with requires and ensures
/// // cvlr_true works for any context type and uses eval_with_states for ensures
/// let spec = cvlr_spec(cvlr_true::<MyContext>(), cvlr_true::<MyContext>());
///
/// let ctx = MyContext { value: 5 };
/// spec.assume_requires(&ctx);
/// ```
pub trait CvlrSpec {
    type Context;
    /// Assumes that the preconditions (requires) hold for the given pre-state.
    ///
    /// This should be called before executing the operation to add the preconditions
    /// as assumptions that the verifier will take as true.
    ///
    /// # Arguments
    ///
    /// * `pre_state` - The state before the operation
    fn assume_requires(&self, pre_state: &Self::Context);

    /// Checks that the postconditions (ensures) hold for the given pre/post state pair.
    ///
    /// This should be called after executing the operation to verify that the
    /// postconditions are satisfied.
    ///
    /// # Arguments
    ///
    /// * `post` - The post-state (state after the operation)
    /// * `old` - The pre-state (state before the operation)
    fn check_ensures(&self, post: &Self::Context, old: &Self::Context);
}

/// An implementation of [`CvlrSpec`] that combines a precondition and postcondition.
///
/// This type stores a boolean expression for the precondition (requires) and
/// a boolean expression for the postcondition (ensures). The ensures expression
/// uses [`eval_with_states`](crate::CvlrFormula::eval_with_states) to evaluate
/// over both pre-state and post-state.
#[derive(Copy, Clone)]
pub struct CvlrPropImpl<Pre, Post>(Pre, Post);

impl<Pre, Post> CvlrSpec for CvlrPropImpl<Pre, Post>
where
    Pre: CvlrFormula,
    Post: CvlrFormula<Context = Pre::Context>,
{
    type Context = Pre::Context;

    fn assume_requires(&self, pre_state: &Self::Context) {
        self.0.assume(pre_state);
    }
    fn check_ensures(&self, post_state: &Self::Context, old: &Self::Context) {
        self.1.assert_with_states(post_state, old);
    }
}

/// Creates a specification from a precondition and postcondition.
///
/// This function combines a requires clause (precondition) and an ensures clause
/// (postcondition) into a [`CvlrSpec`] implementation.
///
/// # Arguments
///
/// * `requires` - A boolean expression over the context type representing the precondition
/// * `ensures` - A boolean expression over the context type that uses [`eval_with_states`](crate::CvlrFormula::eval_with_states)
///   to evaluate over both pre-state and post-state
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_spec, cvlr_predicate, cvlr_def_states_predicate};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Define a predicate for the ensures clause
/// cvlr_def_states_predicate! {
///     pred ValueIncreases([c, o]: Counter) {
///         c.value > o.value
///     }
/// }
///
/// // Define a spec: requires value >= 0, ensures value increases
/// let spec = cvlr_spec(
///     // requires - a predicate over Counter
///     cvlr_predicate! { |c: Counter| -> { c.value >= 0 } },
///     // ensures - a predicate that uses eval_with_states
///     ValueIncreases,
/// );
/// ```
pub fn cvlr_spec<Requires, Ensures>(
    requires: Requires,
    ensures: Ensures,
) -> impl CvlrSpec<Context = Requires::Context>
where
    Requires: CvlrFormula,
    Ensures: CvlrFormula<Context = Requires::Context>,
{
    CvlrPropImpl(requires, ensures)
}

/// A specification for invariants that must hold before and after operations.
///
/// This type represents a specification where:
/// - An assumption (additional precondition) is assumed before the operation
/// - An invariant must hold both before and after the operation
///
/// The invariant is assumed in the pre-state and asserted in the post-state.
#[derive(Copy, Clone)]
pub struct CvlrInvarSpec<A, B>(A, B);

impl<A, B> CvlrInvarSpec<A, B> {
    /// Returns a reference to the invariant expression.
    pub fn invariant(&self) -> &B {
        &self.1
    }

    /// Returns a reference to the assumption expression.
    pub fn assumption(&self) -> &A {
        &self.0
    }
}

impl<A, B> CvlrSpec for CvlrInvarSpec<A, B>
where
    A: CvlrFormula,
    B: CvlrFormula<Context = A::Context>,
{
    type Context = A::Context;
    fn assume_requires(&self, pre_state: &Self::Context) {
        self.0.assume(pre_state);
        self.1.assume(pre_state);
    }
    fn check_ensures(&self, post_state: &Self::Context, _: &Self::Context) {
        // -- invaraint is only over one-state so that it can be assumed in pre
        self.1.assert(post_state);
    }
}

/// Creates an invariant specification from an assumption and an invariant.
///
/// This function creates a specification where:
/// - The assumption is taken as an additional precondition
/// - The invariant must hold in both the pre-state (assumed) and post-state (asserted)
///
/// # Arguments
///
/// * `assumption` - A boolean expression representing an additional precondition
/// * `invariant` - A boolean expression representing an invariant that must hold before and after
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_invar_spec, CvlrFormula};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Create a spec with an assumption and an invariant
/// let spec = cvlr_invar_spec(
///     // assumption: value is even
///     |c: &Counter| c.value % 2 == 0,
///     // invariant: value is non-negative
///     |c: &Counter| c.value >= 0,
/// );
/// ```
pub fn cvlr_invar_spec<A, B>(assumption: A, invariant: B) -> CvlrInvarSpec<A, B>
where
    A: CvlrFormula,
    B: CvlrFormula<Context = A::Context>,
{
    CvlrInvarSpec(assumption, invariant)
}

/// A trait for defining lemmas with preconditions (requires) and postconditions (ensures).
///
/// A lemma is a logical statement that can be verified: if the preconditions hold,
/// then the postconditions must also hold. This trait provides a way to define
/// such lemmas and verify them using the CVLR verification framework.
///
/// # Associated Types
///
/// * [`Context`](CvlrLemma::Context) - The context type representing the state. Must implement [`Nondet`](cvlr_nondet::Nondet)
///   and [`CvlrLog`](cvlr_log::CvlrLog) to support verification.
///
/// # Methods
///
/// Implementations must provide:
/// - [`requires`](CvlrLemma::requires) - Returns a boolean expression representing the preconditions
/// - [`ensures`](CvlrLemma::ensures) - Returns a boolean expression representing the postconditions
///
/// The trait provides default implementations for:
/// - [`verify`](CvlrLemma::verify) - Verifies the lemma with a non-deterministic context
/// - [`verify_with_context`](CvlrLemma::verify_with_context) - Verifies the lemma with a specific context
/// - [`apply`](CvlrLemma::apply) - Applies the lemma (assumes requires, asserts ensures)
///
/// # Usage
///
/// Lemmas are typically created using the [`cvlr_lemma!`] macro:
///
/// ```ignore
/// extern crate cvlr;
/// use cvlr_spec::cvlr_lemma;
///
/// // Counter must implement Nondet and CvlrLog for lemma verification
/// #[derive(cvlr::derive::Nondet, cvlr::derive::CvlrLog)]
/// struct Counter {
///     value: i32,
/// }
///
/// cvlr_lemma! {
///     CounterPositiveLemma(c: Counter) {
///         requires -> {
///             c.value > 0
///         }
///         ensures -> {
///             c.value > 0
///         }
///     }
/// }
///
/// // Verify the lemma
/// let lemma = CounterPositiveLemma;
/// lemma.verify();
/// ```
///
/// # Verification
///
/// When verifying a lemma:
/// 1. The preconditions (requires) are assumed to hold
/// 2. The postconditions (ensures) are asserted to hold
///
/// If the postconditions don't hold when the preconditions are assumed,
/// the verification will fail.
pub trait CvlrLemma {
    type Context: cvlr_nondet::Nondet + cvlr_log::CvlrLog;

    /// Returns a boolean expression representing the preconditions (requires) of the lemma.
    ///
    /// This expression will be assumed to hold during verification.
    fn requires(&self) -> impl CvlrFormula<Context = Self::Context>;

    /// Returns a boolean expression representing the postconditions (ensures) of the lemma.
    ///
    /// This expression will be asserted to hold during verification.
    fn ensures(&self) -> impl CvlrFormula<Context = Self::Context>;

    /// Verifies the lemma with a non-deterministic context.
    ///
    /// This method:
    /// 1. Creates a non-deterministic context of type `Ctx`
    /// 2. Logs the context using [`clog!`](cvlr_log::clog)
    /// 3. Calls [`verify_with_context`](CvlrLemma::verify_with_context) with that context
    ///
    /// This is useful for verifying lemmas over all possible contexts.
    fn verify(&self) {
        let ctx = cvlr_nondet::nondet::<Self::Context>();
        cvlr_log::clog!(ctx);
        self.verify_with_context(&ctx);
    }

    /// Verifies the lemma with a specific context.
    ///
    /// This method:
    /// 1. Assumes the preconditions (requires) hold for the given context
    /// 2. Asserts that the postconditions (ensures) hold for the given context
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context to verify the lemma with
    fn verify_with_context(&self, ctx: &Self::Context) {
        self.requires().assume(ctx);
        self.ensures().assert(ctx);
    }

    /// Applies the lemma to a context.
    ///
    /// This method assumes the preconditions and asserts the postconditions,
    /// which is useful for applying lemmas in proofs. It has the same behavior
    /// as [`verify_with_context`](CvlrLemma::verify_with_context), but the name
    /// emphasizes that this is for applying the lemma rather than verifying it.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context to apply the lemma to
    fn apply(&self, ctx: &Self::Context) {
        self.requires().assume(ctx);
        self.ensures().assert(ctx);
    }
}
