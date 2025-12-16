//! Combinators for composing boolean expressions.

use crate::bool_expr::CvlrBoolExpr;
use crate::state_pair::StatePair;
use core::marker::PhantomData;

/// A boolean expression representing the logical AND of two expressions.
///
/// This expression evaluates to `true` only when both sub-expressions evaluate to `true`.
/// When asserting or assuming, both sub-expressions are processed.
#[derive(Copy, Clone)]
pub struct CvlrAnd<A, B>(A, B);

impl<C, A: CvlrBoolExpr<C>, B: CvlrBoolExpr<C>> CvlrBoolExpr<C> for CvlrAnd<A, B> {
    fn eval(&self, ctx: &C) -> bool {
        self.0.eval(ctx) && self.1.eval(ctx)
    }
    fn assert(&self, ctx: &C) {
        self.0.assert(ctx);
        self.1.assert(ctx);
    }
    fn assume(&self, ctx: &C) {
        self.0.assume(ctx);
        self.1.assume(ctx);
    }
}

/// A wrapper type that preserves higher-ranked trait bounds (HRTB) for [`StatePair`].
///
/// This type is used internally to maintain the HRTB bound when composing
/// boolean expressions that operate on `StatePair` contexts. It ensures that
/// the resulting expression can work with any lifetime parameter for the `StatePair`.
#[derive(Copy, Clone)]
pub struct CvlrAndStatePair<Ctx, A, B>(A, B, PhantomData<Ctx>);

impl<'a, Ctx, A, B> CvlrBoolExpr<StatePair<'a, Ctx>> for CvlrAndStatePair<Ctx, A, B>
where
    A: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
    B: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
{
    fn eval(&self, ctx: &StatePair<'a, Ctx>) -> bool {
        self.0.eval(ctx) && self.1.eval(ctx)
    }
    fn assert(&self, ctx: &StatePair<'a, Ctx>) {
        self.0.assert(ctx);
        self.1.assert(ctx);
    }
    fn assume(&self, ctx: &StatePair<'a, Ctx>) {
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
pub fn cvlr_and<Ctx, A, B>(a: A, b: B) -> impl CvlrBoolExpr<Ctx>
where
    A: CvlrBoolExpr<Ctx>,
    B: CvlrBoolExpr<Ctx>,
{
    CvlrAnd(a, b)
}

/// Combines two boolean expressions with logical AND, preserving HRTB bounds for [`StatePair`].
///
/// This is a specialized version of [`cvlr_and`] for expressions that operate on `StatePair`.
/// It preserves the higher-ranked trait bound, allowing composition of `StatePair` expressions
/// while maintaining the HRTB bound.
///
/// # Arguments
///
/// * `a` - The first boolean expression (must work with any `StatePair` lifetime)
/// * `b` - The second boolean expression (must work with any `StatePair` lifetime)
///
/// # See Also
///
/// [`cvlr_and`] for the general version that works with any context type.
pub fn cvlr_and_pair<Ctx, A, B>(a: A, b: B) -> impl for<'a> CvlrBoolExpr<StatePair<'a, Ctx>>
where
    A: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
    B: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
{
    CvlrAndStatePair(a, b, PhantomData)
}

/// A boolean expression representing logical implication (A → B).
///
/// This expression evaluates to `true` when either the antecedent `A` is `false`,
/// or when both `A` and `B` are `true`. When asserting or assuming, the consequent
/// `B` is only processed if the antecedent `A` evaluates to `true`.
#[derive(Copy, Clone)]
pub struct CvlrImpl<A, B>(A, B);

impl<C, A: CvlrBoolExpr<C>, B: CvlrBoolExpr<C>> CvlrBoolExpr<C> for CvlrImpl<A, B> {
    fn eval(&self, ctx: &C) -> bool {
        if self.0.eval(ctx) {
            self.1.eval(ctx)
        } else {
            true
        }
    }

    fn assert(&self, ctx: &C) {
        if self.0.eval(ctx) {
            self.1.assert(ctx);
        }
    }
    fn assume(&self, ctx: &C) {
        if self.0.eval(ctx) {
            self.1.assume(ctx);
        }
    }
}

/// A wrapper type that preserves higher-ranked trait bounds (HRTB) for [`StatePair`] in implications.
///
/// This type is used internally to maintain the HRTB bound when composing
/// implication expressions that operate on `StatePair` contexts.
#[derive(Copy, Clone)]
pub struct CvlrImplStatePair<Ctx, A, B>(A, B, PhantomData<Ctx>);

impl<'a, Ctx, A, B> CvlrBoolExpr<StatePair<'a, Ctx>> for CvlrImplStatePair<Ctx, A, B>
where
    A: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
    B: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
{
    fn eval(&self, ctx: &StatePair<'a, Ctx>) -> bool {
        if self.0.eval(ctx) {
            self.1.eval(ctx)
        } else {
            true
        }
    }

    fn assert(&self, ctx: &StatePair<'a, Ctx>) {
        if self.0.eval(ctx) {
            self.1.assert(ctx);
        }
    }
    fn assume(&self, ctx: &StatePair<'a, Ctx>) {
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
pub fn cvlr_impl<Ctx, A, B>(a: A, b: B) -> impl CvlrBoolExpr<Ctx>
where
    A: CvlrBoolExpr<Ctx>,
    B: CvlrBoolExpr<Ctx>,
{
    CvlrImpl(a, b)
}

/// Creates a boolean expression representing logical implication, preserving HRTB bounds for [`StatePair`].
///
/// This is a specialized version of [`cvlr_impl`] for expressions that operate on `StatePair`.
/// It preserves the higher-ranked trait bound, allowing composition of `StatePair` expressions
/// while maintaining the HRTB bound.
///
/// # Arguments
///
/// * `a` - The antecedent expression (must work with any `StatePair` lifetime)
/// * `b` - The consequent expression (must work with any `StatePair` lifetime)
///
/// # See Also
///
/// [`cvlr_impl`] for the general version that works with any context type.
pub fn cvlr_impl_pair<Ctx, A, B>(a: A, b: B) -> impl for<'a> CvlrBoolExpr<StatePair<'a, Ctx>>
where
    A: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
    B: for<'b> CvlrBoolExpr<StatePair<'b, Ctx>>,
{
    CvlrImplStatePair(a, b, PhantomData)
}
