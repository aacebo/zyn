use proc_macro2::Span;
use syn::Lit;

use crate::input::Input;
use crate::meta::Arg;
use crate::meta::Args;

pub trait FromInput: Sized {
    type Error: Into<syn::Error>;

    fn from_input(input: &Input) -> Result<Self, Self::Error>;
}

pub trait FromArg: Sized {
    fn from_arg(arg: &Arg) -> syn::Result<Self>;
}

fn lit_from_arg(arg: &Arg) -> Option<&Lit> {
    match arg {
        Arg::Lit(lit) => Some(lit),
        Arg::Expr(_, syn::Expr::Lit(syn::ExprLit { lit, .. })) => Some(lit),
        _ => None,
    }
}

fn span_of(arg: &Arg) -> Span {
    match arg {
        Arg::Flag(i) | Arg::Expr(i, _) | Arg::List(i, _) => i.span(),
        Arg::Lit(_) => Span::call_site(),
    }
}

impl FromArg for bool {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::Flag(_) => Ok(true),
            _ => Err(syn::Error::new(span_of(arg), "expected flag for bool")),
        }
    }
}

impl FromArg for String {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match lit_from_arg(arg) {
            Some(Lit::Str(s)) => Ok(s.value()),
            _ => Err(syn::Error::new(span_of(arg), "expected string literal")),
        }
    }
}

macro_rules! impl_from_arg_int {
    ($($t:ty),*) => {
        $(
            impl FromArg for $t {
                fn from_arg(arg: &Arg) -> syn::Result<Self> {
                    match lit_from_arg(arg) {
                        Some(Lit::Int(i)) => i.base10_parse::<$t>().map_err(|e| syn::Error::new(i.span(), e)),
                        _ => Err(syn::Error::new(span_of(arg), concat!("expected integer literal for ", stringify!($t)))),
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
                fn from_arg(arg: &Arg) -> syn::Result<Self> {
                    match lit_from_arg(arg) {
                        Some(Lit::Float(f)) => f.base10_parse::<$t>().map_err(|e| syn::Error::new(f.span(), e)),
                        _ => Err(syn::Error::new(span_of(arg), concat!("expected float literal for ", stringify!($t)))),
                    }
                }
            }
        )*
    };
}

impl_from_arg_float!(f32, f64);

impl FromArg for char {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match lit_from_arg(arg) {
            Some(Lit::Char(c)) => Ok(c.value()),
            _ => Err(syn::Error::new(span_of(arg), "expected char literal")),
        }
    }
}

impl FromArg for syn::Ident {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::Flag(i) => Ok(i.clone()),
            _ => Err(syn::Error::new(span_of(arg), "expected identifier")),
        }
    }
}

impl FromArg for syn::Path {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::Flag(i) => Ok(syn::Path::from(i.clone())),
            _ => Err(syn::Error::new(
                span_of(arg),
                "expected identifier for path",
            )),
        }
    }
}

impl FromArg for syn::Expr {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::Expr(_, expr) => Ok(expr.clone()),
            _ => Err(syn::Error::new(span_of(arg), "expected expression")),
        }
    }
}

impl FromArg for syn::LitStr {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match lit_from_arg(arg) {
            Some(Lit::Str(s)) => Ok(s.clone()),
            _ => Err(syn::Error::new(span_of(arg), "expected string literal")),
        }
    }
}

impl FromArg for syn::LitInt {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match lit_from_arg(arg) {
            Some(Lit::Int(i)) => Ok(i.clone()),
            _ => Err(syn::Error::new(span_of(arg), "expected integer literal")),
        }
    }
}

impl<T: FromArg> FromArg for Option<T> {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        T::from_arg(arg).map(Some)
    }
}

impl<T: FromArg> FromArg for Vec<T> {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::List(_, args) => args.iter().map(T::from_arg).collect(),
            _ => Err(syn::Error::new(span_of(arg), "expected list argument")),
        }
    }
}

impl FromArg for Args {
    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::List(_, args) => syn::parse2(args.to_token_stream()),
            _ => Err(syn::Error::new(span_of(arg), "expected list argument")),
        }
    }
}

use quote::ToTokens;

#[cfg(test)]
mod tests {
    use super::*;

    mod bool_impl {
        use super::*;

        #[test]
        fn from_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert_eq!(bool::from_arg(&arg).unwrap(), true);
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
