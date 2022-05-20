use proc_macro2::TokenStream as TokenStream2;
use std::fmt::Display;
use std::marker::PhantomData;
use syn::parse::Parse;

/// The container struct for assert method and its type.
pub struct Assert<T: Parse, U: Display> {
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

/// This should be use with type signature.
pub fn make_assert<T, U>() -> Assert<T, U>
where
    T: Parse,
    U: Display,
{
    Assert::<T, U> {
        _t: PhantomData,
        _u: PhantomData,
    }
}

impl<T: Parse, U: Display> Assert<T, U> {
    /// # Panics
    ///
    /// * when it is success to parse
    /// * when there is different in error message
    pub fn error(&self, arg: TokenStream2, error: U) {
        let arg: syn::Result<T> = syn::parse2(arg);
        match arg {
            Ok(_) => {
                panic!("must occurre error.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), error.to_string());
            }
        }
    }

    /// # Panics
    ///
    /// * when it is failed to parse
    /// * when it is failed to finish to call assert
    pub fn ok<V>(&self, arg: TokenStream2, assert: V)
    where
        V: FnOnce(T),
    {
        let arg: syn::Result<T> = syn::parse2(arg);
        match arg {
            Ok(a) => {
                assert(a);
            }
            Err(_) => {
                panic!("must not occurre error.");
            }
        }
    }
}
