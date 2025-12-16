//! Boolean expression types and traits.

use cvlr_asserts::{cvlr_assert, cvlr_assume};

/// A Boolean expression that can be evaluated, assumed, or asserted.
///
/// This trait represents a boolean expression parameterized by a context type `Ctx`.
/// Expressions implementing this trait can be:
/// - Evaluated to a boolean value via [`eval`](CvlrBoolExpr::eval)
/// - Asserted (checked) via [`assert`](CvlrBoolExpr::assert)
/// - Assumed (taken as a precondition) via [`assume`](CvlrBoolExpr::assume)
///
/// # Type Parameters
///
/// * `Ctx` - The context type used to evaluate the expression. This typically
///   represents the state or environment in which the expression is evaluated.
///
/// # Examples
///
/// ```
/// use cvlr_spec::CvlrBoolExpr;
///
/// struct MyContext {
///     value: i32,
/// }
///
/// struct IsPositive;
///
/// impl CvlrBoolExpr<MyContext> for IsPositive {
///     fn eval(&self, ctx: &MyContext) -> bool {
///         ctx.value > 0
///     }
/// }
/// ```
pub trait CvlrBoolExpr<Ctx> {
    /// Evaluates the expression in the given context.
    ///
    /// Returns `true` if the expression holds, `false` otherwise.
    fn eval(&self, ctx: &Ctx) -> bool;

    /// Asserts that the expression holds in the given context.
    ///
    /// This will cause a verification failure if the expression evaluates to `false`.
    /// The default implementation uses [`cvlr_assert!`] to check the result of [`eval`](CvlrBoolExpr::eval).
    fn assert(&self, ctx: &Ctx) {
        cvlr_assert!(self.eval(ctx));
    }

    /// Assumes that the expression holds in the given context.
    ///
    /// This adds the expression as a precondition that the verifier will assume to be true.
    /// The default implementation uses [`cvlr_assume!`] to assume the result of [`eval`](CvlrBoolExpr::eval).
    fn assume(&self, ctx: &Ctx) {
        cvlr_assume!(self.eval(ctx));
    }
}

/// A boolean expression that always evaluates to `true`.
///
/// This is a constant expression that can be used as a base case or placeholder
/// in boolean expression compositions.
#[derive(Copy, Clone)]
pub struct CvlrTrue;

impl<Ctx> CvlrBoolExpr<Ctx> for CvlrTrue {
    fn eval(&self, _ctx: &Ctx) -> bool {
        true
    }
    fn assert(&self, _ctx: &Ctx) {}
    fn assume(&self, _ctx: &Ctx) {}
}
