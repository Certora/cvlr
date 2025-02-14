use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Path};

fn default_when_feature() -> String {
    "certora".into()
}

#[derive(Debug, FromMeta)]
struct MockFnArgs {
    #[darling(default = "default_when_feature")]
    when: String,
    with: Path,
}

pub fn mock_fn_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return darling::Error::from(e).write_errors().into();
        }
    };

    let args = match MockFnArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let when = args.when;
    let mock_fn = args.with;

    let fn_ast = parse_macro_input!(item as ItemFn);

    let vis = fn_ast.vis.clone();
    let ident = fn_ast.sig.ident.clone();

    let tks = quote! {

        #[cfg(not(feature = #when))]
        #fn_ast

        #[cfg(feature = #when)]
        #vis use #mock_fn as #ident;
    };
    tks.into()
}
