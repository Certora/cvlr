use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Meta, Token};

/**
* This macro is used to insert a hook at the start of a function.
* # Example
* #[cvlr_hook_on_entry(hook())]
   fn t1() {
       // hook inserted here
       println!("t1");
   }

   expands to

   fn t1() {
       hook();
       println!("t1");
   }
*/
#[proc_macro_attribute]
pub fn cvlr_hook_on_entry(attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse the attribute argument
    let attr = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    if attr.len() != 1 {
        return quote! {
            compile_error!("Expected 1 argument");
        }
        .into();
    }

    let arg = attr.first().unwrap();

    // parse the input tokens and make sure it is a function
    let mut fn_item = parse_macro_input!(input as ItemFn);

    // insert tokens_start into fn item statements at position 0
    let tokens_start: syn::Stmt = syn::parse_quote! { #arg; };

    fn_item
        .block
        .stmts
        .insert(0, tokens_start);

    fn_item.into_token_stream().into()
}

/**
* This macro is used to insert a hook at the end of a function.
* If the function returns a value, the hook is inserted before the return statement.
* # Example
* #[cvlr_hook_on_exit(hook())]
   fn t1() {
       assert_eq!(1, 1);
       assert_eq!(2, 2);
       // hook inserted here
   }

   expands to

   fn t1() {
       assert_eq!(1, 1);
       assert_eq!(2, 2);
       hook()
   }
*/
#[proc_macro_attribute]
pub fn cvlr_hook_on_exit(attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse the attribute argument

    let attr = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);
    if attr.len() != 1 {
        return quote! {
            compile_error!("Expected 1 argument");
        }
        .into();
    }

    let arg = &attr.first().unwrap();

    // parse the input tokens and make sure it is a function
    let mut fn_item = parse_macro_input!(input as ItemFn);
    let ret_type = &fn_item.sig.output;

    // create tokens_end
    let stmt_end: syn::Stmt = syn::parse_quote! { #arg; };

    // len of fn item statements
    let len = fn_item.block.stmts.len();

    match ret_type {
        syn::ReturnType::Default => {
            // insert tokens_end into fn item statements at position len
            fn_item
                .block
                .stmts
                .insert(len, stmt_end);
        }
        syn::ReturnType::Type(_, _) => {
            // insert tokens_end into fn item statements at position len-1
            fn_item
                .block
                .stmts
                .insert(len - 1, stmt_end);
        }
    }

    fn_item.into_token_stream().into()
}
