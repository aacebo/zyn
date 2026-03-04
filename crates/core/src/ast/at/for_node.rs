use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::super::Element;

use crate::Expand;

pub struct ForNode {
    pub span: Span,
    pub binding: syn::Ident,
    pub iter: TokenStream,
    pub body: Box<Element>,
}

impl ForNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ForNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let params;
        syn::parenthesized!(params in input);

        let fork = params.fork();
        let is_classic = if fork.call(syn::Ident::parse_any).is_ok() {
            let Ok(kw) = fork.call(syn::Ident::parse_any) else {
                return Self::parse_classic(&params, input);
            };
            kw != "in"
        } else {
            true
        };

        if is_classic {
            return Self::parse_classic(&params, input);
        }

        let binding: syn::Ident = params.call(syn::Ident::parse_any)?;
        let _in_kw: syn::Ident = params.call(syn::Ident::parse_any)?;
        let iter: TokenStream = params.parse()?;

        let body_content;
        syn::braced!(body_content in input);
        let body = body_content.parse::<Element>()?;

        Ok(Self {
            span: Span::call_site(),
            binding,
            iter,
            body: Box::new(body),
        })
    }
}

impl ForNode {
    fn parse_classic(params: ParseStream, input: ParseStream) -> syn::Result<Self> {
        let count: TokenStream = params.parse()?;
        let binding = syn::Ident::new("_", Span::call_site());
        let iter: TokenStream = quote! { 0..#count };

        let body_content;
        syn::braced!(body_content in input);
        let body = body_content.parse::<Element>()?;

        Ok(Self {
            span: Span::call_site(),
            binding,
            iter,
            body: Box::new(body),
        })
    }
}

impl Expand for ForNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        let binding = &self.binding;
        let iter = &self.iter;
        let body_expanded = self.body.expand(output, idents);

        quote! {
            for #binding in #iter {
                #body_expanded
            }
        }
    }
}
