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

use core::marker::PhantomData;
use cvlr_asserts::{cvlr_assert, cvlr_assume};

pub mod cvlr_macros {
    pub use cvlr_macros::*;
}

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

/// Defines a predicate (boolean expression) over a context type.
///
/// This macro creates a new type implementing [`CvlrBoolExpr`] for the specified context.
/// The predicate body consists of one or more expressions that are evaluated, asserted,
/// or assumed together.
///
/// # Syntax
///
/// ```ignore
/// cvlr_def_predicate! {
///     pred <name> ( <context_var> : <context_type> ) {
///         <expression1>;
///         <expression2>;
///         ...
///     }
/// }
/// ```
///
/// # Parameters
///
/// * `name` - The name of the predicate type to create
/// * `context_var` - The variable name to use for the context in the predicate body
/// * `context_type` - The type of the context
/// * `expressions` - One or more expressions that form the predicate body
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::cvlr_def_predicate;
///
/// struct MyContext {
///     x: i32,
///     y: i32,
/// }
///
/// cvlr_def_predicate! {
///     pred IsPositive ( c : MyContext ) {
///         c.x > 0;
///         c.y > 0;
///     }
/// }
///
/// let ctx = MyContext { x: 5, y: 10 };
/// let pred = IsPositive;
/// assert!(pred.eval(&ctx));
/// ```
#[macro_export]
macro_rules! cvlr_def_predicate {
    (pred $name: ident ( $c:ident : $ctx: ident ) {  $( $e: expr );* $(;)? }) => {
        struct $name;
        impl CvlrBoolExpr<$ctx> for $name {
            fn eval(&self, ctx: &$ctx) -> bool {
                let $c = ctx;
                $crate::cvlr_macros::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert(&self, ctx: &$ctx) {
                let $c = ctx;
                $crate::cvlr_macros::cvlr_assert_all!(
                    $($e),*
                );
            }

            fn assume(&self, ctx: &$ctx) {
                let $c = ctx;
                $crate::cvlr_macros::cvlr_assume_all!(
                    $($e),*
                );
            }
        }
    };
}

/// Defines a predicate over a [`StatePair`] context.
///
/// This macro creates a new type implementing [`CvlrBoolExpr`] for `StatePair<Ctx>`.
/// The predicate body has access to both the current context (`c`) and the old context (`o`),
/// allowing you to express postconditions that compare pre-state and post-state.
///
/// # Syntax
///
/// ```ignore
/// cvlr_def_state_pair_predicate! {
///     pred <name> ( [ <current_var>, <old_var> ] : <context_type> ) {
///         <expression1>;
///         <expression2>;
///         ...
///     }
/// }
/// ```
///
/// # Parameters
///
/// * `name` - The name of the predicate type to create
/// * `current_var` - The variable name for the current/post-state context
/// * `old_var` - The variable name for the old/pre-state context
/// * `context_type` - The type of the context
/// * `expressions` - One or more expressions that form the predicate body
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::cvlr_def_state_pair_predicate;
///
/// struct Counter {
///     value: i32,
/// }
///
/// cvlr_def_state_pair_predicate! {
///     pred CounterIncreases ( [ c, o ] : Counter ) {
///         c.value > o.value;
///     }
/// }
/// ```
#[macro_export]
macro_rules! cvlr_def_state_pair_predicate {
    (pred $name: ident ( [ $c:ident, $o: ident ] : $ctx: ident ) {  $( $e: expr );* $(;)? }) => {
        struct $name;
        impl CvlrBoolExpr<::cvlr_spec::StatePair<'_, $ctx>> for $name {
            fn eval(&self, ctx: &::cvlr_spec::StatePair<'_, $ctx>) -> bool {
                let $c = ctx.ctx();
                let $o = ctx.old();
                $crate::cvlr_macros::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert(&self, ctx: &::cvlr_spec::StatePair<'_, $ctx>) {
                let $c = ctx.ctx();
                let $o = ctx.old();
                $crate::cvlr_macros::cvlr_assert_all!(
                    $($e),*
                );
            }

            fn assume(&self, ctx: &::cvlr_spec::StatePair<'_, $ctx>) {
                let $c = ctx.ctx();
                let $o = ctx.old();
                $crate::cvlr_macros::cvlr_assume_all!(
                    $($e),*
                );
            }
        }
    };
}

/// Defines multiple predicates in a single macro invocation.
///
/// This is a convenience macro that allows you to define multiple predicates
/// at once using the same syntax as [`cvlr_def_predicate!`].
///
/// # Syntax
///
/// ```ignore
/// cvlr_def_predicates! {
///     pred <name1> ( <c1> : <ctx> ) { ... }
///     pred <name2> ( <c2> : <ctx> ) { ... }
///     ...
/// }
/// ```
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::cvlr_def_predicates;
///
/// struct MyContext {
///     x: i32,
/// }
///
/// cvlr_def_predicates! {
///     pred IsPositive ( c : MyContext ) {
///         c.x > 0;
///     }
///     pred IsEven ( c : MyContext ) {
///         c.x % 2 == 0;
///     }
/// }
/// ```
#[macro_export]
macro_rules! cvlr_def_predicates {
    ($(pred $name: ident ( $c:ident : $ctx: ident ) {  $( $e: expr );* $(;)? })*) => {
        $(
            $crate::cvlr_def_predicate! {
                pred $name ( $c : $ctx ) { $( $e );* }
            }
        )*
    };
}

/// Defines multiple state pair predicates in a single macro invocation.
///
/// This is a convenience macro that allows you to define multiple predicates
/// over [`StatePair`] at once using the same syntax as [`cvlr_def_state_pair_predicate!`].
///
/// # Syntax
///
/// ```ignore
/// cvlr_def_state_pair_predicates! {
///     pred <name1> ( [ <c1>, <o1> ] : <ctx> ) { ... }
///     pred <name2> ( [ <c2>, <o2> ] : <ctx> ) { ... }
///     ...
/// }
/// ```
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::cvlr_def_state_pair_predicates;
///
/// struct Counter {
///     value: i32,
/// }
///
/// cvlr_def_state_pair_predicates! {
///     pred Increases ( [ c, o ] : Counter ) {
///         c.value > o.value;
///     }
///     pred NonDecreasing ( [ c, o ] : Counter ) {
///         c.value >= o.value;
///     }
/// }
/// ```
#[macro_export]
macro_rules! cvlr_def_state_pair_predicates {
    ($(pred $name: ident ( [ $c:ident, $o: ident ] : $ctx: ident ) {  $( $e: expr );* $(;)? })*) => {
        $(
            $crate::cvlr_def_state_pair_predicate! {
                pred $name ( [ $c, $o ] : $ctx ) { $( $e );* }
            }
        )*
    };
}

/// A pair of contexts representing pre-state and post-state.
///
/// This type is used to express postconditions that compare the state before
/// and after an operation. It holds references to both the current (post) state
/// and the old (pre) state.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the context references
/// * `Ctx` - The type of the context
///
/// # Examples
///
/// ```
/// use cvlr_spec::StatePair;
///
/// struct Counter {
///     value: i32,
/// }
///
/// let pre = Counter { value: 5 };
/// let post = Counter { value: 10 };
/// let pair = StatePair::new(&post, &pre);
///
/// // Access current state
/// assert_eq!(pair.ctx().value, 10);
/// assert_eq!(pair.post().value, 10);
///
/// // Access old state
/// assert_eq!(pair.old().value, 5);
/// assert_eq!(pair.pre().value, 5);
/// ```
#[derive(Copy, Clone)]
pub struct StatePair<'a, Ctx> {
    /// The current (post-state) context.
    pub ctx: &'a Ctx,
    /// The old (pre-state) context.
    pub old: &'a Ctx,
}

impl<'a, Ctx> StatePair<'a, Ctx> {
    /// Creates a new `StatePair` from separate pre-state and post-state contexts.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The current/post-state context
    /// * `old` - The old/pre-state context
    pub fn new(ctx: &'a Ctx, old: &'a Ctx) -> Self {
        Self { ctx, old }
    }

    /// Creates a `StatePair` where both pre-state and post-state refer to the same context.
    ///
    /// This is useful when you need a `StatePair` but there's no meaningful distinction
    /// between pre and post states (e.g., for invariants that hold at a single point in time).
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context to use for both pre-state and post-state
    pub fn singleton(ctx: &'a Ctx) -> Self {
        Self { ctx, old: ctx }
    }

    /// Returns a reference to the current (post-state) context.
    ///
    /// This is an alias for accessing the post-state. See also [`post`](StatePair::post).
    pub fn ctx(&self) -> &'a Ctx {
        self.ctx
    }

    /// Returns a reference to the old (pre-state) context.
    ///
    /// This is an alias for accessing the pre-state. See also [`pre`](StatePair::pre).
    pub fn old(&self) -> &'a Ctx {
        self.old
    }

    /// Returns a reference to the pre-state context.
    ///
    /// This is an alias for [`old`](StatePair::old).
    pub fn pre(&self) -> &'a Ctx {
        self.old
    }

    /// Returns a reference to the post-state context.
    ///
    /// This is an alias for [`ctx`](StatePair::ctx).
    pub fn post(&self) -> &'a Ctx {
        self.ctx
    }
}

impl<Ctx> core::ops::Deref for StatePair<'_, Ctx> {
    type Target = Ctx;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

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
