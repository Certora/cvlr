use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, ItemFn};

mod assert_that;
mod mock;
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

/// Assert a condition using a DSL syntax
///
/// This macro provides a convenient DSL for writing assertions. It supports both
/// guarded (conditional) and unguarded assertions, and automatically detects
/// comparison operators to expand to the appropriate `cvlr_assert_*` macros.
///
/// # Syntax
///
/// The macro accepts either:
/// - **Unguarded expression**: `cvlr_assert_that!(condition)`
/// - **Guarded expression**: `cvlr_assert_that!(if guard { condition })`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ## Unguarded comparisons
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
/// ## Guarded comparisons
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let flag = true;
/// let a = 1;
/// let b = 2;
/// let x = 5;
/// let y = 10;
/// let z = 15;
///
/// // Assert b < a only if flag is true
/// cvlr_assert_that!(if flag { a < b });  // expands to cvlr_assert_lt_if!(flag, a, b)
/// cvlr_assert_that!(if x > 0 { y <= z }); // expands to cvlr_assert_le_if!(x > 0, y, z)
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
/// let z = 7;
///
/// // Unguarded boolean
/// cvlr_assert_that!(flag);                    // expands to cvlr_assert!(flag)
/// cvlr_assert_that!(x > 0 && y < 10);         // expands to cvlr_assert!(x > 0 && y < 10)
///
/// // Guarded boolean
/// cvlr_assert_that!(if flag { x > 0 });       // expands to cvlr_assert_if!(flag, x > 0)
/// cvlr_assert_that!(if x > 0 { y > 0 && z < 10 }); // expands to cvlr_assert_if!(x > 0, y > 0 && z < 10)
/// ```
///
/// ## Complex expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let a = 1;
/// let c = 3;
/// let d = 4;
/// let p = 5;
/// let x = 5;
/// let y = 3;
/// let z = 10;
///
/// // Complex guard and condition
/// cvlr_assert_that!(if a > c { d < p });      // expands to cvlr_assert_lt_if!(a > c, d, p)
/// cvlr_assert_that!(if x + 1 > 0 { y * 2 < z }); // expands to cvlr_assert_lt_if!(x + 1 > 0, y * 2, z)
/// ```
///
/// # Expansion
///
/// The macro automatically detects comparison operators and expands to the
/// appropriate assertion macro:
///
/// - Comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`) expand to `cvlr_assert_<op>!` or `cvlr_assert_<op>_if!`
/// - Boolean expressions expand to `cvlr_assert!` or `cvlr_assert_if!`
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
/// - Unguarded: `condition`
/// - Guarded: `if guard { condition }`
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_all;
///
/// let x = 5;
/// let y = 10;
/// let c = true;
///
/// // Multiple unguarded assertions
/// cvlr_assert_all!(x > 0, y < 20, x < y);
///
/// // Mixed guarded and unguarded
/// cvlr_assert_all!(x > 0, if c { x < y });
///
/// // Using semicolons
/// cvlr_assert_all!(x > 0; y < 20; if c { x < y });
///
/// // Mixed separators
/// cvlr_assert_all!(x > 0, y < 20; if c { x < y });
/// ```
///
/// # Expansion
///
/// This macro expands directly to the underlying assertion macros (not to `cvlr_assert_that!` calls):
///
/// ```text
/// // Input:
/// cvlr_assert_all!(x > 0, if c { x < y });
///
/// // Expands to:
/// ::cvlr::asserts::cvlr_assert_gt!(x, 0);
/// ::cvlr::asserts::cvlr_assert_lt_if!(c, x, y);
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
/// The macro accepts either:
/// - **Unguarded expression**: `cvlr_assume_that!(condition)`
/// - **Guarded expression**: `cvlr_assume_that!(if guard { condition })`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ## Unguarded comparisons
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
/// ## Guarded comparisons
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_that;
///
/// let flag = true;
/// let a = 1;
/// let b = 2;
///
/// // Assume a < b only if flag is true
/// cvlr_assume_that!(if flag { a < b });  // expands to: if flag { cvlr_assume_lt!(a, b); }
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
/// // Unguarded boolean
/// cvlr_assume_that!(flag);                    // expands to cvlr_assume!(flag)
/// cvlr_assume_that!(x > 0 && y < 10);         // expands to cvlr_assume!(x > 0 && y < 10)
///
/// // Guarded boolean
/// cvlr_assume_that!(if flag { x > 0 });       // expands to: if flag { cvlr_assume!(x > 0); }
/// ```
///
/// # Expansion
///
/// The macro automatically detects comparison operators and expands to the
/// appropriate assume macro:
///
/// - Comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`) expand to `cvlr_assume_<op>!`
/// - For guarded expressions, the assume is wrapped in an `if` block
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
/// - Unguarded: `condition`
/// - Guarded: `if guard { condition }`
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_all;
///
/// let x = 5;
/// let y = 10;
/// let c = true;
///
/// // Multiple unguarded assumptions
/// cvlr_assume_all!(x > 0, y < 20, x < y);
///
/// // Mixed guarded and unguarded
/// cvlr_assume_all!(x > 0, if c { x < y });
///
/// // Using semicolons
/// cvlr_assume_all!(x > 0; y < 20; if c { x < y });
/// ```
///
/// # Expansion
///
/// This macro expands directly to the underlying assume macros:
///
/// ```text
/// // Input:
/// cvlr_assume_all!(x > 0, if c { x < y });
///
/// // Expands to:
/// ::cvlr::asserts::cvlr_assume_gt!(x, 0);
/// if c {
///     ::cvlr::asserts::cvlr_assume_lt!(x, y);
/// }
/// ```
#[proc_macro]
pub fn cvlr_assume_all(input: TokenStream) -> TokenStream {
    assert_that::assume_all_impl(input)
}
