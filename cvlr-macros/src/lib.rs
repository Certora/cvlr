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

#[proc_macro]
pub fn cvlr_assert_that(input: TokenStream) -> TokenStream {
    assert_that::assert_that_impl(input)
}
