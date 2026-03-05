use std::ops::Index;

use quote::ToTokens;
use quote::TokenStreamExt;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::Arg;

#[derive(Default)]
pub struct Args(Vec<Arg>);

impl Args {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn has(&self, name: &str) -> bool {
        self.0
            .iter()
            .any(|arg| arg.name().is_some_and(|n| n == name))
    }

    pub fn get(&self, name: &str) -> Option<&Arg> {
        self.0
            .iter()
            .find(|arg| arg.name().is_some_and(|n| n == name))
    }

    pub fn get_index(&self, index: usize) -> Option<&Arg> {
        self.0.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arg> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extend(&mut self, other: Args) {
        self.0.extend(other.0);
    }

    pub fn merge(&self, other: &Args) -> Args {
        let mut result: Vec<Arg> = Vec::new();

        for arg in &self.0 {
            if let Some(name) = arg.name()
                && other.has(&name.to_string())
            {
                continue;
            }

            result.push(clone_arg(arg));
        }

        for arg in &other.0 {
            result.push(clone_arg(arg));
        }

        Args(result)
    }
}

fn clone_arg(arg: &Arg) -> Arg {
    syn::parse2(arg.to_token_stream()).unwrap()
}

impl Index<usize> for Args {
    type Output = Arg;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for Args {
    type Item = Arg;
    type IntoIter = std::vec::IntoIter<Arg>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Args {
    type Item = &'a Arg;
    type IntoIter = std::slice::Iter<'a, Arg>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();

        while !input.is_empty() {
            args.push(input.parse::<Arg>()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self(args))
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for (i, arg) in self.0.iter().enumerate() {
            if i > 0 {
                tokens.append(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone));
            }

            arg.to_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn empty() {
            let args: Args = syn::parse_str("").unwrap();
            assert!(args.is_empty());
            assert_eq!(args.len(), 0);
        }

        #[test]
        fn single_flag() {
            let args: Args = syn::parse_str("skip").unwrap();
            assert_eq!(args.len(), 1);
            assert!(args.has("skip"));
        }

        #[test]
        fn multiple() {
            let args: Args = syn::parse_str("skip, rename = \"foo\", serde(flatten)").unwrap();
            assert_eq!(args.len(), 3);
            assert!(args.has("skip"));
            assert!(args.has("rename"));
            assert!(args.has("serde"));
        }

        #[test]
        fn trailing_comma() {
            let args: Args = syn::parse_str("skip, rename = \"foo\",").unwrap();
            assert_eq!(args.len(), 2);
        }

        #[test]
        fn lit() {
            let args: Args = syn::parse_str("\"hello\"").unwrap();
            assert_eq!(args.len(), 1);
            assert!(args[0].is_lit());
        }

        #[test]
        fn nested() {
            let args: Args = syn::parse_str("outer(inner(deep = 1))").unwrap();
            assert_eq!(args.len(), 1);

            let outer = args.get("outer").unwrap().as_args();
            assert!(outer.has("inner"));

            let inner = outer.get("inner").unwrap().as_args();
            assert!(inner.has("deep"));
        }
    }

    mod query {
        use super::*;

        #[test]
        fn has_existing() {
            let args: Args = syn::parse_str("skip, rename = \"foo\"").unwrap();
            assert!(args.has("skip"));
            assert!(args.has("rename"));
        }

        #[test]
        fn has_missing() {
            let args: Args = syn::parse_str("skip").unwrap();
            assert!(!args.has("rename"));
        }

        #[test]
        fn get_existing() {
            let args: Args = syn::parse_str("skip, rename = \"foo\"").unwrap();
            assert!(args.get("skip").unwrap().is_flag());
            assert!(args.get("rename").unwrap().is_expr());
        }

        #[test]
        fn get_missing() {
            let args: Args = syn::parse_str("skip").unwrap();
            assert!(args.get("rename").is_none());
        }

        #[test]
        fn index() {
            let args: Args = syn::parse_str("skip, rename = \"foo\"").unwrap();
            assert!(args[0].is_flag());
            assert!(args[1].is_expr());
        }

        #[test]
        fn iter() {
            let args: Args = syn::parse_str("a, b, c").unwrap();
            let names: Vec<String> = args
                .iter()
                .filter_map(|a| a.name())
                .map(|n| n.to_string())
                .collect();
            assert_eq!(names, vec!["a", "b", "c"]);
        }
    }

    mod merge {
        use super::*;

        #[test]
        fn no_overlap() {
            let a: Args = syn::parse_str("skip").unwrap();
            let b: Args = syn::parse_str("rename = \"foo\"").unwrap();
            let merged = a.merge(&b);
            assert_eq!(merged.len(), 2);
            assert!(merged.has("skip"));
            assert!(merged.has("rename"));
        }

        #[test]
        fn override_key() {
            let a: Args = syn::parse_str("rename = \"foo\"").unwrap();
            let b: Args = syn::parse_str("rename = \"bar\"").unwrap();
            let merged = a.merge(&b);
            assert_eq!(merged.len(), 1);
        }

        #[test]
        fn extend_keeps_duplicates() {
            let mut a: Args = syn::parse_str("skip").unwrap();
            let b: Args = syn::parse_str("skip, rename = \"foo\"").unwrap();
            a.extend(b);
            assert_eq!(a.len(), 3);
        }

        #[test]
        fn empty_merge() {
            let a: Args = syn::parse_str("skip").unwrap();
            let b: Args = syn::parse_str("").unwrap();
            let merged = a.merge(&b);
            assert_eq!(merged.len(), 1);
            assert!(merged.has("skip"));
        }
    }

    mod to_tokens {
        use super::*;

        #[test]
        fn round_trip() {
            let input = "skip , rename = \"foo\" , serde (flatten)";
            let args: Args = syn::parse_str(input).unwrap();
            let output = args.to_token_stream().to_string();
            let reparsed: Args = syn::parse_str(&output).unwrap();
            assert_eq!(reparsed.len(), 3);
            assert!(reparsed.has("skip"));
            assert!(reparsed.has("rename"));
            assert!(reparsed.has("serde"));
        }
    }
}
