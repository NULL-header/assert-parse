# assert_parse

The util to assert macro parsing.

# Examples

```rust
use assert_parse::*;
use syn::parse::Parse;
use thiserror::Error;

#[derive(Error, Debug)]
enum InputError {
    #[error("This is not ident.")]
    NotIdent,
}

struct Input(syn::Ident);

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let i = match input.parse() {
            Ok(i) => i,
            Err(e) => {
                return syn::Error::new(e.span(), InputError::NotIdent.to_string());
            }
        };
        Ok(Self(i))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use rstest::*;

    type Assert = Assert<Input, InputError>;

    #[fixture]
    fn assert() -> Assert {
        make_assert()
    }

    #[rstest]
    fn error(assert: Assert) {
        let input = quote! {1};
        assert.error(input, InputError::NotIdent);
    }

    #[rstest]
    fn ok() {
        let input = quote! {mock};
        assert.ok(input, |i| {
            assert_eq!(i.to_string(), "mock".to_string());
        });
    }
}
```

Or you can use the macro like this.

```rust
use assert_parse::*;
use syn::parse::Parse;
use thiserror::Error;

#[derive(Error, Debug)]
enum InputError {
    #[error("This is not ident.")]
    NotIdent,
}

struct Input(syn::Ident);

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let i = match input.parse() {
            Ok(i) => i,
            Err(e) => {
                return syn::Error::new(e.span(), InputError::NotIdent.to_string());
            }
        };
        Ok(Self(i))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use rstest::*;
    use assert_parse::register_assert;

    register_assert!(Input,InputError);

    #[rstest]
    fn error(assert: Assert) {
        let input = quote! {1};
        assert.error(input, InputError::NotIdent);
    }

    #[rstest]
    fn ok() {
        let input = quote! {mock};
        assert.ok(input, |i| {
            assert_eq!(i.to_string(), "mock".to_string());
        });
    }
}
```
# what this do

The crate works very smally, but it makes some bases of codes same. So, this is useful as the templete maker.

# dependencies

The crate does not have used on an example as dependencies. You can choice only using this or joining this and others to develop.

# license

MIT
