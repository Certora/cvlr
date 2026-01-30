use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Expr, FnArg, ItemFn, Pat, PatType, Stmt, Type, TypeReference};

use crate::assert_that::{analyze_assume_condition, analyze_condition};

/// Converts a snake_case identifier to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Extracts the context type and parameter name from a function parameter
fn extract_context_info(arg: &FnArg) -> syn::Result<(Type, syn::Ident)> {
    match arg {
        FnArg::Receiver(_) => Err(syn::Error::new(
            Span::call_site(),
            "cvlr_predicate functions cannot have self parameter",
        )),
        FnArg::Typed(PatType { pat, ty, .. }) => {
            // Extract the parameter name
            let param_name = match pat.as_ref() {
                Pat::Ident(ident) => ident.ident.clone(),
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "cvlr_predicate parameter must be a simple identifier",
                    ));
                }
            };

            // Extract the context type from &Ctx or &mut Ctx
            let ctx_type = match ty.as_ref() {
                Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "cvlr_predicate parameter must be a reference type (e.g., &Ctx)",
                    ));
                }
            };

            Ok((ctx_type, param_name))
        }
    }
}

/// Returns true if the expression represents an empty statement (e.g. a bare `;`).
fn is_empty_expr(expr: &Expr) -> bool {
    if let Expr::Verbatim(ts) = expr {
        ts.is_empty()
    } else {
        false
    }
}

/// Separates let statements from expressions in function body statements.
/// Skips empty statements (bare `;`) which may be added by the cvlr_predicate! macro.
fn separate_statements(stmts: &[Stmt]) -> syn::Result<(Vec<&Stmt>, Vec<Expr>)> {
    let mut let_statements = Vec::new();
    let mut expressions = Vec::new();

    for stmt in stmts {
        match stmt {
            // Stmt::Local represents let statements
            Stmt::Local(_) => let_statements.push(stmt),
            // Stmt::Expr covers both expressions with and without semicolons
            Stmt::Expr(expr, _) => {
                if !is_empty_expr(expr) {
                    expressions.push(expr.clone());
                }
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "cvlr_predicate function body can only contain let statements and expressions",
                ));
            }
        }
    }

    Ok((let_statements, expressions))
}

pub fn cvlr_predicate_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);

    // Extract function name and convert to PascalCase for struct name
    let fn_name = &fn_item.sig.ident;
    let struct_name_str = to_pascal_case(&fn_name.to_string());
    let struct_name = syn::Ident::new(&struct_name_str, fn_name.span());

    // Extract function visibility
    let vis = &fn_item.vis;

    // Validate function signature - must have exactly one or two parameters
    let num_params = fn_item.sig.inputs.len();
    if num_params != 1 && num_params != 2 {
        return syn::Error::new(
            Span::call_site(),
            "cvlr_predicate function must have exactly one or two parameters",
        )
        .to_compile_error()
        .into();
    }

    // Extract context types and parameter names
    let (ctx_type, param1_name) = match extract_context_info(&fn_item.sig.inputs[0]) {
        Ok(info) => info,
        Err(e) => return e.to_compile_error().into(),
    };

    // For two-parameter case, extract second parameter and verify same context type
    let (param2_name, is_two_state) = if num_params == 2 {
        let (ctx_type2, param2) = match extract_context_info(&fn_item.sig.inputs[1]) {
            Ok(info) => info,
            Err(e) => return e.to_compile_error().into(),
        };

        // Verify both parameters have the same context type
        if ctx_type != ctx_type2 {
            return syn::Error::new(
                Span::call_site(),
                "cvlr_predicate function with two parameters must have the same context type for both",
            )
            .to_compile_error()
            .into();
        }

        (param2, true)
    } else {
        (syn::Ident::new("_unused", Span::call_site()), false)
    };

    // Separate let statements from expressions in function body
    let (let_statements, expressions) = match separate_statements(&fn_item.block.stmts) {
        Ok(result) => result,
        Err(e) => return e.to_compile_error().into(),
    };

    // Generate assert statements using analyze_condition
    let mut assert_statements = Vec::new();
    for expr in &expressions {
        match analyze_condition(expr) {
            Ok(assertion) => assert_statements.push(assertion),
            Err(e) => return e.to_compile_error().into(),
        }
    }

    // Generate assume statements using analyze_assume_condition
    let mut assume_statements = Vec::new();
    for expr in &expressions {
        match analyze_assume_condition(expr) {
            Ok(assumption) => assume_statements.push(assumption),
            Err(e) => return e.to_compile_error().into(),
        }
    }

    // Build the eval block with lazy evaluation using an accumulator variable
    let mut eval_statements = Vec::new();
    for expr in &expressions {
        eval_statements.push(quote! {
            __cvlr_eval_res = __cvlr_eval_res && { #expr };
        });
    }

    // If we get to the end, return eval result
    eval_statements.push(quote! { __cvlr_eval_res });

    // Generate the struct and impl, keeping the original function for IDE error checking
    let expanded = if is_two_state {
        // Two-parameter case: use _with_states methods
        // First parameter (param1_name) = post-state (ctx0)
        // Second parameter (param2_name) = pre-state (ctx1)
        quote! {
            // Keep the original function so IDEs can report errors
            // But mark it dead code and unused must use to avoid warnings
            #[allow(unused_must_use, dead_code)]
            #fn_item

            #vis struct #struct_name;

            impl ::cvlr::spec::CvlrFormula for #struct_name {

                type Context = #ctx_type;

                fn eval_with_states(&self, ctx0: &Self::Context, ctx1: &Self::Context) -> bool {
                    let #param1_name = ctx0;
                    let #param2_name = ctx1;
                    {
                        #(#let_statements)*
                        let mut __cvlr_eval_res = true;
                        #(#eval_statements)*
                    }
                }

                fn assert_with_states(&self, ctx0: &Self::Context, ctx1: &Self::Context) {
                    let #param1_name = ctx0;
                    let #param2_name = ctx1;
                    #(#let_statements)*
                    #(#assert_statements)*
                }

                fn assume_with_states(&self, ctx0: &Self::Context, ctx1: &Self::Context) {
                    let #param1_name = ctx0;
                    let #param2_name = ctx1;
                    #(#let_statements)*
                    #(#assume_statements)*
                }

                fn eval(&self, _ctx: &Self::Context) -> bool {
                    ::cvlr::asserts::cvlr_assert!(false);
                    panic!("eval should never be called for a two-state predicate; use eval_with_states instead");
                }

                fn assert(&self, _ctx: &Self::Context) {
                    ::cvlr::asserts::cvlr_assert!(false);
                    panic!("assert should never be called for a two-state predicate; use assert_with_states instead");
                }

                fn assume(&self, _ctx: &Self::Context) {
                    ::cvlr::asserts::cvlr_assert!(false);
                    panic!("assume should never be called for a two-state predicate; use assume_with_states instead");
                }
            }

            impl ::cvlr::spec::CvlrPredicate for #struct_name { }
        }
    } else {
        // Single-parameter case: use single-state methods
        quote! {
            // Keep the original function so IDEs can report errors
            // But mark it dead code and unused must use to avoid warnings
            #[allow(unused_must_use, dead_code)]
            #fn_item

            #vis struct #struct_name;

            impl ::cvlr::spec::CvlrFormula for #struct_name {

                type Context = #ctx_type;

                fn eval(&self, ctx: &Self::Context) -> bool {
                    let #param1_name = ctx;
                    {
                        #(#let_statements)*
                        let mut __cvlr_eval_res = true;
                        #(#eval_statements)*
                    }
                }

                fn assert(&self, ctx: &Self::Context) {
                    let #param1_name = ctx;
                    #(#let_statements)*
                    #(#assert_statements)*
                }

                fn assume(&self, ctx: &Self::Context) {
                    let #param1_name = ctx;
                    #(#let_statements)*
                    #(#assume_statements)*
                }
            }

            impl ::cvlr::spec::CvlrPredicate for #struct_name { }
        }
    };

    expanded.into()
}
