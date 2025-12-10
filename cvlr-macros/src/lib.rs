use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, ItemFn};

mod assert_that;
mod mock;
/// Mark a method as a CVT rule
///
/// # Example
///
/// ```
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
/// ```rust
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
/// ```rust
/// use cvlr_macros::cvlr_assert_that;
///
/// let flag = true;
/// let a = 1;
/// let b = 2;
///
/// // Assert b < a only if flag is true
/// cvlr_assert_that!(if flag { a < b });  // expands to cvlr_assert_lt_if!(flag, a, b)
/// cvlr_assert_that!(if x > 0 { y <= z }); // expands to cvlr_assert_le_if!(x > 0, y, z)
/// ```
///
/// ## Boolean expressions
///
/// ```rust
/// use cvlr_macros::cvlr_assert_that;
///
/// let flag = true;
/// let x = 5;
/// let y = 3;
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
/// ```rust
/// use cvlr_macros::cvlr_assert_that;
///
/// let a = 1;
/// let c = 3;
/// let d = 4;
/// let p = 5;
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
