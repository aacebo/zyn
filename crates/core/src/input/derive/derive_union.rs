use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone)]
pub struct DeriveUnion {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::DataUnion,
}

impl DeriveUnion {
    pub fn data(&self) -> &syn::DataUnion {
        &self.data
    }
}

impl Parse for DeriveUnion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let di: syn::DeriveInput = input.parse()?;
        match di.data {
            syn::Data::Union(data) => Ok(Self {
                attrs: di.attrs,
                vis: di.vis,
                ident: di.ident,
                generics: di.generics,
                data,
            }),
            _ => Err(syn::Error::new_spanned(&di.ident, "expected union")),
        }
    }
}

impl ToTokens for DeriveUnion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let generics = &self.generics;
        let fields = &self.data.fields;
        quote::quote! {
            #(#attrs)*
            #vis union #ident #generics { #fields }
        }
        .to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_union() {
        let u: DeriveUnion = syn::parse_str("union Bits { i: i32, f: f32 }").unwrap();
        assert_eq!(u.ident.to_string(), "Bits");
    }

    #[test]
    fn struct_rejected() {
        let result: syn::Result<DeriveUnion> = syn::parse_str("struct Foo {}");
        assert!(result.is_err());
    }
}
