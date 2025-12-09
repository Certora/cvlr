use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse_macro_input,
        Data::{Enum, Struct, Union},
        DeriveInput,
        Fields::{self, Named, Unnamed},
        FieldsNamed, FieldsUnnamed, Ident,
    },
};

fn of_named_fields(n: &Ident, named_fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let initialize = named_fields.named.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! {
            #name: ::cvlr::nondet::nondet(),
        }
    });

    quote! {
        #n {
            #( #initialize )*
        }
    }
}

fn of_unnamed_fields(n: &Ident, unnamed: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let initialize = unnamed.unnamed.iter().map(|_| {
        quote! { ::cvlr::nondet::nondet(), }
    });

    quote! {
        #n (
            #( #initialize )*
        )
    }
}

/// Derive macro for implementing the `Nondet` trait
///
/// This macro generates an implementation of `Nondet` for structs,
/// allowing them to be created with non-deterministic (symbolic) values.
///
/// # Example
///
/// ```ignore
/// use cvlr_derive::Nondet;
/// use cvlr::prelude::*;
///
/// #[derive(Nondet)]
/// struct Point {
///     x: u64,
///     y: u64,
/// }
///
/// let p = Point::nondet();
/// ```
#[proc_macro_derive(Nondet)]
pub fn derive_nondet(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    match input.data {
        Enum(_) => {
            todo!("Enum not supported yet")
        }

        Union(_) => {
            todo!("Union not supported yet")
        }

        Struct(ds) => match ds.fields {
            Fields::Unit => quote! {
                impl ::cvlr::nondet::Nondet for #name {
                    fn nondet() -> #name {
                        #name
                    }
                }
            }
            .into(),

            Named(named) => {
                let init = of_named_fields(&name, &named);
                quote! {
                    impl ::cvlr::nondet::Nondet for #name {
                        fn nondet() -> #name {
                            #init
                        }
                    }
                }
                .into()
            }

            Unnamed(fields) => {
                let init = of_unnamed_fields(&name, &fields);
                quote! {
                    impl ::cvlr::nondet::Nondet for #name {
                        fn nondet() -> #name {
                            #init
                        }
                    }
                }
                .into()
            }
        },
    }
}
