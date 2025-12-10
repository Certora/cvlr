use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Token};

// Custom parser for the assert_that DSL
struct AssertThatInput {
    guard: Option<Expr>,
    condition: Expr,
}

impl Parse for AssertThatInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Try to parse as: guard => condition
        let guard_expr: Expr = input.parse()?;

        // Check if next token is =>
        if input.peek(Token![=>]) {
            let _arrow: Token![=>] = input.parse()?;
            let condition: Expr = input.parse()?;
            Ok(AssertThatInput {
                guard: Some(guard_expr),
                condition,
            })
        } else {
            // No guard, the first expression is the condition
            Ok(AssertThatInput {
                guard: None,
                condition: guard_expr,
            })
        }
    }
}

pub fn assert_that_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertThatInput);

    // Analyze the condition to detect comparison operators
    let expanded = match analyze_condition(&input.condition, input.guard.as_ref()) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    };

    expanded.into()
}

fn analyze_condition(condition: &Expr, guard: Option<&Expr>) -> syn::Result<TokenStream2> {
    // Check if condition is a binary comparison
    if let Expr::Binary(bin) = condition {
        let op = &bin.op;
        let left = &bin.left;
        let right = &bin.right;

        // Determine the macro name based on the operator
        let macro_name = match op {
            syn::BinOp::Lt(_) => "cvlr_assert_lt",
            syn::BinOp::Le(_) => "cvlr_assert_le",
            syn::BinOp::Gt(_) => "cvlr_assert_gt",
            syn::BinOp::Ge(_) => "cvlr_assert_ge",
            syn::BinOp::Eq(_) => "cvlr_assert_eq",
            syn::BinOp::Ne(_) => "cvlr_assert_ne",
            _ => {
                // Not a comparison operator, treat as boolean expression
                return handle_boolean_condition(condition, guard);
            }
        };

        // Generate the macro call
        if let Some(guard_expr) = guard {
            // Guarded assertion: cvlr_assert_<op>_if!(guard, lhs, rhs)
            let macro_name_if = format!("{}_if", macro_name);
            let macro_ident = syn::Ident::new(&macro_name_if, Span::call_site());
            Ok(quote! {
                ::cvlr::asserts::#macro_ident!(#guard_expr, #left, #right);
            })
        } else {
            // Unguarded assertion: cvlr_assert_<op>!(lhs, rhs)
            let macro_ident = syn::Ident::new(macro_name, Span::call_site());
            Ok(quote! {
                ::clvr::asserts::#macro_ident!(#left, #right);
            })
        }
    } else {
        // Not a binary comparison, treat as boolean expression
        handle_boolean_condition(condition, guard)
    }
}

fn handle_boolean_condition(condition: &Expr, guard: Option<&Expr>) -> syn::Result<TokenStream2> {
    if let Some(guard_expr) = guard {
        // Guarded boolean: cvlr_assert_if!(guard, condition)
        Ok(quote! {
            ::cvrl::asserts::cvlr_assert_if!(#guard_expr, #condition);
        })
    } else {
        // Unguarded boolean: cvlr_assert!(condition)
        Ok(quote! {
            ::cvlr::asserts::cvlr_assert!(#condition);
        })
    }
}
