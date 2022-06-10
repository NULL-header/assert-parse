use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{
    parse::{Parse, ParseStream},
    Generics, Ident, Token,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum ArgsError {
    #[error("The macro needs two ident as args.")]
    Empty,
    #[error("The macro needs to separate with comma.")]
    NotComma,
    #[error("The macro needs two.")]
    Single,
    #[error("The macro needs idents.")]
    NotIdent,
    #[error("The macro needs an ident after comma too.")]
    InvalidOmit,
    #[error("The macro needs only two idents as args.")]
    TooMany,
}

impl ArgsError {
    fn to_syn_error(&self, span: Span) -> syn::Error {
        syn::Error::new(span, self.to_string())
    }
}

pub struct Args {
    parsable: Ident,
    error: Ident,
    generics: Option<Generics>,
}

impl Args {
    pub fn get_parsable(&self) -> &Ident {
        &self.parsable
    }
    pub fn get_error(&self) -> &Ident {
        &self.error
    }
    pub fn get_generics(&self) -> &Option<Generics> {
        &self.generics
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(ArgsError::Empty.to_syn_error(input.span()));
        }
        let parsable: Ident = match input.parse() {
            Ok(i) => i,
            Err(e) => {
                return Err(ArgsError::NotIdent.to_syn_error(e.span()));
            }
        };
        if input.is_empty() {
            return Err(ArgsError::Single.to_syn_error(input.span()));
        }
        let generics: Option<Generics> = input.parse().ok();
        let _comma: Token![,] = match input.parse() {
            Ok(i) => i,
            Err(e) => {
                return Err(ArgsError::NotComma.to_syn_error(e.span()));
            }
        };
        if input.is_empty() {
            return Err(ArgsError::InvalidOmit.to_syn_error(input.span()));
        }
        let error: Ident = match input.parse() {
            Ok(i) => i,
            Err(e) => {
                return Err(ArgsError::NotIdent.to_syn_error(e.span()));
            }
        };
        if !input.is_empty() {
            return Err(ArgsError::TooMany.to_syn_error(input.span()));
        }
        Ok(Self {
            error,
            parsable,
            generics,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_parse_core::*;
    use quote::quote;
    use rstest::*;

    type Assert = assert_parse_core::Assert<Args, ArgsError>;

    #[fixture]
    fn assert() -> Assert {
        make_assert()
    }

    #[rstest]
    fn empty(assert: Assert) {
        let args = quote! {};
        assert.error(args, ArgsError::Empty);
    }

    #[rstest]
    fn not_ident_first(assert: Assert) {
        let args = quote! {1};
        assert.error(args, ArgsError::NotIdent);
    }

    #[rstest]
    fn single(assert: Assert) {
        let args = quote! {Mock};
        assert.error(args, ArgsError::Single);
    }

    #[rstest]
    fn not_comma(assert: Assert) {
        let args = quote! {Mock.};
        assert.error(args, ArgsError::NotComma);
    }

    #[rstest]
    fn invalid_omit(assert: Assert) {
        let args = quote! {Mock,};
        assert.error(args, ArgsError::InvalidOmit);
    }

    #[rstest]
    fn not_ident_second(assert: Assert) {
        let args = quote! {Mock,1};
        assert.error(args, ArgsError::NotIdent);
    }

    #[rstest]
    fn ok(assert: Assert) {
        let args = quote! {Mock,MockError};
        assert.ok(args, |args| {
            assert_eq!(&args.parsable.to_string(), "Mock");
            assert_eq!(&args.error.to_string(), "MockError");
        });
    }

    #[rstest]
    fn ok_with_generics(assert: Assert) {
        let args = quote! {Mock<'a>,MockError};
        assert.ok(args, |args| {
            assert_eq!(args.get_parsable().to_string(), "Mock");
            let target_generics = args.get_generics();
            let target_generics = quote! {#target_generics};
            assert_eq!(target_generics.to_string(), "< 'a >");
            assert_eq!(args.get_error().to_string(), "MockError");
        });
    }

    #[rstest]
    fn too_many(assert: Assert) {
        let args = quote! {Mock,MockError,Something};
        assert.error(args, ArgsError::TooMany);
    }
}
