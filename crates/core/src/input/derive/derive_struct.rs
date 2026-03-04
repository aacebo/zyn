use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone)]
pub struct DeriveStruct {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::DataStruct,
}

impl DeriveStruct {
    pub fn data(&self) -> &syn::DataStruct {
        &self.data
    }
}

impl Parse for DeriveStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let di: syn::DeriveInput = input.parse()?;
        match di.data {
            syn::Data::Struct(data) => Ok(Self {
                attrs: di.attrs,
                vis: di.vis,
                ident: di.ident,
                generics: di.generics,
                data,
            }),
            _ => Err(syn::Error::new_spanned(&di.ident, "expected struct")),
        }
    }
}

impl ToTokens for DeriveStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let fields = match &self.data.fields {
            syn::Fields::Named(f) => quote::quote! { { #f } },
            syn::Fields::Unnamed(f) => quote::quote! { ( #f ); },
            syn::Fields::Unit => quote::quote! { ; },
        };
        quote::quote! {
            #(#attrs)*
            #vis struct #ident #impl_generics #where_clause #fields
        }
        .to_tokens(tokens);
        let _ = ty_generics;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_named_struct() {
        let s: DeriveStruct = syn::parse_str("struct Point { x: f32, y: f32 }").unwrap();
        assert_eq!(s.ident.to_string(), "Point");
    }

    #[test]
    fn enum_rejected() {
        let result: syn::Result<DeriveStruct> = syn::parse_str("enum Foo {}");
        assert!(result.is_err());
    }
}
