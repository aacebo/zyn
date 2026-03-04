use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ImplItemFn(pub syn::ImplItemFn);

impl Parse for ImplItemFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ImplItemFn {
    type Target = syn::ImplItemFn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ImplItemFn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ImplItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let f: ImplItemFn = syn::parse_str("fn bar(&self) -> u32 { 42 }").unwrap();
        assert_eq!(f.sig.ident.to_string(), "bar");
    }
}
