use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;
use quote::quote;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::super::Element;

use crate::Expand;

pub struct MatchNode {
    pub span: Span,
    pub expr: TokenStream,
    pub arms: Vec<(TokenStream, Element)>,
}

impl MatchNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for MatchNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr_content;
        syn::parenthesized!(expr_content in input);
        let expr: TokenStream = expr_content.parse()?;

        let arms_content;
        syn::braced!(arms_content in input);

        let mut arms = Vec::new();

        while !arms_content.is_empty() {
            let mut pattern = TokenStream::new();

            while !arms_content.is_empty() {
                if arms_content.peek(Token![=>]) {
                    arms_content.parse::<Token![=>]>()?;
                    break;
                }

                let tt: TokenTree = arms_content.parse()?;
                tt.to_tokens(&mut pattern);
            }

            let body_content;
            syn::braced!(body_content in arms_content);
            let body = body_content.parse::<Element>()?;

            arms.push((pattern, body));

            if arms_content.peek(Token![,]) {
                arms_content.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            span: Span::call_site(),
            expr,
            arms,
        })
    }
}

impl Expand for MatchNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        let expr = &self.expr;
        let arms: Vec<TokenStream> = self
            .arms
            .iter()
            .map(|(pattern, body)| {
                let body_expanded = body.expand(output, idents);
                quote! { #pattern => { #body_expanded } }
            })
            .collect();

        quote! {
            match #expr {
                #(#arms),*
            }
        }
    }
}
