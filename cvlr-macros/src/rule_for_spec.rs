use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Ident, LitStr, Token};

/// Parser for the cvlr_rule_for_spec macro input
struct RuleForSpecInput {
    name: LitStr,
    spec: Expr,
    base: Ident,
}

impl Parse for RuleForSpecInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse struct-like syntax: name: "...", spec: expr, base: ident
        let name_ident: Ident = input.parse()?;
        if name_ident.to_string() != "name" {
            return Err(syn::Error::new(
                name_ident.span(),
                "expected `name` field",
            ));
        }
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        let spec_ident: Ident = input.parse()?;
        if spec_ident.to_string() != "spec" {
            return Err(syn::Error::new(
                spec_ident.span(),
                "expected `spec` field",
            ));
        }
        input.parse::<Token![:]>()?;
        let spec: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        let base_ident: Ident = input.parse()?;
        if base_ident.to_string() != "base" {
            return Err(syn::Error::new(
                base_ident.span(),
                "expected `base` field",
            ));
        }
        input.parse::<Token![:]>()?;
        let base: Ident = input.parse()?;

        // Optional trailing comma
        let _ = input.parse::<Token![,]>();

        Ok(RuleForSpecInput { name, spec, base })
    }
}

/// Converts a string to snake_case
fn to_snake_case(s: &str) -> String {
    // For now, we'll just lowercase and replace spaces/hyphens with underscores
    // If the input is already in a different case, we'll handle it
    s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_whitespace() || c == '-' {
                '_'
            } else if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

pub fn cvlr_rule_for_spec_impl(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as RuleForSpecInput);

    // Convert name string literal to snake_case
    let name_str = parsed.name.value();
    let name_snake = to_snake_case(&name_str);

    // Get base identifier as string and strip "base_" prefix if present
    let base_str = parsed.base.to_string();
    let base_stripped = if base_str.starts_with("base_") {
        base_str.strip_prefix("base_").unwrap().to_string()
    } else {
        base_str
    };

    // Combine name and base to create rule_name
    let rule_name_str = if base_stripped.is_empty() {
        name_snake
    } else {
        format!("{}_{}", name_snake, base_stripped)
    };
    let rule_name = Ident::new(&rule_name_str, Span::call_site());

    // Extract fields for use in quote
    let spec = &parsed.spec;
    let base = &parsed.base;

    // Generate the macro call
    let expanded = quote! {
        cvlr_impl_rule!{#rule_name, #spec, #base}
    };

    expanded.into()
}

