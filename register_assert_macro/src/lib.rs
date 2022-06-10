extern crate proc_macro;

mod parse;

use parse::Args;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
/// make fixture for assert
pub fn register_assert(args: TokenStream) -> TokenStream {
    let args: Args = match syn::parse(args) {
        Ok(a) => a,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let parsable = args.get_parsable();
    let error = args.get_error();
    let generics = match args.get_generics() {
        Some(i) => quote! {#i},
        None => quote! {},
    };
    quote! {
        type Assert = assert_parse::Assert<#parsable #generics, #error>;

        #[rstest::fixture]
        fn assert() -> Assert {
            assert_parse::make_assert()
        }
    }
    .into()
}
