use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse_macro_input,
        Data::{Enum, Struct, Union},
        DeriveInput,
        Fields::{self, Named, Unnamed},
        FieldsNamed, FieldsUnnamed, Ident, Index, Variant,
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

fn of_enum_variant(variant: &Variant, enum_name: &Ident) -> proc_macro2::TokenStream {
    let variant_name = &variant.ident;
    match &variant.fields {
        Fields::Unit => quote! {
            #enum_name::#variant_name
        },
        Fields::Unnamed(unnamed) => {
            let initialize = unnamed.unnamed.iter().map(|_| {
                quote! { ::cvlr::nondet::nondet(), }
            });
            quote! {
                #enum_name::#variant_name(
                    #( #initialize )*
                )
            }
        }
        Fields::Named(named) => {
            let initialize = named.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! {
                    #field_name: ::cvlr::nondet::nondet(),
                }
            });
            quote! {
                #enum_name::#variant_name {
                    #( #initialize )*
                }
            }
        }
    }
}

/// Derive macro for implementing the `Nondet` trait
///
/// This macro generates an implementation of `Nondet` for structs and enums,
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
/// #[derive(Nondet)]
/// enum MyEnum {
///     Variant1,
///     Variant2(u64),
///     Variant3 { x: u64 },
/// }
///
/// let p = Point::nondet();
/// let e = MyEnum::nondet();
/// ```
#[proc_macro_derive(Nondet)]
pub fn derive_nondet(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    match input.data {
        Enum(data_enum) => {
            let variants = &data_enum.variants;
            let variant_count = variants.len();

            if variant_count == 0 {
                return quote! {
                    compile_error!("Enum must have at least one variant");
                }
                .into();
            }

            let mut match_arms = Vec::new();
            for (index, variant) in variants.iter().enumerate() {
                let variant_expr = of_enum_variant(variant, &name);
                if index == variant_count - 1 {
                    // Last variant is catch-all
                    match_arms.push(quote! {
                        _ => #variant_expr,
                    });
                } else {
                    let index_lit = index as u64;
                    match_arms.push(quote! {
                        #index_lit => #variant_expr,
                    });
                }
            }

            quote! {
                impl ::cvlr::nondet::Nondet for #name {
                    fn nondet() -> #name {
                        match ::cvlr::nondet::nondet::<u64>() {
                            #( #match_arms )*
                        }
                    }
                }
            }
            .into()
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

/// Derive macro for implementing the `CvlrLog` trait
///
/// This macro generates an implementation of `CvlrLog` for structs,
/// allowing them to be logged with CVLR's logging system.
///
/// Supports:
/// - Structs with named fields (uses field names as tags)
/// - Structs with unnamed fields (uses field indices "0", "1", "2", etc. as tags)
/// - Unit structs (empty scope)
///
/// # Example
///
/// ```ignore
/// use cvlr_derive::CvlrLog;
/// use cvlr::log::CvlrLog;
///
/// #[derive(CvlrLog)]
/// struct Point {
///     x: u64,
///     y: u64,
/// }
///
/// #[derive(CvlrLog)]
/// struct Tuple(u64, i32);
///
/// let p = Point { x: 1, y: 2 };
/// p.log("point", &mut logger);
///
/// let t = Tuple(1, -2);
/// t.log("tuple", &mut logger);
/// ```
#[proc_macro_derive(CvlrLog)]
pub fn derive_cvlr_log(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    
    match input.data {
        Enum(_) => {
            quote! {
                compile_error!("CvlrLog derive is only supported for structs");
            }
            .into()
        }
        
        Union(_) => {
            quote! {
                compile_error!("CvlrLog derive is only supported for structs");
            }
            .into()
        }
        
        Struct(ds) => match ds.fields {
            Fields::Unit => {
                quote! {
                    impl ::cvlr::log::CvlrLog for #name {
                        fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
                            logger.log_scope_start(tag);
                            logger.log_scope_end(tag);
                        }
                    }
                }
                .into()
            }
            
            Fields::Unnamed(unnamed) => {
                let field_logs: Vec<_> = unnamed.unnamed.iter().enumerate().map(|(index, _f)| {
                    let field_index = Index::from(index);
                    let field_index_str = index.to_string();
                    quote! {
                        ::cvlr::log::cvlr_log_with(#field_index_str, &self.#field_index, logger);
                    }
                }).collect();
                
                quote! {
                    impl ::cvlr::log::CvlrLog for #name {
                        fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
                            logger.log_scope_start(tag);
                            #( #field_logs )*
                            logger.log_scope_end(tag);
                        }
                    }
                }
                .into()
            }
            
            Fields::Named(named) => {
                let field_logs: Vec<_> = named.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_name_str = field_name.to_string();
                    quote! {
                        ::cvlr::log::cvlr_log_with(#field_name_str, &self.#field_name, logger);
                    }
                }).collect();
                
                quote! {
                    impl ::cvlr::log::CvlrLog for #name {
                        fn log(&self, tag: &str, logger: &mut ::cvlr::log::CvlrLogger) {
                            logger.log_scope_start(tag);
                            #( #field_logs )*
                            logger.log_scope_end(tag);
                        }
                    }
                }
                .into()
            }
        },
    }
}
