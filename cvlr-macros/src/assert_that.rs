use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Lit, Token};

// Custom parser for the assert_that DSL
struct AssertThatInput {
    condition: Expr,
}

impl Parse for AssertThatInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse as a condition expression
        let condition: Expr = input.parse()?;
        Ok(AssertThatInput { condition })
    }
}

pub fn assert_that_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertThatInput);

    // Analyze the condition to detect comparison operators
    let expanded = match analyze_condition(&input.condition) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    };

    expanded.into()
}

// Helper function to unwrap Expr::Group and Expr::Paren expressions
fn unwrap_groups(expr: &Expr) -> &Expr {
    match expr {
        Expr::Group(group) => unwrap_groups(&group.expr),
        Expr::Paren(paren) => unwrap_groups(&paren.expr),
        _ => expr,
    }
}

pub fn analyze_condition(condition: &Expr) -> syn::Result<TokenStream2> {
    // Unwrap any groups first
    let condition = unwrap_groups(condition);
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
                return handle_boolean_condition(condition);
            }
        };

        // Generate the macro call: cvlr_assert_<op>!(lhs, rhs)
        let macro_ident = syn::Ident::new(macro_name, Span::call_site());
        Ok(quote! {
            ::cvlr::asserts::#macro_ident!(#left, #right);
        })
    } else {
        // Not a binary comparison, treat as boolean expression
        handle_boolean_condition(condition)
    }
}

fn handle_boolean_condition(condition: &Expr) -> syn::Result<TokenStream2> {
    handle_boolean_condition_with_macro(condition, "cvlr_assert")
}

fn handle_boolean_condition_with_macro(
    condition: &Expr,
    macro_name: &str,
) -> syn::Result<TokenStream2> {
    // Check if condition is literal `true`
    if let Expr::Lit(lit) = condition {
        if let Lit::Bool(lit_bool) = &lit.lit {
            if lit_bool.value {
                // If condition is `true`, output unit `()`
                return Ok(quote! { () });
            }
        }
    }

    // Check if condition is an if expression
    if let Expr::If(if_expr) = condition {
        let guard = &if_expr.cond;

        // Process the then branch
        let then_branch = if if_expr.then_branch.stmts.len() == 1 {
            // Extract expression from single statement
            match &if_expr.then_branch.stmts[0] {
                syn::Stmt::Expr(expr, _) => {
                    // Recursively handle the condition in the then branch
                    handle_boolean_condition_with_macro(expr, macro_name)?
                }
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "expected an expression in if then branch",
                    ));
                }
            }
        } else {
            return Err(syn::Error::new(
                Span::call_site(),
                "expected exactly one statement in if then branch",
            ));
        };

        // Process the else branch if present
        let else_branch = if let Some((_, else_expr)) = &if_expr.else_branch {
            match else_expr.as_ref() {
                Expr::Block(block) => {
                    if block.block.stmts.len() == 1 {
                        match &block.block.stmts[0] {
                            syn::Stmt::Expr(expr, _) => {
                                // Recursively handle the condition in the else branch
                                Some(handle_boolean_condition_with_macro(expr, macro_name)?)
                            }
                            _ => {
                                return Err(syn::Error::new(
                                    Span::call_site(),
                                    "expected an expression in if else branch",
                                ));
                            }
                        }
                    } else {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "expected exactly one statement in if else branch",
                        ));
                    }
                }
                expr => {
                    // Direct expression in else branch
                    Some(handle_boolean_condition_with_macro(expr, macro_name)?)
                }
            }
        } else {
            None
        };

        // Generate if-else with macro calls in branches
        if let Some(else_branch_expr) = else_branch {
            Ok(quote! {
                if #guard {
                    #then_branch
                } else {
                    #else_branch_expr
                }
            })
        } else {
            // No else branch, just then branch
            Ok(quote! {
                if #guard {
                    #then_branch
                }
            })
        }
    } else {
        // Regular condition - generate macro call
        let macro_ident = syn::Ident::new(macro_name, Span::call_site());
        Ok(quote! {
            ::cvlr::asserts::#macro_ident!(#condition);
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
        match analyze_condition(&expr.condition) {
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

pub fn analyze_assume_condition(condition: &Expr) -> syn::Result<TokenStream2> {
    // Unwrap any groups first
    let condition = unwrap_groups(condition);

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
                return handle_assume_boolean_condition(condition);
            }
        };

        // Generate the macro call: cvlr_assume_<op>!(lhs, rhs)
        let macro_ident = syn::Ident::new(macro_name, Span::call_site());
        Ok(quote! {
            ::cvlr::asserts::#macro_ident!(#left, #right);
        })
    } else {
        // Not a binary comparison, treat as boolean expression
        handle_assume_boolean_condition(condition)
    }
}

fn handle_assume_boolean_condition(condition: &Expr) -> syn::Result<TokenStream2> {
    handle_boolean_condition_with_macro(condition, "cvlr_assume")
}

pub fn assume_that_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertThatInput);

    // Analyze the condition to detect comparison operators
    let expanded = match analyze_assume_condition(&input.condition) {
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
        match analyze_assume_condition(&expr.condition) {
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

pub fn analyze_eval_condition(condition: &Expr) -> syn::Result<TokenStream2> {
    // Expression: { condition }
    Ok(quote! {
        {
            #condition
        }
    })
}

pub fn eval_that_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertThatInput);

    // Generate the boolean expression wrapped in a scope
    let expanded = match analyze_eval_condition(&input.condition) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    };

    expanded.into()
}

pub fn eval_all_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertAllInput);

    // Generate evaluated expressions for each input
    let mut evaluated_exprs = Vec::new();

    for expr in &input.expressions {
        match analyze_eval_condition(&expr.condition) {
            Ok(eval_expr) => evaluated_exprs.push(eval_expr),
            // stop on first error
            Err(e) => return e.to_compile_error().into(),
        }
    }

    // Create the accumulator variable name
    let res_var = syn::Ident::new("__cvlr_eval_all_res", Span::call_site());

    // Build the block that accumulates results using shadowing
    // Start with initial value
    let mut statements = vec![quote! { let #res_var = true; }];

    // Add accumulation statements for each expression
    for eval_expr in &evaluated_exprs {
        statements.push(quote! {
            let #res_var = #res_var && #eval_expr;
        });
    }

    // Add final return
    statements.push(quote! { #res_var });

    quote! {
        {
            #(#statements)*
        }
    }
    .into()
}
