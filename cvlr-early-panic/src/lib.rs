use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, parse_quote, Expr, ItemFn};

/// Replaces question mark operator by unwrap
struct EarlyPanic;

impl VisitMut for EarlyPanic {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Try(expr) = &mut *node {
            let prefix: &mut Expr = expr.expr.as_mut();
            // -- recurse on prefix since it might have nested q-mark
            visit_mut::visit_expr_mut(self, prefix);
            *node = parse_quote!(#prefix.unwrap());
            return;
        }

        // -- recurse on other expression types
        visit_mut::visit_expr_mut(self, node);
    }
}

/// Attribute to replace question mark operator by unwrap.
///
/// # Example
///
/// ```
/// use cvlr_early_panic::early_panic;
/// #[early_panic]
/// fn foo() -> Option<u64> {
///     let v = "42".parse::<u64>()?;
///     Some(v)
/// }
/// ```
#[proc_macro_attribute]
pub fn early_panic(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut fn_ast = parse_macro_input!(input as ItemFn);
    EarlyPanic.visit_item_fn_mut(&mut fn_ast);
    TokenStream::from(quote!(#fn_ast))
}
