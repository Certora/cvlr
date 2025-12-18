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

/// Defines a predicate that evaluates over two states.
///
/// This macro creates a new type implementing [`CvlrBoolExpr`] with `Context = Ctx`.
/// The predicate uses [`eval_with_states`](crate::CvlrBoolExpr::eval_with_states) to evaluate
/// over both the current/post-state (`c`) and the old/pre-state (`o`),
/// allowing you to express postconditions that compare pre-state and post-state.
///
/// # Syntax
///
/// ```ignore
/// cvlr_def_states_predicate! {
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
/// use cvlr_spec::cvlr_def_states_predicate;
///
/// struct Counter {
///     value: i32,
/// }
///
/// cvlr_def_states_predicate! {
///     pred CounterIncreases ( [ c, o ] : Counter ) {
///         c.value > o.value;
///     }
/// }
///
/// // Use the predicate with eval_with_states
/// let pre = Counter { value: 5 };
/// let post = Counter { value: 10 };
/// let pred = CounterIncreases;
/// assert!(pred.eval_with_states(&post, &pre));
/// ```
#[macro_export]
macro_rules! cvlr_def_states_predicate {
    (pred $name: ident ( [ $c:ident, $o: ident ] : $ctx: ident ) {  $( $e: expr );* $(;)? }) => {
        struct $name;
        impl $crate::CvlrBoolExpr for $name {
            type Context = $ctx;
            fn eval_with_states(&self, ctx0: &Self::Context, ctx1: &Self::Context) -> bool {
                let $c = ctx0;
                let $o = ctx1;
                $crate::__macro_support::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert_with_states(&self, ctx0: &Self::Context, ctx1: &Self::Context) {
                let $c = ctx0;
                let $o = ctx1;
                $crate::__macro_support::cvlr_assert_all!(
                    $($e),*
                );
            }
            fn assume_with_states(&self, ctx0: &Self::Context, ctx1: &Self::Context) {
                let $c = ctx0;
                let $o = ctx1;
                $crate::__macro_support::cvlr_assume_all!(
                    $($e),*
                );
            }

            fn eval(&self, ctx: &Self::Context) -> bool {
                $crate::__macro_support::cvlr_assert!(false);
                panic!("eval should never be called for a state pair predicate; use eval_with_states instead");
            }
            fn assert(&self, ctx: &Self::Context) {
                $crate::__macro_support::cvlr_assert!(false);
                panic!("assert should never be called for a state pair predicate; use assert_with_states instead");
            }
            fn assume(&self, ctx: &Self::Context) {
                $crate::__macro_support::cvlr_assert!(false);
                panic!("assume should never be called for a state pair predicate; use assume_with_states instead");
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

/// Defines multiple state predicates in a single macro invocation.
///
/// This is a convenience macro that allows you to define multiple predicates
/// that evaluate over two states at once using the same syntax as [`cvlr_def_states_predicate!`].
///
/// # Syntax
///
/// ```ignore
/// cvlr_def_states_predicates! {
///     pred <name1> ( [ <c1>, <o1> ] : <ctx> ) { ... }
///     pred <name2> ( [ <c2>, <o2> ] : <ctx> ) { ... }
///     ...
/// }
/// ```
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::cvlr_def_states_predicates;
///
/// struct Counter {
///     value: i32,
/// }
///
/// cvlr_def_states_predicates! {
///     pred Increases ( [ c, o ] : Counter ) {
///         c.value > o.value;
///     }
///     pred NonDecreasing ( [ c, o ] : Counter ) {
///         c.value >= o.value;
///     }
/// }
/// ```
#[macro_export]
macro_rules! cvlr_def_states_predicates {
    ($(pred $name: ident ( [ $c:ident, $o: ident ] : $ctx: ident ) {  $( $e: expr );* $(;)? })*) => {
        $(
            $crate::cvlr_def_states_predicate! {
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

/// Defines multiple rules for a specification across multiple base functions.
///
/// This macro is a convenience macro that generates multiple rule definitions
/// by calling [`cvlr_rule_for_spec!`](crate::__macro_support::cvlr_rule_for_spec) for each
/// base function in the list. Each rule will have the same name and specification,
/// but will be applied to different base functions.
///
/// # Syntax
///
/// ```ignore
/// cvlr_rules! {
///     name: "rule_name",
///     spec: spec_expression,
///     bases: [
///         base_function1,
///         base_function2,
///         base_function3,
///     ]
/// }
/// ```
///
/// # Parameters
///
/// * `name` - A string literal that will be converted to snake_case and combined with each base function name
/// * `spec` - An expression representing the specification (must implement [`CvlrSpec`](crate::spec::CvlrSpec))
/// * `bases` - A list of function identifiers (if they start with `base_`, that prefix is stripped)
///
/// # Expansion
///
/// This macro expands to multiple calls to `cvlr_rule_for_spec!`, one for each base function:
///
/// ```text
/// // Input:
/// cvlr_rules! {
///     name: "solvency",
///     spec: my_spec,
///     bases: [base_function1, base_function2]
/// }
///
/// // Expands to:
/// cvlr_rule_for_spec!{name: "solvency", spec: my_spec, base: base_function1}
/// cvlr_rule_for_spec!{name: "solvency", spec: my_spec, base: base_function2}
/// ```
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_rules, cvlr_spec, cvlr_predicate};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Create a spec
/// let my_spec = cvlr_spec! {
///     requires: cvlr_predicate! { | c : Counter | -> { c.value > 0 } },
///     ensures: cvlr_predicate! { | c : Counter | -> { c.value >= 0 } }
/// };
///
/// // Define rules for multiple functions
/// cvlr_rules! {
///     name: "solvency",
///     spec: my_spec,
///     bases: [
///         base_update_counter,
///         base_reset_counter,
///         base_increment_counter,
///     ]
/// }
/// ```
///
/// This will create three rules:
/// - `solvency_update_counter`
/// - `solvency_reset_counter`
/// - `solvency_increment_counter`
#[macro_export]
macro_rules! cvlr_rules {
    (name: $name:literal, spec: $spec:expr, bases: [ $( $base:ident ),* $(,)? ] ) => {
        $(
            $crate::__macro_support::cvlr_rule_for_spec!{name: $name, spec: $spec, base: $base}
        )*
    };
}

/// Creates a specification from preconditions (requires) and postconditions (ensures).
///
/// This macro is a convenience wrapper around [`cvlr_spec`](crate::spec::cvlr_spec) that
/// provides a more readable syntax for creating specifications.
///
/// # Syntax
///
/// ```ignore
/// cvlr_spec! {
///     requires: requires_expression,
///     ensures: ensures_expression,
/// }
/// ```
///
/// # Parameters
///
/// * `requires` - A boolean expression over the context type representing the precondition
/// * `ensures` - A boolean expression over the context type that uses [`eval_with_states`](crate::CvlrBoolExpr::eval_with_states)
///   to evaluate over both pre-state and post-state
///
/// # Returns
///
/// Returns a value implementing [`CvlrSpec`](crate::spec::CvlrSpec) with the same context type as the requires expression.
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
/// // Define a predicate for the ensures clause that compares pre and post states
/// cvlr_def_states_predicate! {
///     pred CounterIncreases ( [ c, o ] : Counter ) {
///         c.value > o.value;
///     }
/// }
///
/// // Create a spec using the macro
/// let spec = cvlr_spec! {
///     requires: cvlr_predicate! { | c : Counter | -> { c.value >= 0 } },
///     ensures: CounterIncreases,
/// };
///
/// // Use the spec
/// let pre = Counter { value: 5 };
/// let post = Counter { value: 10 };
/// spec.assume_requires(&pre);
/// spec.check_ensures(&post, &pre);
/// ```
#[macro_export]
macro_rules! cvlr_spec {
    (requires: $r:expr, ensures: $e:expr) => {
        $crate::cvlr_spec($r, $e)
    };
}

/// Creates an invariant specification from an assumption and an invariant.
///
/// This macro is a convenience wrapper around [`cvlr_invar_spec`](crate::spec::cvlr_invar_spec) that
/// provides a more readable syntax for creating invariant specifications.
///
/// An invariant specification represents a contract where:
/// - An assumption (additional precondition) is assumed before the operation
/// - An invariant must hold both before and after the operation
///
/// # Syntax
///
/// ```ignore
/// cvlr_invar_spec! {
///     assumption: assumption_expression,
///     invariant: invariant_expression,
/// }
/// ```
///
/// # Parameters
///
/// * `assumption` - A boolean expression representing an additional precondition
/// * `invariant` - A boolean expression representing an invariant that must hold before and after
///
/// # Returns
///
/// Returns a value implementing [`CvlrSpec`](crate::spec::CvlrSpec) with the same context type as the assumption expression.
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_invar_spec, cvlr_predicate};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Create an invariant spec
/// let spec = cvlr_invar_spec! {
///     assumption: cvlr_predicate! { | c : Counter | -> { c.value % 2 == 0 } },
///     invariant: cvlr_predicate! { | c : Counter | -> { c.value >= 0 } },
/// };
///
/// // Use the spec
/// let ctx = Counter { value: 4 };
/// spec.assume_requires(&ctx);  // Assumes both assumption and invariant
/// spec.check_ensures(&ctx, &ctx);  // Asserts invariant holds
/// ```
#[macro_export]
macro_rules! cvlr_invar_spec {
    (assumption: $a:expr, invariant: $i:expr) => {
        $crate::cvlr_invar_spec($a, $i)
    };
}

/// Defines multiple rules for an invariant specification across multiple base functions.
///
/// This macro is a convenience macro that combines [`cvlr_invar_spec!`] and [`cvlr_rules!`]
/// to create multiple rules with an invariant specification. It generates multiple rule definitions
/// by creating an invariant spec from the assumption and invariant, then applying it to each
/// base function in the list.
///
/// # Syntax
///
/// ```ignore
/// cvlr_invariant_rules! {
///     name: "rule_name",
///     assumption: assumption_expression,
///     invariant: invariant_expression,
///     bases: [
///         base_function1,
///         base_function2,
///         base_function3,
///     ]
/// }
/// ```
///
/// # Parameters
///
/// * `name` - A string literal that will be converted to snake_case and combined with each base function name
/// * `assumption` - A boolean expression representing an additional precondition
/// * `invariant` - A boolean expression representing an invariant that must hold before and after
/// * `bases` - A list of function identifiers (if they start with `base_`, that prefix is stripped)
///
/// # Expansion
///
/// This macro expands to multiple calls to `cvlr_rule_for_spec!`, one for each base function,
/// with an invariant spec created from the assumption and invariant:
///
/// ```text
/// // Input:
/// cvlr_invariant_rules! {
///     name: "non_negative",
///     assumption: assumption_expr,
///     invariant: invariant_expr,
///     bases: [base_function1, base_function2]
/// }
///
/// // Expands to:
/// cvlr_rule_for_spec!{name: "non_negative", spec: cvlr_invar_spec(assumption_expr, invariant_expr), base: base_function1}
/// cvlr_rule_for_spec!{name: "non_negative", spec: cvlr_invar_spec(assumption_expr, invariant_expr), base: base_function2}
/// ```
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_invariant_rules, cvlr_predicate};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Define invariant rules for multiple functions
/// cvlr_invariant_rules! {
///     name: "non_negative",
///     assumption: cvlr_predicate! { | c : Counter | -> { c.value % 2 == 0 } },
///     invariant: cvlr_predicate! { | c : Counter | -> { c.value >= 0 } },
///     bases: [
///         base_update_counter,
///         base_reset_counter,
///         base_increment_counter,
///     ]
/// }
/// ```
///
/// This will create three rules:
/// - `non_negative_update_counter`
/// - `non_negative_reset_counter`
/// - `non_negative_increment_counter`
///
/// Each rule uses an invariant specification that:
/// - Assumes both the assumption and invariant in the pre-state
/// - Asserts the invariant in the post-state
#[macro_export]
macro_rules! cvlr_invariant_rules {
    (name: $name:literal, assumption: $a:expr, invariant: $i:expr, bases: [ $( $base:ident ),* $(,)? ] ) => {
        $(
            $crate::__macro_support::cvlr_rule_for_spec!{
                name: $name,
                spec: $crate::spec::cvlr_invar_spec($a, $i),
                base: $base
            }
        )*
    };
}

/// Creates a boolean expression representing the logical AND of two or more expressions.
///
/// This macro is a convenience wrapper around [`cvlr_and`](crate::cvlr_and) that
/// provides flexible syntax for combining boolean expressions. It supports both identifiers
/// and expressions as arguments, and can combine 2 to 6 expressions.
///
/// # Syntax
///
/// ```ignore
/// cvlr_and!(a, b)                    // Two arguments
/// cvlr_and!(a, b, c)                 // Three arguments
/// cvlr_and!(a, b, c, d)              // Four arguments
/// cvlr_and!(a, b, c, d, e)           // Five arguments
/// cvlr_and!(a, b, c, d, e, f)        // Six arguments
/// ```
///
/// # Arguments
///
/// The macro accepts identifiers or expressions that implement [`CvlrBoolExpr`](crate::CvlrBoolExpr)
/// with the same context type. Arguments can be:
/// - Identifiers (e.g., `XPositive`)
/// - Expressions (e.g., `cvlr_predicate! { | c : Ctx | -> { c.x > 0 } }`)
/// - Mixed (e.g., `cvlr_and!(XPositive, cvlr_predicate! { | c : Ctx | -> { c.y > 0 } })`)
///
/// # Returns
///
/// Returns a value implementing [`CvlrBoolExpr`](crate::CvlrBoolExpr) that evaluates to `true`
/// only when all input expressions evaluate to `true`.
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_and, cvlr_predicate, CvlrBoolExpr};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Using identifiers
/// cvlr_def_predicate! {
///     pred IsPositive(c: Counter) {
///         c.value > 0
///     }
/// }
///
/// cvlr_def_predicate! {
///     pred IsEven(c: Counter) {
///         c.value % 2 == 0
///     }
/// }
///
/// let ctx = Counter { value: 6 };
/// let expr = cvlr_and!(IsPositive, IsEven);
/// assert!(expr.eval(&ctx));
///
/// // Using expressions
/// let expr2 = cvlr_and!(
///     cvlr_predicate! { | c : Counter | -> { c.value > 0 } },
///     cvlr_predicate! { | c : Counter | -> { c.value < 100 } }
/// );
/// assert!(expr2.eval(&ctx));
///
/// // Using multiple arguments
/// let expr3 = cvlr_and!(
///     IsPositive,
///     IsEven,
///     cvlr_predicate! { | c : Counter | -> { c.value < 100 } }
/// );
/// assert!(expr3.eval(&ctx));
/// ```
#[macro_export]
macro_rules! cvlr_and {
    ($a:expr, $b:expr) => {
        $crate::cvlr_and($a, $b)
    };

    ($a:expr, $b:expr, $c:expr) => {
        $crate::cvlr_and($a, $crate::cvlr_and($b, $c))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::cvlr_and($a, $crate::cvlr_and($b, $crate::cvlr_and($c, $d)))
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::cvlr_and(
            $a,
            $crate::cvlr_and($b, $crate::cvlr_and($c, $crate::cvlr_and($d, $e))),
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        $crate::cvlr_and(
            $a,
            $crate::cvlr_and(
                $b,
                $crate::cvlr_and($c, $crate::cvlr_and($d, $crate::cvlr_and($e, $f))),
            ),
        )
    };
}

/// Creates a boolean expression representing logical implication (A → B).
///
/// This macro is a convenience wrapper around [`cvlr_implies`](crate::cvlr_implies) that
/// provides flexible syntax for creating implications. It supports both identifiers and expressions
/// as arguments.
///
/// An implication `A → B` evaluates to `true` when either:
/// - The antecedent `A` is `false`, or
/// - Both `A` and `B` are `true`
///
/// # Syntax
///
/// ```ignore
/// cvlr_implies!(antecedent, consequent)
/// ```
///
/// # Arguments
///
/// * `antecedent` - The left-hand side (A) of the implication, can be an identifier or expression
/// * `consequent` - The right-hand side (B) of the implication, can be an identifier or expression
///
/// Both arguments must implement [`CvlrBoolExpr`](crate::CvlrBoolExpr) with the same context type.
///
/// # Returns
///
/// Returns a value implementing [`CvlrBoolExpr`](crate::CvlrBoolExpr) that represents the logical
/// implication `antecedent → consequent`.
///
/// # Examples
///
/// ```ignore
/// use cvlr_spec::{cvlr_implies, cvlr_predicate, CvlrBoolExpr};
///
/// struct Counter {
///     value: i32,
/// }
///
/// // Using identifiers
/// cvlr_def_predicate! {
///     pred IsPositive(c: Counter) {
///         c.value > 0
///     }
/// }
///
/// cvlr_def_predicate! {
///     pred IsEven(c: Counter) {
///         c.value % 2 == 0
///     }
/// }
///
/// let ctx1 = Counter { value: 6 };
/// let expr = cvlr_implies!(IsPositive, IsEven);
/// assert!(expr.eval(&ctx1)); // 6 > 0 → 6 % 2 == 0 (both true, so true)
///
/// let ctx2 = Counter { value: 5 };
/// assert!(!expr.eval(&ctx2)); // 5 > 0 → 5 % 2 == 0 (antecedent true, consequent false, so false)
///
/// let ctx3 = Counter { value: -2 };
/// assert!(expr.eval(&ctx3)); // -2 > 0 → ... (antecedent false, so true)
///
/// // Using expressions
/// let expr2 = cvlr_implies!(
///     cvlr_predicate! { | c : Counter | -> { c.value > 0 } },
///     cvlr_predicate! { | c : Counter | -> { c.value < 100 } }
/// );
/// assert!(expr2.eval(&ctx1));
///
/// // Mixed identifiers and expressions
/// let expr3 = cvlr_implies!(
///     IsPositive,
///     cvlr_predicate! { | c : Counter | -> { c.value < 100 } }
/// );
/// assert!(expr3.eval(&ctx1));
/// ```
#[macro_export]
macro_rules! cvlr_implies {
    ($a:expr, $b:expr) => {
        $crate::cvlr_implies($a, $b)
    };
}
