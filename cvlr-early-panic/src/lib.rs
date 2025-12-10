use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, parse_quote, Expr, ItemFn, Stmt};

/// Replaces question mark operator by unwrap
struct EarlyPanic;

impl EarlyPanic {
    /// Check if an expression is `Err(...)`
    fn is_err_expr(expr: &Expr) -> bool {
        match expr {
            Expr::Call(call) => {
                // Check if the function is `Err` or `Result::Err` or `std::result::Result::Err`
                match &*call.func {
                    Expr::Path(path_expr) => {
                        let path = &path_expr.path;
                        // Check if the last segment is "Err"
                        path.segments
                            .last()
                            .map(|seg| seg.ident == "Err")
                            .unwrap_or(false)
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

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

    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        match node {
            Stmt::Expr(expr, _) => {
                // Recurse first to handle nested expressions (including ? operators)
                visit_mut::visit_expr_mut(self, expr);

                // Check if this expression should panic
                // Use a temporary immutable reference for pattern matching
                let is_return_err = {
                    match expr {
                        Expr::Return(ret_expr) => {
                            ret_expr.expr.as_ref().is_some_and(|v| Self::is_err_expr(v))
                        }
                        _ => false,
                    }
                };

                let is_bare_err = { !matches!(*expr, Expr::Return(_)) && Self::is_err_expr(expr) };

                if is_return_err || is_bare_err {
                    *node = parse_quote!(panic!(););
                }
            }
            _ => {
                // Recurse on other statement types
                visit_mut::visit_stmt_mut(self, node);
            }
        }
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
