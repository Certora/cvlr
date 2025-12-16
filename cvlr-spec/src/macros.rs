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
        impl $crate::CvlrBoolExpr<$ctx> for $name {
            fn eval(&self, ctx: &$ctx) -> bool {
                let $c = ctx;
                $crate::__macro_support::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert(&self, ctx: &$ctx) {
                let $c = ctx;
                $crate::__macro_support::cvlr_assert_all!(
                    $($e),*
                );
            }

            fn assume(&self, ctx: &$ctx) {
                let $c = ctx;
                $crate::__macro_support::cvlr_assume_all!(
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
        impl $crate::CvlrBoolExpr<$crate::StatePair<'_, $ctx>> for $name {
            fn eval(&self, ctx: &$crate::StatePair<'_, $ctx>) -> bool {
                let $c = ctx.ctx();
                let $o = ctx.old();
                $crate::__macro_support::cvlr_eval_all!(
                    $($e),*
                )
            }
            fn assert(&self, ctx: &$crate::StatePair<'_, $ctx>) {
                let $c = ctx.ctx();
                let $o = ctx.old();
                $crate::__macro_support::cvlr_assert_all!(
                    $($e),*
                );
            }

            fn assume(&self, ctx: &$crate::StatePair<'_, $ctx>) {
                let $c = ctx.ctx();
                let $o = ctx.old();
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

// cvlr_lemma! {
//     UpdatePriceSolvencyLemma (c: UpdatePriceSolvencyCtx) {
//         requires -> {
//             c.supply_price.as_int().is_u64();
//             c.borrow_price.as_int().is_u64();
//             c.supply_price.as_int() >= EXCHANGE_PRICES_PRECISION as u64;
//             c.borrow_price.as_int() >= EXCHANGE_PRICES_PRECISION as u64;
//         }
//     }
//     ensures -> {
//         c.supply_delta <= c.borrow_delta
//     }
// }
#[macro_export]
macro_rules! cvlr_lemma {
    ($name: ident ( $c:ident : $ctx: ident ) {
        requires -> { $( $requires: expr );* $(;)? }
        ensures -> { $( $ensures: expr );* $(;)? } }) => {
            struct $name;
            impl $crate::spec::CvlrLemma<$ctx> for $name {
                fn requires(&self) -> impl $crate::CvlrBoolExpr<$ctx> {
                    $crate::cvlr_predicate! { | $c : $ctx | -> { $( $requires );* } }
                }
                fn ensures(&self) -> impl $crate::CvlrBoolExpr<$ctx> {
                    $crate::cvlr_predicate! { | $c : $ctx | -> { $( $ensures );* } }
                }
            }
        };
}
