use proc_macro2::Ident;

use quote::ToTokens;
use quote::TokenStreamExt;

use syn::Expr;
use syn::Lit;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::Args;

pub enum Arg {
    Flag(Ident),
    Expr(Ident, Expr),
    List(Ident, Args),
    Lit(Lit),
}

impl Arg {
    pub fn name(&self) -> Option<&Ident> {
        match self {
            Self::Flag(name) => Some(name),
            Self::Expr(name, _) => Some(name),
            Self::List(name, _) => Some(name),
            Self::Lit(_) => None,
        }
    }

    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag(_))
    }

    pub fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(_, _))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_, _))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn as_expr(&self) -> &Expr {
        match self {
            Self::Expr(_, expr) => expr,
            _ => panic!("called `Arg::as_expr()` on a non-Expr variant"),
        }
    }

    pub fn as_args(&self) -> &Args {
        match self {
            Self::List(_, args) => args,
            _ => panic!("called `Arg::as_args()` on a non-List variant"),
        }
    }

    pub fn as_lit(&self) -> &Lit {
        match self {
            Self::Lit(lit) => lit,
            _ => panic!("called `Arg::as_lit()` on a non-Lit variant"),
        }
    }

    pub fn as_flag(&self) -> &Ident {
        match self {
            Self::Flag(i) => i,
            _ => panic!("called `Arg::as_flag()` on a non-Flag variant"),
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Self::Lit(Lit::Str(s)) => s.value(),
            Self::Expr(
                _,
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(s), ..
                }),
            ) => s.value(),
            _ => panic!("called `Arg::as_str()` on a non-string variant"),
        }
    }

    pub fn as_int<T: std::str::FromStr>(&self) -> T
    where
        T::Err: std::fmt::Display,
    {
        match self {
            Self::Lit(Lit::Int(i)) => i.base10_parse().expect("invalid integer literal"),
            Self::Expr(
                _,
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Int(i), ..
                }),
            ) => i.base10_parse().expect("invalid integer literal"),
            _ => panic!("called `Arg::as_int()` on a non-integer variant"),
        }
    }

    pub fn as_float<T: std::str::FromStr>(&self) -> T
    where
        T::Err: std::fmt::Display,
    {
        match self {
            Self::Lit(Lit::Float(f)) => f.base10_parse().expect("invalid float literal"),
            Self::Expr(
                _,
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Float(f), ..
                }),
            ) => f.base10_parse().expect("invalid float literal"),
            _ => panic!("called `Arg::as_float()` on a non-float variant"),
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Lit(Lit::Char(c)) => c.value(),
            Self::Expr(
                _,
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Char(c), ..
                }),
            ) => c.value(),
            _ => panic!("called `Arg::as_char()` on a non-char variant"),
        }
    }

    pub fn as_expr_lit(&self) -> Option<&Lit> {
        match self {
            Self::Lit(lit) => Some(lit),
            Self::Expr(_, Expr::Lit(syn::ExprLit { lit, .. })) => Some(lit),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Self::Lit(Lit::Bool(b)) => b.value,
            Self::Expr(
                _,
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Bool(b), ..
                }),
            ) => b.value,
            _ => panic!("called `Arg::as_bool()` on a non-bool variant"),
        }
    }
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lit) || input.peek(syn::LitStr) {
            return Ok(Self::Lit(input.parse()?));
        }

        let name: Ident = input.parse()?;

        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let expr: Expr = input.parse()?;
            Ok(Self::Expr(name, expr))
        } else if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let args: Args = content.parse()?;
            Ok(Self::List(name, args))
        } else {
            Ok(Self::Flag(name))
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Flag(name) => name.to_tokens(tokens),
            Self::Expr(name, expr) => {
                name.to_tokens(tokens);
                tokens.append(proc_macro2::Punct::new('=', proc_macro2::Spacing::Alone));
                expr.to_tokens(tokens);
            }
            Self::List(name, args) => {
                name.to_tokens(tokens);
                let inner = args.to_token_stream();
                tokens.append(proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    inner,
                ));
            }
            Self::Lit(lit) => lit.to_tokens(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(arg.is_flag());
            assert_eq!(arg.name().unwrap(), "skip");
        }

        #[test]
        fn expr() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            assert!(arg.is_expr());
            assert_eq!(arg.name().unwrap(), "rename");
        }

        #[test]
        fn list() {
            let arg: Arg = syn::parse_str("serde(rename_all = \"camelCase\")").unwrap();
            assert!(arg.is_list());
            assert_eq!(arg.name().unwrap(), "serde");
            assert!(arg.as_args().has("rename_all"));
        }

        #[test]
        fn lit_str() {
            let arg: Arg = syn::parse_str("\"hello\"").unwrap();
            assert!(arg.is_lit());
            assert!(arg.name().is_none());
        }

        #[test]
        fn lit_int() {
            let arg: Arg = syn::parse_str("42").unwrap();
            assert!(arg.is_lit());
        }
    }

    mod to_tokens {
        use super::*;

        #[test]
        fn flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "skip");
        }

        #[test]
        fn expr() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "rename = \"foo\"");
        }

        #[test]
        fn list() {
            let arg: Arg = syn::parse_str("serde(skip)").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "serde (skip)");
        }

        #[test]
        fn lit() {
            let arg: Arg = syn::parse_str("\"hello\"").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "\"hello\"");
        }
    }

    mod accessors {
        use super::*;

        #[test]
        #[should_panic(expected = "non-Expr")]
        fn as_expr_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_expr();
        }

        #[test]
        #[should_panic(expected = "non-List")]
        fn as_args_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_args();
        }

        #[test]
        #[should_panic(expected = "non-Lit")]
        fn as_lit_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_lit();
        }

        #[test]
        fn as_flag_returns_ident() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert_eq!(arg.as_flag().to_string(), "skip");
        }

        #[test]
        #[should_panic(expected = "non-Flag")]
        fn as_flag_panics_on_expr() {
            let arg: Arg = syn::parse_str("x = 1").unwrap();
            arg.as_flag();
        }

        #[test]
        fn as_str_from_lit() {
            let arg: Arg = syn::parse_str("\"hello\"").unwrap();
            assert_eq!(arg.as_str(), "hello");
        }

        #[test]
        fn as_str_from_expr() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            assert_eq!(arg.as_str(), "foo");
        }

        #[test]
        #[should_panic(expected = "non-string")]
        fn as_str_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_str();
        }

        #[test]
        fn as_int_from_lit() {
            let arg: Arg = syn::parse_str("42").unwrap();
            assert_eq!(arg.as_int::<i64>(), 42i64);
        }

        #[test]
        fn as_int_from_expr() {
            let arg: Arg = syn::parse_str("count = 7").unwrap();
            assert_eq!(arg.as_int::<i64>(), 7i64);
        }

        #[test]
        #[should_panic(expected = "non-integer")]
        fn as_int_panics_on_string_lit() {
            let arg: Arg = syn::parse_str("\"hello\"").unwrap();
            arg.as_int::<i64>();
        }

        #[test]
        fn as_char_from_lit() {
            let arg: Arg = syn::parse_str("'x'").unwrap();
            assert_eq!(arg.as_char(), 'x');
        }

        #[test]
        #[should_panic(expected = "non-char")]
        fn as_char_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_char();
        }
    }
}
