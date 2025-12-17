//! Combinators for composing boolean expressions.

use crate::bool_expr::CvlrBoolExpr;

/// A boolean expression representing the logical AND of two expressions.
///
/// This expression evaluates to `true` only when both sub-expressions evaluate to `true`.
/// When asserting or assuming, both sub-expressions are processed.
#[derive(Copy, Clone)]
pub struct CvlrAnd<A, B>(A, B);

impl<A, B> CvlrBoolExpr for CvlrAnd<A, B>
where
    A: CvlrBoolExpr,
    B: CvlrBoolExpr<Context = A::Context>,
{
    type Context = A::Context;
    fn eval(&self, ctx: &Self::Context) -> bool {
        self.0.eval(ctx) && self.1.eval(ctx)
    }
    fn assert(&self, ctx: &Self::Context) {
        self.0.assert(ctx);
        self.1.assert(ctx);
    }
    fn assume(&self, ctx: &Self::Context) {
        self.0.assume(ctx);
        self.1.assume(ctx);
    }
}

/// Combines two boolean expressions with logical AND.
///
/// Returns a new expression that evaluates to `true` only when both input
/// expressions evaluate to `true`.
///
/// # Arguments
///
/// * `a` - The first boolean expression
/// * `b` - The second boolean expression
///
/// # Examples
///
/// ```
/// use cvlr_spec::{cvlr_and, CvlrTrue, CvlrBoolExpr};
///
/// let expr = cvlr_and(CvlrTrue, CvlrTrue);
/// assert!(expr.eval(&()));
/// ```
pub fn cvlr_and<A, B>(a: A, b: B) -> CvlrAnd<A, B>
where
    A: CvlrBoolExpr,
    B: CvlrBoolExpr<Context = A::Context>,
{
    CvlrAnd(a, b)
}

/// A boolean expression representing logical implication (A → B).
///
/// This expression evaluates to `true` when either the antecedent `A` is `false`,
/// or when both `A` and `B` are `true`. When asserting or assuming, the consequent
/// `B` is only processed if the antecedent `A` evaluates to `true`.
#[derive(Copy, Clone)]
pub struct CvlrImpl<A, B>(A, B);

impl<A, B> CvlrBoolExpr for CvlrImpl<A, B>
where
    A: CvlrBoolExpr,
    B: CvlrBoolExpr<Context = A::Context>,
{
    type Context = A::Context;
    fn eval(&self, ctx: &Self::Context) -> bool {
        if self.0.eval(ctx) {
            self.1.eval(ctx)
        } else {
            true
        }
    }

    fn assert(&self, ctx: &Self::Context) {
        if self.0.eval(ctx) {
            self.1.assert(ctx);
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        if self.0.eval(ctx) {
            self.1.assume(ctx);
        }
    }
}


/// Creates a boolean expression representing logical implication (A → B).
///
/// Returns a new expression that evaluates to `true` when either `a` is `false`
/// or when both `a` and `b` are `true`.
///
/// # Arguments
///
/// * `a` - The antecedent (left-hand side) of the implication
/// * `b` - The consequent (right-hand side) of the implication
///
/// # Examples
///
/// ```
/// use cvlr_spec::{cvlr_impl, CvlrTrue, CvlrBoolExpr};
///
/// // true → true evaluates to true
/// let expr = cvlr_impl(CvlrTrue, CvlrTrue);
/// assert!(expr.eval(&()));
/// ```
pub fn cvlr_impl<A, B>(a: A, b: B) -> CvlrImpl<A, B>
where
    A: CvlrBoolExpr,
    B: CvlrBoolExpr<Context = A::Context>,
{
    CvlrImpl(a, b)
}
