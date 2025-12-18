use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Ident, ItemFn};

mod assert_that;
mod mock;
mod predicate;
mod rule_for_spec;
/// Mark a method as a CVT rule
///
/// # Example
///
/// ```rust,no_run
/// use cvlr::prelude::*;
/// #[rule]
/// fn foo()  {
///    cvlr_assert!(false);
/// }
/// ```
#[proc_macro_attribute]
pub fn rule(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_ast = parse_macro_input!(item as ItemFn);
    // add #[no_mangle] attribute
    fn_ast.attrs.push(parse_quote! { #[no_mangle] });
    // The first statement in rules is a call to the macro `cvlr_rule_location!`
    // to automatically insert the location of the rule.
    fn_ast
        .block
        .stmts
        .insert(0, parse_quote! { cvlr::log::cvlr_rule_location!(); });
    fn_ast
        .block
        .stmts
        .push(parse_quote! { cvlr::cvlr_vacuity_check!(); });
    fn_ast.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn mock_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    mock::mock_fn_impl(attr, item)
}

/// Converts a function into a CVLR predicate.
///
/// This attribute macro transforms a function into a struct that implements
/// [`CvlrFormula`](cvlr_spec::CvlrFormula) for the specified context type.
/// The function body is parsed and used to generate `eval`, `assert`, and `assume`
/// methods using the same helpers as [`cvlr_def_predicate!`](cvlr_spec::cvlr_def_predicate).
///
/// # Syntax
///
/// ```ignore
/// #[cvlr_predicate]
/// pub fn predicate_name(c: &Ctx) {
///     c.x > 0;
///     c.y < 100;
/// }
/// ```
///
/// # Parameters
///
/// * The function must have exactly one parameter of type `&Ctx` or `&mut Ctx`
/// * The function body can contain one or more expressions (statements ending with `;`)
/// * The function name will be converted from snake_case to PascalCase for the struct name
///
/// # Examples
///
/// ```ignore
/// use cvlr_macros::cvlr_predicate;
/// use cvlr_spec::CvlrFormula;
///
/// struct Ctx {
///     x: i32,
///     y: i32,
/// }
///
/// #[cvlr_predicate]
/// pub fn x_gt_zero(c: &Ctx) {
///     c.x > 0;
/// }
///
/// // This generates:
/// // pub struct XGtZero;
/// // impl CvlrFormula<Ctx> for XGtZero { ... }
///
/// let ctx = Ctx { x: 5, y: 10 };
/// let pred = XGtZero;
/// assert!(pred.eval(&ctx));
/// ```
///
/// # Generated Code
///
/// The macro generates a struct and implementation similar to [`cvlr_def_predicate!`](cvlr_spec::cvlr_def_predicate):
///
/// ```ignore
/// pub struct XGtZero;
/// impl CvlrFormula<Ctx> for XGtZero {
///     fn eval(&self, ctx: &Ctx) -> bool {
///         let c = ctx;
///         cvlr_eval_all!(c.x > 0)
///     }
///     fn assert(&self, ctx: &Ctx) {
///         let c = ctx;
///         cvlr_assert_all!(c.x > 0);
///     }
///     fn assume(&self, ctx: &Ctx) {
///         let c = ctx;
///         cvlr_assume_all!(c.x > 0);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn cvlr_predicate(attr: TokenStream, item: TokenStream) -> TokenStream {
    predicate::cvlr_predicate_impl(attr, item)
}

/// Assert a condition using a DSL syntax
///
/// This macro provides a convenient DSL for writing assertions and automatically detects
/// comparison operators to expand to the appropriate `cvlr_assert_*` macros.
///
/// # Syntax
///
/// The macro accepts:
/// - **Expression**: `cvlr_assert_that!(condition)`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ## Comparisons
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let x = 5;
/// let y = 10;
///
/// cvlr_assert_that!(x < y);        // expands to cvlr_assert_lt!(x, y)
/// cvlr_assert_that!(x <= y);       // expands to cvlr_assert_le!(x, y)
/// cvlr_assert_that!(x > 0);        // expands to cvlr_assert_gt!(x, 0)
/// cvlr_assert_that!(x >= 0);       // expands to cvlr_assert_ge!(x, 0)
/// cvlr_assert_that!(x == 5);       // expands to cvlr_assert_eq!(x, 5)
/// cvlr_assert_that!(x != 0);       // expands to cvlr_assert_ne!(x, 0)
/// ```
///
/// ## Boolean expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let flag = true;
/// let x = 5;
/// let y = 3;
///
/// cvlr_assert_that!(flag);                    // expands to cvlr_assert!(flag)
/// cvlr_assert_that!(x > 0 && y < 10);         // expands to cvlr_assert!(x > 0 && y < 10)
/// ```
///
/// ## Complex expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let a = 1;
/// let d = 4;
/// let p = 5;
/// let x = 5;
/// let y = 3;
/// let z = 10;
///
/// // Complex conditions
/// cvlr_assert_that!(a < d);                   // expands to cvlr_assert_lt!(a, d)
/// cvlr_assert_that!(x + 1 > 0 && y * 2 < z); // expands to cvlr_assert!(x + 1 > 0 && y * 2 < z)
/// ```
///
/// # Expansion
///
/// The macro automatically detects comparison operators and expands to the
/// appropriate assertion macro:
///
/// - Comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`) expand to `cvlr_assert_<op>!`
/// - Boolean expressions expand to `cvlr_assert!`
#[proc_macro]
pub fn cvlr_assert_that(input: TokenStream) -> TokenStream {
    assert_that::assert_that_impl(input)
}

/// Assert multiple conditions using the same DSL syntax as `cvlr_assert_that!`
///
/// This macro takes a list of DSL expressions (same syntax as `cvlr_assert_that!`)
/// and expands to multiple calls to `cvlr_assert_that!`. Expressions can be
/// separated by either commas (`,`) or semicolons (`;`).
///
/// # Syntax
///
/// Expressions can be separated by commas or semicolons:
/// - `cvlr_assert_all!(expr1, expr2, expr3);`
/// - `cvlr_assert_all!(expr1; expr2; expr3);`
/// - `cvlr_assert_all!(expr1, expr2; expr3);`  // Mixed separators are also allowed
///
/// Each expression follows the same syntax as `cvlr_assert_that!`:
/// - `condition`
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_all;
///
/// let x = 5;
/// let y = 10;
///
/// // Multiple assertions
/// cvlr_assert_all!(x > 0, y < 20, x < y);
///
/// // Using semicolons
/// cvlr_assert_all!(x > 0; y < 20; x < y);
///
/// // Mixed separators
/// cvlr_assert_all!(x > 0, y < 20; x < y);
/// ```
///
/// # Expansion
///
/// This macro expands directly to the underlying assertion macros (not to `cvlr_assert_that!` calls):
///
/// ```text
/// // Input:
/// cvlr_assert_all!(x > 0, x < y);
///
/// // Expands to:
/// ::cvlr::asserts::cvlr_assert_gt!(x, 0);
/// ::cvlr::asserts::cvlr_assert_lt!(x, y);
/// ```
#[proc_macro]
pub fn cvlr_assert_all(input: TokenStream) -> TokenStream {
    assert_that::assert_all_impl(input)
}

/// Assume a condition using a DSL syntax (analogous to `cvlr_assert_that!`)
///
/// This macro provides the same DSL syntax as `cvlr_assert_that!` but expands to
/// `cvlr_assume_*` macros instead of `cvlr_assert_*` macros.
///
/// # Syntax
///
/// The macro accepts:
/// - **Expression**: `cvlr_assume_that!(condition)`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ## Comparisons
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_that;
///
/// let x = 5;
/// let y = 10;
///
/// cvlr_assume_that!(x < y);        // expands to cvlr_assume_lt!(x, y)
/// cvlr_assume_that!(x <= y);       // expands to cvlr_assume_le!(x, y)
/// cvlr_assume_that!(x > 0);        // expands to cvlr_assume_gt!(x, 0)
/// cvlr_assume_that!(x >= 0);       // expands to cvlr_assume_ge!(x, 0)
/// cvlr_assume_that!(x == 5);       // expands to cvlr_assume_eq!(x, 5)
/// cvlr_assume_that!(x != 0);       // expands to cvlr_assume_ne!(x, 0)
/// ```
///
/// ## Boolean expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_that;
///
/// let flag = true;
/// let x = 5;
/// let y = 3;
///
/// cvlr_assume_that!(flag);                    // expands to cvlr_assume!(flag)
/// cvlr_assume_that!(x > 0 && y < 10);         // expands to cvlr_assume!(x > 0 && y < 10)
/// ```
///
/// # Expansion
///
/// The macro automatically detects comparison operators and expands to the
/// appropriate assume macro:
///
/// - Comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`) expand to `cvlr_assume_<op>!`
/// - Boolean expressions expand to `cvlr_assume!`
#[proc_macro]
pub fn cvlr_assume_that(input: TokenStream) -> TokenStream {
    assert_that::assume_that_impl(input)
}

/// Assume multiple conditions using the same DSL syntax as `cvlr_assume_that!`
///
/// This macro takes a list of DSL expressions (same syntax as `cvlr_assume_that!`)
/// and expands directly to the underlying `cvlr_assume_*` macros. Expressions can be
/// separated by either commas (`,`) or semicolons (`;`).
///
/// # Syntax
///
/// Expressions can be separated by commas or semicolons:
/// - `cvlr_assume_all!(expr1, expr2, expr3);`
/// - `cvlr_assume_all!(expr1; expr2; expr3);`
/// - `cvlr_assume_all!(expr1, expr2; expr3);`  // Mixed separators are also allowed
///
/// Each expression follows the same syntax as `cvlr_assume_that!`:
/// - `condition`
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_all;
///
/// let x = 5;
/// let y = 10;
///
/// // Multiple assumptions
/// cvlr_assume_all!(x > 0, y < 20, x < y);
///
/// // Using semicolons
/// cvlr_assume_all!(x > 0; y < 20; x < y);
/// ```
///
/// # Expansion
///
/// This macro expands directly to the underlying assume macros:
///
/// ```text
/// // Input:
/// cvlr_assume_all!(x > 0, x < y);
///
/// // Expands to:
/// ::cvlr::asserts::cvlr_assume_gt!(x, 0);
/// ::cvlr::asserts::cvlr_assume_lt!(x, y);
/// ```
#[proc_macro]
pub fn cvlr_assume_all(input: TokenStream) -> TokenStream {
    assert_that::assume_all_impl(input)
}

/// Evaluate a condition as a boolean expression using the same DSL syntax as `cvlr_assert_that!`
///
/// This macro provides the same DSL syntax as `cvlr_assert_that!` but instead of asserting,
/// it evaluates the condition as a boolean expression. The result is wrapped in its own scope.
///
/// # Syntax
///
/// The macro accepts:
/// - **Expression**: `cvlr_eval_that!(condition)`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_eval_that;
///
/// let x = 5;
/// let y = 10;
///
/// let result = cvlr_eval_that!(x < y);        // expands to: { x < y }
/// let flag = cvlr_eval_that!(x > 0 && y < 20); // expands to: { x > 0 && y < 20 }
/// ```
///
/// # Expansion
///
/// Expressions expand to `{ condition }`
#[proc_macro]
pub fn cvlr_eval_that(input: TokenStream) -> TokenStream {
    assert_that::eval_that_impl(input)
}

/// Evaluate multiple conditions as a boolean expression using the same DSL syntax as `cvlr_eval_that!`
///
/// This macro takes a list of DSL expressions (same syntax as `cvlr_eval_that!`)
/// and evaluates them all, aggregating the results with `&&`. The result is wrapped in its own scope.
/// Expressions can be separated by either commas (`,`) or semicolons (`;`).
///
/// # Syntax
///
/// Expressions can be separated by commas or semicolons:
/// - `cvlr_eval_all!(expr1, expr2, expr3);`
/// - `cvlr_eval_all!(expr1; expr2; expr3);`
/// - `cvlr_eval_all!(expr1, expr2; expr3);`  // Mixed separators are also allowed
///
/// Each expression follows the same syntax as `cvlr_eval_that!`:
/// - `condition`
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_eval_all;
///
/// let x = 5;
/// let y = 10;
///
/// // Multiple expressions
/// let result = cvlr_eval_all!(x > 0, y < 20, x < y);
///
/// // Using semicolons
/// let result = cvlr_eval_all!(x > 0; y < 20; x < y);
/// ```
///
/// # Expansion
///
/// This macro expands to a block that evaluates each expression and aggregates with `&&`:
///
/// ```text
/// // Input:
/// cvlr_eval_all!(x > 0, y > 0);
///
/// // Expands to:
/// {
///     let mut __cvlr_eval_all_res = true;
///     __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 };
///     __cvlr_eval_all_res = __cvlr_eval_all_res && { y > 0 };
///     __cvlr_eval_all_res
/// }
/// ```
#[proc_macro]
pub fn cvlr_eval_all(input: TokenStream) -> TokenStream {
    assert_that::eval_all_impl(input)
}

/// Generate a rule name and call `cvlr_impl_rule!` macro
///
/// This macro takes a name, spec expression, and base function identifier,
/// and generates a call to `cvlr_impl_rule!` with a combined rule name.
///
/// # Syntax
///
/// ```ignore
/// cvlr_rule_for_spec! {
///     name: "rule_name",
///     spec: expr,
///     base: base_function_name,
/// }
/// ```
///
/// # Parameters
///
/// * `name`: A string literal that will be converted to snake_case
/// * `spec`: An expression (the spec to use)
/// * `base`: An identifier of a function (if it starts with `base_`, that prefix is stripped)
///
/// # Examples
///
/// ```ignore
/// use cvlr_macros::cvlr_rule_for_spec;
///
/// cvlr_rule_for_spec! {
///     name: "solvency",
///     spec: expr,
///     base: base_update_exchange_price_no_interest_free_new,
/// }
///
/// // Expands to:
/// // cvlr_impl_rule!{solvency_update_exchange_price_no_interest_free_new, expr, base_update_exchange_price_no_interest_free_new}
/// ```
#[proc_macro]
pub fn cvlr_rule_for_spec(input: TokenStream) -> TokenStream {
    rule_for_spec::cvlr_rule_for_spec_impl(input)
}

/// Convert a snake_case identifier to PascalCase
///
/// This macro takes an identifier in snake_case and converts it to PascalCase,
/// outputting a new identifier.
///
/// # Syntax
///
/// ```ignore
/// cvlr_pif!(identifier_name)
/// ```
///
/// # Examples
///
/// The macro converts snake_case identifiers to PascalCase:
///
/// - `cvlr_pif!(my_struct)` expands to `MyStruct`
/// - `cvlr_pif!(x_gt_zero)` expands to `XGtZero`
/// - `cvlr_pif!(some_long_name)` expands to `SomeLongName`
#[proc_macro]
pub fn cvlr_pif(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    // Convert snake_case to PascalCase
    let pascal_case = predicate::to_pascal_case(&ident.to_string());

    // Create new identifier with the same span as the input
    let new_ident = Ident::new(&pascal_case, ident.span());

    quote! { #new_ident }.into()
}
