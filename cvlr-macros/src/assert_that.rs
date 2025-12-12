use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, ExprIf, Token};

// Custom parser for the assert_that DSL
struct AssertThatInput {
    guard: Option<Expr>,
    condition: Expr,
}

impl Parse for AssertThatInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Try to parse as: if guard { condition }
        if input.peek(Token![if]) {
            let if_expr: ExprIf = input.parse()?;
            // Extract guard and condition from if expression
            let guard = *if_expr.cond;
            // The condition should be a single expression in the block
            if if_expr.then_branch.stmts.len() != 1 {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "expected exactly one statement in if block",
                ));
            }
            // Extract the expression from the statement
            let condition = match &if_expr.then_branch.stmts[0] {
                syn::Stmt::Expr(expr, _) => expr.clone(),
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "expected an expression, not a statement",
                    ));
                }
            };
            Ok(AssertThatInput {
                guard: Some(guard),
                condition,
            })
        } else {
            // No guard, parse as unguarded condition
            let condition: Expr = input.parse()?;
            Ok(AssertThatInput {
                guard: None,
                condition,
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
                ::cvlr::asserts::#macro_ident!(#left, #right);
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
            ::cvlr::asserts::cvlr_assert_if!(#guard_expr, #condition);
        })
    } else {
        // Unguarded boolean: cvlr_assert!(condition)
        Ok(quote! {
            ::cvlr::asserts::cvlr_assert!(#condition);
        })
    }
}

// Parser for a list of AssertThatInput expressions separated by comma or semicolon
struct AssertAllInput {
    expressions: Vec<AssertThatInput>,
}

impl Parse for AssertAllInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut expressions = Vec::new();

        loop {
            // Parse one expression
            let expr: AssertThatInput = input.parse()?;
            expressions.push(expr);

            // Check for separator (comma or semicolon)
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else if input.peek(Token![;]) {
                let _: Token![;] = input.parse()?;
            } else {
                // No more separators, we're done
                break;
            }

            // Check if we're at the end
            if input.is_empty() {
                break;
            }
        }

        Ok(AssertAllInput { expressions })
    }
}

pub fn assert_all_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertAllInput);

    // Generate the underlying assertion macros directly for each expression
    let mut assertions = Vec::new();

    for expr in &input.expressions {
        match analyze_condition(&expr.condition, expr.guard.as_ref()) {
            Ok(assertion) => assertions.push(assertion),
            // stop on first error
            Err(e) => return e.to_compile_error().into(),
        }
    }

    quote! {
        #(#assertions)*
    }
    .into()
}

fn analyze_assume_condition(condition: &Expr, guard: Option<&Expr>) -> syn::Result<TokenStream2> {
    // Check if condition is a binary comparison
    if let Expr::Binary(bin) = condition {
        let op = &bin.op;
        let left = &bin.left;
        let right = &bin.right;

        // Determine the macro name based on the operator
        let macro_name = match op {
            syn::BinOp::Lt(_) => "cvlr_assume_lt",
            syn::BinOp::Le(_) => "cvlr_assume_le",
            syn::BinOp::Gt(_) => "cvlr_assume_gt",
            syn::BinOp::Ge(_) => "cvlr_assume_ge",
            syn::BinOp::Eq(_) => "cvlr_assume_eq",
            syn::BinOp::Ne(_) => "cvlr_assume_ne",
            _ => {
                // Not a comparison operator, treat as boolean expression
                return handle_assume_boolean_condition(condition, guard);
            }
        };

        // Generate the macro call
        if let Some(guard_expr) = guard {
            // Guarded assumption: if guard { cvlr_assume_<op>!(lhs, rhs) }
            // Note: There are no cvlr_assume_<op>_if macros, so we wrap in if
            let macro_ident = syn::Ident::new(macro_name, Span::call_site());
            Ok(quote! {
                if #guard_expr {
                    ::cvlr::asserts::#macro_ident!(#left, #right);
                }
            })
        } else {
            // Unguarded assumption: cvlr_assume_<op>!(lhs, rhs)
            let macro_ident = syn::Ident::new(macro_name, Span::call_site());
            Ok(quote! {
                ::cvlr::asserts::#macro_ident!(#left, #right);
            })
        }
    } else {
        // Not a binary comparison, treat as boolean expression
        handle_assume_boolean_condition(condition, guard)
    }
}

fn handle_assume_boolean_condition(
    condition: &Expr,
    guard: Option<&Expr>,
) -> syn::Result<TokenStream2> {
    if let Some(guard_expr) = guard {
        // Guarded boolean: if guard { cvlr_assume!(condition) }
        Ok(quote! {
            if #guard_expr {
                ::cvlr::asserts::cvlr_assume!(#condition);
            }
        })
    } else {
        // Unguarded boolean: cvlr_assume!(condition)
        Ok(quote! {
            ::cvlr::asserts::cvlr_assume!(#condition);
        })
    }
}

pub fn assume_that_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertThatInput);

    // Analyze the condition to detect comparison operators
    let expanded = match analyze_assume_condition(&input.condition, input.guard.as_ref()) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    };

    expanded.into()
}

pub fn assume_all_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertAllInput);

    // Generate the underlying assume macros directly for each expression
    let mut assumptions = Vec::new();

    for expr in &input.expressions {
        match analyze_assume_condition(&expr.condition, expr.guard.as_ref()) {
            Ok(assumption) => assumptions.push(assumption),
            // stop on first error
            Err(e) => return e.to_compile_error().into(),
        }
    }

    quote! {
        #(#assumptions)*
    }
    .into()
}

fn analyze_eval_condition(condition: &Expr, guard: Option<&Expr>) -> syn::Result<TokenStream2> {
    if let Some(guard_expr) = guard {
        // Guarded expression: if guard { condition } else { true }
        Ok(quote! {
            {
                if #guard_expr {
                    #condition
                } else {
                    true
                }
            }
        })
    } else {
        // Unguarded expression: { condition }
        Ok(quote! {
            {
                #condition
            }
        })
    }
}

pub fn eval_that_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertThatInput);

    // Generate the boolean expression wrapped in a scope
    let expanded = match analyze_eval_condition(&input.condition, input.guard.as_ref()) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    };

    expanded.into()
}
