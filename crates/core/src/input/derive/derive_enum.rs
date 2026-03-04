use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone)]
pub struct DeriveEnum {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::DataEnum,
}

impl DeriveEnum {
    pub fn data(&self) -> &syn::DataEnum {
        &self.data
    }
}

impl Parse for DeriveEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let di: syn::DeriveInput = input.parse()?;
        match di.data {
            syn::Data::Enum(data) => Ok(Self {
                attrs: di.attrs,
                vis: di.vis,
                ident: di.ident,
                generics: di.generics,
                data,
            }),
            _ => Err(syn::Error::new_spanned(&di.ident, "expected enum")),
        }
    }
}

impl ToTokens for DeriveEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let generics = &self.generics;
        let variants = &self.data.variants;
        quote::quote! {
            #(#attrs)*
            #vis enum #ident #generics { #variants }
        }
        .to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_enum() {
        let e: DeriveEnum = syn::parse_str("enum Dir { North, South }").unwrap();
        assert_eq!(e.ident.to_string(), "Dir");
    }

    #[test]
    fn struct_rejected() {
        let result: syn::Result<DeriveEnum> = syn::parse_str("struct Foo {}");
        assert!(result.is_err());
    }
}
