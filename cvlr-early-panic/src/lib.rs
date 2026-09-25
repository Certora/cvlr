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
/// Turns error returns in the annotated function into panics, so that under
/// the Certora Prover the error paths end the execution instead of returning
/// an error to the caller. A panic stops the execution: no later assertion is
/// reached, so executions that take an error path cannot violate the
/// assertions that follow it, and rules effectively consider only executions
/// in which the function succeeds. Use it on code under verification (usually
/// behind the verification feature) to focus a rule on the success case, and
/// keep in mind that it hides error paths: a rule about "the function fails
/// when ..." cannot be checked through a function with this attribute. Also
/// available as `cvlr_early_panic` in `cvlr`'s prelude.
///
/// The rewriting, applied throughout the function body including nested
/// blocks and closures:
///
/// * every `expr?` becomes `expr.unwrap()`. For a `Result`, this requires the
///   error type to implement `Debug`, and no `From` conversion of the error
///   takes place;
/// * every statement `return Err(...)` and every statement consisting of an
///   `Err(...)` expression (including a block's tail expression) becomes
///   `panic!()`.
///
/// An `Err(...)` that is not a statement on its own, such as a match arm
/// `x => Err(e)` or `let r = Err(e);`, is left unchanged.
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
