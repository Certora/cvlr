//! Attributes that insert a call at the start
//! ([`macro@cvlr_hook_on_entry`]) or end ([`macro@cvlr_hook_on_exit`]) of a
//! function, for running spec code inside the code under verification. Use
//! them through the `cvlr` prelude.

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Meta, Token};

/// Inserts a call at the start of a function.
///
/// `#[cvlr_hook_on_entry(hook(args))]` makes `hook(args);` the first statement
/// of the annotated function. Use it to run specification code (logging,
/// assumptions, assertions, bookkeeping in ghost state) every time a function
/// of the code under verification is entered, typically behind the
/// verification feature:
/// `#[cfg_attr(feature = "certora", cvlr::hook_on_entry(spec::on_withdraw()))]`.
///
/// The argument must be exactly one call expression of the form
/// `path(tokens)`, such as `hook()` or `crate::spec::check(amount)`; it is
/// pasted into the function body, so it can refer to the function's
/// parameters. Also available as `hook_on_entry` in `cvlr`'s prelude.
///
/// # Example
///
/// ```ignore
/// #[cvlr_hook_on_entry(hook())]
/// fn t1() {
///     println!("t1");
/// }
///
/// // expands to
///
/// fn t1() {
///     hook();
///     println!("t1");
/// }
/// ```
#[proc_macro_attribute]
pub fn cvlr_hook_on_entry(attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse the attribute argument
    let attr = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    if attr.len() != 1 {
        return quote! {
            compile_error!("Expected 1 argument");
        }
        .into();
    }

    let arg = attr.first().unwrap();

    // parse the input tokens and make sure it is a function
    let mut fn_item = parse_macro_input!(input as ItemFn);

    // insert tokens_start into fn item statements at position 0
    let tokens_start: syn::Stmt = syn::parse_quote! { #arg; };

    fn_item.block.stmts.insert(0, tokens_start);

    fn_item.into_token_stream().into()
}

/// Inserts a call at the end of a function's body.
///
/// `#[cvlr_hook_on_exit(hook(args))]` adds the statement `hook(args);` at the
/// end of the annotated function's body. The argument has the same form as for
/// [`macro@cvlr_hook_on_entry`]. Also available as `hook_on_exit` in `cvlr`'s
/// prelude.
///
/// The insertion is purely syntactic, which limits what the hook observes:
///
/// * If the function has no return type, the hook is appended after the last
///   statement.
/// * If the function declares a return type, the hook is inserted *before*
///   the last statement, i.e. before the tail expression that computes the
///   return value is evaluated. The hook therefore does not see any effect of
///   that expression; if the result is computed by a call (`process(x)`), the
///   hook runs before the call.
/// * Exits through `return`, `?` or a panic do not run the hook.
///
/// # Example
///
/// ```ignore
/// #[cvlr_hook_on_exit(hook())]
/// fn t1() {
///     assert_eq!(1, 1);
///     assert_eq!(2, 2);
/// }
///
/// // expands to
///
/// fn t1() {
///     assert_eq!(1, 1);
///     assert_eq!(2, 2);
///     hook();
/// }
/// ```
#[proc_macro_attribute]
pub fn cvlr_hook_on_exit(attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse the attribute argument

    let attr = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);
    if attr.len() != 1 {
        return quote! {
            compile_error!("Expected 1 argument");
        }
        .into();
    }

    let arg = &attr.first().unwrap();

    // parse the input tokens and make sure it is a function
    let mut fn_item = parse_macro_input!(input as ItemFn);
    let ret_type = &fn_item.sig.output;

    // create tokens_end
    let stmt_end: syn::Stmt = syn::parse_quote! { #arg; };

    // len of fn item statements
    let len = fn_item.block.stmts.len();

    match ret_type {
        syn::ReturnType::Default => {
            // insert tokens_end into fn item statements at position len
            fn_item.block.stmts.insert(len, stmt_end);
        }
        syn::ReturnType::Type(_, _) => {
            // insert tokens_end into fn item statements at position len-1
            fn_item.block.stmts.insert(len - 1, stmt_end);
        }
    }

    fn_item.into_token_stream().into()
}
