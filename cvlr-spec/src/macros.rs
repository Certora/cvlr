//! Macros for defining predicates.

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
        impl $crate::CvlrBoolExpr for $name {
            type Context = $ctx;
            fn eval(&self, ctx: &Self::Context) -> bool {
                let $c = ctx;
                $crate::__macro_support::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert(&self, ctx: &Self::Context) {
                let $c = ctx;
                $crate::__macro_support::cvlr_assert_all!(
                    $($e),*
                );
            }

            fn assume(&self, ctx: &Self::Context) {
                let $c = ctx;
                $crate::__macro_support::cvlr_assume_all!(
                    $($e),*
                );
            }
        }
    };
}

/// Defines a predicate over a `(Context, Context)` tuple.
///
/// This macro creates a new type implementing [`CvlrBoolExpr`] for `(Ctx, Ctx)` tuples.
/// The predicate body has access to both the current context (`c`, the first element) and the old context (`o`, the second element),
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
        impl $crate::CvlrBoolExpr for $name {
            type Context = ($ctx, $ctx);
            fn eval(&self, ctx: &Self::Context) -> bool {
                let $c = ctx.0;
                let $o = ctx.1;
                $crate::__macro_support::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert(&self, ctx: &Self::Context) {
                let $c = ctx.0;
                let $o = ctx.1;
                $crate::__macro_support::cvlr_assert_all!(
                    $($e),*
                );
            }

            fn assume(&self, ctx: &Self::Context) {
                let $c = ctx.0;
                let $o = ctx.1;
                $crate::__macro_support::cvlr_assume_all!(
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
/// over `(Context, Context)` tuples at once using the same syntax as [`cvlr_def_state_pair_predicate!`].
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

/// Creates an anonymous predicate (boolean expression) over a context type.
///
/// This macro creates an anonymous predicate that implements [`CvlrBoolExpr`](crate::CvlrBoolExpr) for the
/// specified context type. Unlike [`cvlr_def_predicate!`], this macro creates an
/// unnamed predicate that can be used inline without defining a separate type.
///
/// # Syntax
///
/// ```ignore
/// cvlr_predicate! {
///     | <context_var> : <context_type> | -> {
///         <expression1>;
///         <expression2>;
///         ...
///     }
/// }
/// ```
///
/// # Parameters
///
/// * `context_var` - The variable name to use for the context in the predicate body
/// * `context_type` - The type of the context
/// * `expressions` - One or more expressions that form the predicate body
///
/// # Returns
///
/// Returns a value implementing [`CvlrBoolExpr`](crate::CvlrBoolExpr) with `Context = Ctx` that can be evaluated,
/// asserted, or assumed.
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::cvlr_predicate;
///
/// struct Counter {
///     value: i32,
/// }
///
/// let ctx = Counter { value: 5 };
///
/// // Create an anonymous predicate
/// let pred = cvlr_predicate! { | c : Counter | -> {
///     c.value > 0;
///     c.value < 100
/// } };
///
/// assert!(pred.eval(&ctx));
/// ```
///
/// Note: This example is marked `ignore` because it doesn't require special setup.
/// In actual usage, predicates are often used within lemmas or other verification contexts.
///
/// This macro is often used internally by [`cvlr_lemma!`] to create the requires
/// and ensures predicates.
#[macro_export]
macro_rules! cvlr_predicate {
    (| $c:ident : $ctx: ident | -> { $( $e: expr );* $(;)? } ) => {
        {
            $crate::cvlr_def_predicate! {
                pred __AnonymousPredicate ($c : $ctx) { $( $e );* }
            }
            __AnonymousPredicate
        }
    };
}

/// Defines a lemma with preconditions (requires) and postconditions (ensures).
///
/// This macro creates a new type implementing [`CvlrLemma`](spec::CvlrLemma) for the specified context.
/// A lemma is a logical statement: if the preconditions hold, then the postconditions
/// must also hold. Lemmas can be verified using the [`verify`](spec::CvlrLemma::verify) or
/// [`verify_with_context`](spec::CvlrLemma::verify_with_context) methods.
///
/// # Syntax
///
/// ```ignore
/// cvlr_lemma! {
///     <name> ( <context_var> : <context_type> ) {
///         requires -> {
///             <requires_expr1>;
///             <requires_expr2>;
///             ...
///         }
///         ensures -> {
///             <ensures_expr1>;
///             <ensures_expr2>;
///             ...
///         }
///     }
/// }
/// ```
///
/// # Parameters
///
/// * `name` - The name of the lemma type to create
/// * `context_var` - The variable name to use for the context in the requires/ensures clauses
/// * `context_type` - The type of the context (must implement [`Nondet`](cvlr_nondet::Nondet) and [`CvlrLog`](cvlr_log::CvlrLog))
/// * `requires` - One or more expressions that form the preconditions
/// * `ensures` - One or more expressions that form the postconditions
///
/// # Returns
///
/// Creates a struct with the given name that implements [`CvlrLemma`](spec::CvlrLemma) with `Context = Ctx`.
///
/// # Examples
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
/// // Define a lemma: if value > 0, then value > 0 (trivial but demonstrates syntax)
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
/// // Use the lemma
/// let lemma = CounterPositiveLemma;
/// lemma.verify(); // Verifies the lemma holds for all contexts
/// ```
///
/// More complex example:
///
/// ```ignore
/// extern crate cvlr;
/// use cvlr_spec::cvlr_lemma;
///
/// #[derive(cvlr::derive::Nondet, cvlr::derive::CvlrLog)]
/// struct Counter {
///     value: i32,
/// }
///
/// cvlr_lemma! {
///     CounterDoublesLemma(c: Counter) {
///         requires -> {
///             c.value > 0;
///             c.value < 100
///         }
///         ensures -> {
///             c.value > 0;
///             c.value * 2 > c.value
///         }
///     }
/// }
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
#[macro_export]
macro_rules! cvlr_lemma {
    ($name: ident ( $c:ident : $ctx: ident ) {
        requires -> { $( $requires: expr );* $(;)? }
        ensures -> { $( $ensures: expr );* $(;)? } }) => {
            struct $name;
            impl $crate::spec::CvlrLemma for $name {
                type Context = $ctx;
                fn requires(&self) -> impl $crate::CvlrBoolExpr<Context = Self::Context> {
                    $crate::cvlr_predicate! { | $c : $ctx | -> { $( $requires );* } }
                }
                fn ensures(&self) -> impl $crate::CvlrBoolExpr<Context = Self::Context> {
                    $crate::cvlr_predicate! { | $c : $ctx | -> { $( $ensures );* } }
                }
            }
        };
}
