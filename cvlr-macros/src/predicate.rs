use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Expr, FnArg, ItemFn, Pat, PatType, Stmt, Type, TypeReference};

use crate::assert_that::{analyze_assume_condition, analyze_condition, analyze_eval_condition};

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

/// Separates let statements from expressions in function body statements
fn separate_statements(stmts: &[Stmt]) -> syn::Result<(Vec<&Stmt>, Vec<Expr>)> {
    let mut let_statements = Vec::new();
    let mut expressions = Vec::new();

    for stmt in stmts {
        match stmt {
            // Stmt::Local represents let statements
            Stmt::Local(_) => let_statements.push(stmt),
            // Stmt::Expr covers both expressions with and without semicolons
            Stmt::Expr(expr, _) => expressions.push(expr.clone()),
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

    // Validate function signature - must have exactly one parameter
    if fn_item.sig.inputs.len() != 1 {
        return syn::Error::new(
            Span::call_site(),
            "cvlr_predicate function must have exactly one parameter",
        )
        .to_compile_error()
        .into();
    }

    // Extract context type and parameter name
    let (ctx_type, param_name) = match extract_context_info(&fn_item.sig.inputs[0]) {
        Ok(info) => info,
        Err(e) => return e.to_compile_error().into(),
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

    // Generate eval expressions using analyze_eval_condition and combine with &&
    let mut eval_expressions = Vec::new();
    for expr in &expressions {
        match analyze_eval_condition(expr) {
            Ok(eval_expr) => eval_expressions.push(eval_expr),
            Err(e) => return e.to_compile_error().into(),
        }
    }

    // Build the eval block that accumulates results using shadowing (like eval_all_impl)
    let res_var = syn::Ident::new("__cvlr_eval_all_res", Span::call_site());
    let mut eval_statements = vec![quote! { let #res_var = true; }];
    for eval_expr in &eval_expressions {
        eval_statements.push(quote! {
            let #res_var = #res_var && #eval_expr;
        });
    }
    eval_statements.push(quote! { #res_var });

    // Generate the struct and impl, keeping the original function for IDE error checking
    let expanded = quote! {
        // Keep the original function so IDEs can report errors
        // But mark it dead code and unused must use to avoid warnings
        #[allow(unused_must_use, dead_code)]
        #fn_item

        #vis struct #struct_name;

        impl ::cvlr::spec::CvlrBoolExpr for #struct_name {

            type Context = #ctx_type;

            fn eval(&self, ctx: &Self::Context) -> bool {
                let #param_name = ctx;
                {
                    #(#let_statements)*
                    #(#eval_statements)*
                }
            }

            fn assert(&self, ctx: &Self::Context) {
                let #param_name = ctx;
                #(#let_statements)*
                #(#assert_statements)*
            }

            fn assume(&self, ctx: &Self::Context) {
                let #param_name = ctx;
                #(#let_statements)*
                #(#assume_statements)*
            }
        }
    };

    expanded.into()
}
