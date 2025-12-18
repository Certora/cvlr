//! Boolean expression types and traits.

use cvlr_asserts::{cvlr_assert, cvlr_assume};

/// A Boolean expression that can be evaluated, assumed, or asserted.
///
/// This trait represents a boolean expression with an associated context type.
/// Expressions implementing this trait can be:
/// - Evaluated to a boolean value via [`eval`](CvlrFormula::eval) (single state)
/// - Evaluated over two states via [`eval_with_states`](CvlrFormula::eval_with_states) (pre-state and post-state)
/// - Asserted (checked) via [`assert`](CvlrFormula::assert) or [`assert_with_states`](CvlrFormula::assert_with_states)
/// - Assumed (taken as a precondition) via [`assume`](CvlrFormula::assume) or [`assume_with_states`](CvlrFormula::assume_with_states)
///
/// # Associated Types
///
/// * [`Context`](CvlrFormula::Context) - The context type used to evaluate the expression. This typically
///   represents the state or environment in which the expression is evaluated.
///
/// # Examples
///
/// ```
/// use cvlr_spec::CvlrFormula;
///
/// struct MyContext {
///     value: i32,
/// }
///
/// struct IsPositive;
///
/// impl CvlrFormula for IsPositive {
///     type Context = MyContext;
///     fn eval(&self, ctx: &Self::Context) -> bool {
///         ctx.value > 0
///     }
/// }
/// ```
pub trait CvlrFormula {
    type Context;

    /// Evaluates the expression in the given context.
    ///
    /// Returns `true` if the expression holds, `false` otherwise.
    fn eval(&self, ctx: &Self::Context) -> bool;

    /// Asserts that the expression holds in the given context.
    ///
    /// This will cause a verification failure if the expression evaluates to `false`.
    /// The default implementation uses [`cvlr_assert!`] to check the result of [`eval`](CvlrFormula::eval).
    fn assert(&self, ctx: &Self::Context) {
        cvlr_assert!(self.eval(ctx));
    }

    /// Assumes that the expression holds in the given context.
    ///
    /// This adds the expression as a precondition that the verifier will assume to be true.
    /// The default implementation uses [`cvlr_assume!`] to assume the result of [`eval`](CvlrFormula::eval).
    fn assume(&self, ctx: &Self::Context) {
        cvlr_assume!(self.eval(ctx));
    }

    /// Evaluates the expression over two states (pre-state and post-state).
    ///
    /// This method allows expressions to be evaluated in contexts that require
    /// comparing values from two different states, such as checking invariants
    /// across state transitions or comparing before and after values.
    ///
    /// # Parameters
    ///
    /// * `ctx0` - The pre-state context (before the transition)
    /// * `ctx1` - The post-state context (after the transition)
    ///
    /// # Returns
    ///
    /// Returns `true` if the expression holds across both states, `false` otherwise.
    ///
    /// # Default Implementation
    ///
    /// The default implementation evaluates the expression using only the pre-state
    /// context (`ctx0`). Expressions that need to compare both states should override
    /// this method.
    fn eval_with_states(&self, ctx0: &Self::Context, _: &Self::Context) -> bool {
        self.eval(ctx0)
    }

    /// Asserts that the expression holds across two states (pre-state and post-state).
    ///
    /// This will cause a verification failure if the expression evaluates to `false`
    /// when considering both the pre-state and post-state contexts.
    ///
    /// # Parameters
    ///
    /// * `ctx0` - The pre-state context (before the transition)
    /// * `ctx1` - The post-state context (after the transition)
    ///
    /// # Default Implementation
    ///
    /// The default implementation asserts the expression using only the pre-state
    /// context (`ctx0`). Expressions that need to compare both states should override
    /// this method.
    fn assert_with_states(&self, ctx0: &Self::Context, _: &Self::Context) {
        self.assert(ctx0);
    }

    /// Assumes that the expression holds across two states (pre-state and post-state).
    ///
    /// This adds the expression as a precondition that the verifier will assume to be true
    /// when considering both the pre-state and post-state contexts.
    ///
    /// # Parameters
    ///
    /// * `ctx0` - The pre-state context (before the transition)
    /// * `ctx1` - The post-state context (after the transition)
    ///
    /// # Default Implementation
    ///
    /// The default implementation assumes the expression using only the pre-state
    /// context (`ctx0`). Expressions that need to compare both states should override
    /// this method.
    fn assume_with_states(&self, ctx0: &Self::Context, _: &Self::Context) {
        self.assume(ctx0);
    }
}

/// A boolean expression that always evaluates to `true`.
///
/// This is a constant expression that can be used as a base case or placeholder
/// in boolean expression compositions.
#[derive(Copy, Clone)]
pub struct CvlrTrue<Ctx>(core::marker::PhantomData<Ctx>);

impl<Ctx> CvlrFormula for CvlrTrue<Ctx> {
    type Context = Ctx;
    fn eval(&self, _ctx: &Self::Context) -> bool {
        true
    }
    fn assert(&self, _ctx: &Self::Context) {}
    fn assume(&self, _ctx: &Self::Context) {}
}

pub fn cvlr_true<Ctx>() -> impl CvlrFormula<Context = Ctx> {
    CvlrTrue(core::marker::PhantomData)
}
