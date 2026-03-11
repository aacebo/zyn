//! Extractors for resolving typed values from proc macro input.
//!
//! [`FromInput`] is the core extraction trait. Built-in extractors handle the most
//! common patterns — pass `&Input` to resolve them:
//!
//! | Extractor | Resolves |
//! |-----------|---------|
//! | [`Fields<T>`] | Named or unnamed struct fields |
//! | [`Attr<T>`] | A typed `#[derive(Attribute)]` struct from helper attrs |
//! | [`Variants`] | Enum variants |
//! | [`Data<T>`] | Re-parsed full input |
//! | [`Extract<T>`] | Any `FromInput` impl |
//!
//! # Examples
//!
//! Extracting fields and an ident in an element (extractors are detected by type
//! name — no attribute needed):
//!
//! ```ignore
//! #[zyn::element]
//! fn my_impl(
//!     ident: zyn::Extract<zyn::syn::Ident>,
//!     fields: zyn::Fields<zyn::syn::FieldsNamed>,
//! ) -> zyn::TokenStream {
//!     zyn::zyn! {
//!         impl {{ ident }} {
//!             @for (f in fields.named.iter()) {
//!                 pub fn {{ f.ident }}_ref(&self) -> &{{ f.ty }} {
//!                     &self.{{ f.ident }}
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Implementing `FromInput` manually:
//!
//! ```ignore
//! use zyn_core::{FromInput, Input, Result};
//!
//! struct MyExtractor(String);
//!
//! impl FromInput for MyExtractor {
//!     fn from_input(input: &Input) -> Result<Self> {
//!         Ok(MyExtractor(input.ident().to_string()))
//!     }
//! }
//! ```

use syn::Lit;
use syn::spanned::Spanned;

use crate::mark;
use crate::meta::Arg;
use crate::meta::Args;
use crate::types::Input;

mod attr;
mod data;
mod fields;
mod variants;

pub use attr::*;
pub use data::*;
pub use fields::*;
pub use variants::*;

/// Extracts a value from the macro input context.
///
/// Implement this trait to define how a type is resolved from an `Input`
/// (derive or item). Built-in impls exist for `Ident`, `Generics`, and
/// `Visibility`. The `#[element]` macro uses this trait to auto-resolve
/// extractor parameters.
pub trait FromInput: Sized {
    fn from_input(input: &Input) -> crate::Result<Self>;
}

/// Generic extractor wrapper — delegates to `T::from_input`.
///
/// Use this in element parameters to extract any `FromInput` type
/// without giving it a more specific semantic role like `Attr` or `Fields`.
pub struct Extract<T: FromInput>(T);

impl<T: FromInput> Extract<T> {
    /// Consumes the wrapper and returns the inner value.
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T: FromInput> std::ops::Deref for Extract<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromInput> std::ops::DerefMut for Extract<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: FromInput> FromInput for Extract<T> {
    fn from_input(input: &Input) -> crate::Result<Self> {
        T::from_input(input).map(Extract)
    }
}

impl FromInput for proc_macro2::Ident {
    fn from_input(input: &Input) -> crate::Result<Self> {
        Ok(input.ident().clone())
    }
}

impl FromInput for syn::Generics {
    fn from_input(input: &Input) -> crate::Result<Self> {
        Ok(input.generics().clone())
    }
}

impl FromInput for syn::Visibility {
    fn from_input(input: &Input) -> crate::Result<Self> {
        Ok(input.vis().clone())
    }
}

/// Converts a single `Arg` into a typed value.
///
/// Implement this trait to support a type as a field in `#[derive(Attribute)]`
/// structs. Built-in impls cover `bool`, `String`, integer/float primitives,
/// `char`, `Ident`, `Path`, `Expr`, `LitStr`, `LitInt`, `Option<T>`, `Vec<T>`,
/// and `Args`.
pub trait FromArg: Sized {
    fn from_arg(arg: &Arg) -> crate::Result<Self>;
}

impl FromArg for bool {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg {
            Arg::Flag(_) => Ok(true),
            _ => Err(mark::error("expected flag for bool")
                .span(arg.span())
                .build()),
        }
    }
}

impl FromArg for String {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg.as_expr_lit() {
            Some(Lit::Str(s)) => Ok(s.value()),
            _ => Err(mark::error("expected string literal")
                .span(arg.span())
                .build()),
        }
    }
}

macro_rules! impl_from_arg_int {
    ($($t:ty),*) => {
        $(
            impl FromArg for $t {
                fn from_arg(arg: &Arg) -> crate::Result<Self> {
                    match arg.as_expr_lit() {
                        Some(Lit::Int(i)) => i.base10_parse::<$t>().map_err(|e| mark::error(e.to_string()).span(i.span()).build()),
                        _ => Err(mark::error(concat!("expected integer literal for ", stringify!($t))).span(arg.span()).build()),
                    }
                }
            }
        )*
    };
}

impl_from_arg_int!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

macro_rules! impl_from_arg_float {
    ($($t:ty),*) => {
        $(
            impl FromArg for $t {
                fn from_arg(arg: &Arg) -> crate::Result<Self> {
                    match arg.as_expr_lit() {
                        Some(Lit::Float(f)) => f.base10_parse::<$t>().map_err(|e| mark::error(e.to_string()).span(f.span()).build()),
                        _ => Err(mark::error(concat!("expected float literal for ", stringify!($t))).span(arg.span()).build()),
                    }
                }
            }
        )*
    };
}

impl_from_arg_float!(f32, f64);

impl FromArg for char {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg.as_expr_lit() {
            Some(Lit::Char(c)) => Ok(c.value()),
            _ => Err(mark::error("expected char literal")
                .span(arg.span())
                .build()),
        }
    }
}

impl FromArg for syn::Ident {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg {
            Arg::Flag(i) => Ok(i.clone()),
            _ => Err(mark::error("expected identifier").span(arg.span()).build()),
        }
    }
}

impl FromArg for syn::Path {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg {
            Arg::Flag(i) => Ok(syn::Path::from(i.clone())),
            _ => Err(mark::error("expected identifier for path")
                .span(arg.span())
                .build()),
        }
    }
}

impl FromArg for syn::Expr {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg {
            Arg::Expr(_, expr) => Ok(expr.clone()),
            _ => Err(mark::error("expected expression").span(arg.span()).build()),
        }
    }
}

impl FromArg for syn::LitStr {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg.as_expr_lit() {
            Some(Lit::Str(s)) => Ok(s.clone()),
            _ => Err(mark::error("expected string literal")
                .span(arg.span())
                .build()),
        }
    }
}

impl FromArg for syn::LitInt {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg.as_expr_lit() {
            Some(Lit::Int(i)) => Ok(i.clone()),
            _ => Err(mark::error("expected integer literal")
                .span(arg.span())
                .build()),
        }
    }
}

impl<T: FromArg> FromArg for Option<T> {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        T::from_arg(arg).map(Some)
    }
}

impl<T: FromArg> FromArg for Vec<T> {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg {
            Arg::List(_, args) => args.iter().map(T::from_arg).collect(),
            _ => Err(mark::error("expected list argument")
                .span(arg.span())
                .build()),
        }
    }
}

impl FromArg for Args {
    fn from_arg(arg: &Arg) -> crate::Result<Self> {
        match arg {
            Arg::List(_, args) => Ok(args.clone()),
            _ => Err(mark::error("expected list argument")
                .span(arg.span())
                .build()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bool_impl {
        use super::*;

        #[test]
        fn from_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(bool::from_arg(&arg).unwrap());
        }

        #[test]
        fn from_expr_is_err() {
            let arg: Arg = syn::parse_str("skip = true").unwrap();
            assert!(bool::from_arg(&arg).is_err());
        }
    }

    mod string_impl {
        use super::*;

        #[test]
        fn from_expr_string_lit() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            assert_eq!(String::from_arg(&arg).unwrap(), "foo");
        }

        #[test]
        fn from_lit_string() {
            let arg: Arg = syn::parse_str("\"bar\"").unwrap();
            assert_eq!(String::from_arg(&arg).unwrap(), "bar");
        }

        #[test]
        fn from_flag_is_err() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(String::from_arg(&arg).is_err());
        }
    }

    mod int_impl {
        use super::*;

        #[test]
        fn i64_from_lit() {
            let arg: Arg = syn::parse_str("42").unwrap();
            assert_eq!(i64::from_arg(&arg).unwrap(), 42);
        }

        #[test]
        fn i64_from_expr() {
            let arg: Arg = syn::parse_str("count = 7").unwrap();
            assert_eq!(i64::from_arg(&arg).unwrap(), 7);
        }

        #[test]
        fn i64_from_flag_is_err() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(i64::from_arg(&arg).is_err());
        }

        #[test]
        fn u32_from_lit() {
            let arg: Arg = syn::parse_str("100").unwrap();
            assert_eq!(u32::from_arg(&arg).unwrap(), 100u32);
        }
    }

    mod char_impl {
        use super::*;

        #[test]
        fn from_lit() {
            let arg: Arg = syn::parse_str("'x'").unwrap();
            assert_eq!(char::from_arg(&arg).unwrap(), 'x');
        }

        #[test]
        fn from_flag_is_err() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(char::from_arg(&arg).is_err());
        }
    }

    mod ident_impl {
        use super::*;

        #[test]
        fn from_flag() {
            let arg: Arg = syn::parse_str("my_ident").unwrap();
            let ident = syn::Ident::from_arg(&arg).unwrap();
            assert_eq!(ident.to_string(), "my_ident");
        }

        #[test]
        fn from_expr_is_err() {
            let arg: Arg = syn::parse_str("x = 1").unwrap();
            assert!(syn::Ident::from_arg(&arg).is_err());
        }
    }

    mod option_impl {
        use super::*;

        #[test]
        fn some_from_expr() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            assert_eq!(
                Option::<String>::from_arg(&arg).unwrap(),
                Some("foo".to_string())
            );
        }

        #[test]
        fn propagates_err() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(Option::<String>::from_arg(&arg).is_err());
        }
    }

    mod vec_impl {
        use super::*;

        #[test]
        fn from_list() {
            let arg: Arg = syn::parse_str("tags(\"a\", \"b\", \"c\")").unwrap();
            let v = Vec::<String>::from_arg(&arg).unwrap();
            assert_eq!(v, vec!["a", "b", "c"]);
        }

        #[test]
        fn from_flag_is_err() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(Vec::<String>::from_arg(&arg).is_err());
        }
    }

    mod args_impl {
        use super::*;

        #[test]
        fn from_list() {
            let arg: Arg = syn::parse_str("inner(a = 1, b = 2)").unwrap();
            let args = Args::from_arg(&arg).unwrap();
            assert!(args.has("a"));
            assert!(args.has("b"));
        }

        #[test]
        fn from_flag_is_err() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(Args::from_arg(&arg).is_err());
        }
    }
}
