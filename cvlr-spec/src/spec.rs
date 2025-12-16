//! Specification types and traits.

use crate::bool_expr::CvlrBoolExpr;
use crate::state_pair::StatePair;

/// A specification that defines preconditions (requires) and postconditions (ensures).
///
/// This trait represents a contract specification for operations. Implementations
/// should:
/// - Assume preconditions hold before an operation (via [`assume_requires`](CvlrSpec::assume_requires))
/// - Check that postconditions hold after an operation (via [`check_ensures`](CvlrSpec::check_ensures))
///
/// # Type Parameters
///
/// * `Ctx` - The context type representing the state
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{CvlrSpec, CvlrTrue, cvlr_spec};
///
/// struct MyContext {
///     value: i32,
/// }
///
/// // Create a simple spec with requires and ensures
/// // CvlrTrue implements CvlrBoolExpr<Ctx> for any Ctx, including StatePair
/// let spec = cvlr_spec(CvlrTrue, CvlrTrue);
///
/// let ctx = MyContext { value: 5 };
/// spec.assume_requires(&ctx);
/// ```
pub trait CvlrSpec<Ctx> {
    /// Assumes that the preconditions (requires) hold for the given pre-state.
    ///
    /// This should be called before executing the operation to add the preconditions
    /// as assumptions that the verifier will take as true.
    ///
    /// # Arguments
    ///
    /// * `pre_state` - The state before the operation
    fn assume_requires(&self, pre_state: &Ctx);

    /// Checks that the postconditions (ensures) hold for the given pre/post state pair.
    ///
    /// This should be called after executing the operation to verify that the
    /// postconditions are satisfied.
    ///
    /// # Arguments
    ///
    /// * `pre_post_state_pair` - A pair containing both the pre-state and post-state
    fn check_ensures(&self, pre_post_state_pair: StatePair<'_, Ctx>);
}

/// An implementation of [`CvlrSpec`] that combines a precondition and postcondition.
///
/// This type stores a boolean expression for the precondition (requires) and
/// a boolean expression over [`StatePair`] for the postcondition (ensures).
#[derive(Copy, Clone)]
pub struct CvlrPropImpl<Pre, Post>(Pre, Post);

impl<Ctx, Pre, Post> CvlrSpec<Ctx> for CvlrPropImpl<Pre, Post>
where
    Pre: CvlrBoolExpr<Ctx>,
    Post: for<'a> CvlrBoolExpr<StatePair<'a, Ctx>>,
{
    fn assume_requires(&self, pre_state: &Ctx) {
        self.0.assume(pre_state);
    }
    fn check_ensures(&self, post_pre_state: StatePair<'_, Ctx>) {
        self.1.assert(&post_pre_state);
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
/// * `ensures` - A boolean expression over [`StatePair`] representing the postcondition
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_spec, StatePair, CvlrBoolExpr, CvlrTrue};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Define a spec: requires value >= 0, ensures value increases
/// let spec = cvlr_spec(
///     // requires
///     |c: &Counter| c.value >= 0,
///     // ensures
///     |pair: &StatePair<'_, Counter>| pair.ctx().value > pair.old().value,
/// );
/// ```
pub fn cvlr_spec<Ctx, Requires, Ensures>(requires: Requires, ensures: Ensures) -> impl CvlrSpec<Ctx>
where
    Requires: CvlrBoolExpr<Ctx>,
    Ensures: for<'a> CvlrBoolExpr<StatePair<'a, Ctx>>,
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

impl<Ctx, A, B> CvlrSpec<Ctx> for CvlrInvarSpec<A, B>
where
    A: CvlrBoolExpr<Ctx>,
    B: CvlrBoolExpr<Ctx>,
{
    fn assume_requires(&self, pre_state: &Ctx) {
        self.0.assume(pre_state);
        self.1.assume(pre_state);
    }
    fn check_ensures(&self, post_pre_state: StatePair<'_, Ctx>) {
        self.1.assert(post_pre_state.ctx());
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
/// use cvlr_spec::{cvlr_invar_spec, CvlrBoolExpr};
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
pub fn cvlr_invar_spec<Ctx, A, B>(assumption: A, invariant: B) -> CvlrInvarSpec<A, B>
where
    A: CvlrBoolExpr<Ctx>,
    B: CvlrBoolExpr<Ctx>,
{
    CvlrInvarSpec(assumption, invariant)
}
