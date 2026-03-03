use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::Token;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::super::Element;

use crate::Expand;

pub struct IfNode {
    pub span: Span,
    pub branches: Vec<(TokenStream, Element)>,
    pub else_body: Option<Box<Element>>,
}

impl IfNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for IfNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut branches = Vec::new();
        let mut else_body = None;

        let cond_content;
        syn::parenthesized!(cond_content in input);
        let condition: TokenStream = cond_content.parse()?;

        let body_content;
        syn::braced!(body_content in input);
        let body = body_content.parse::<Element>()?;

        branches.push((condition, body));

        while is_at_else(input) {
            input.parse::<Token![@]>()?;
            input.call(syn::Ident::parse_any)?;

            if is_keyword_if(input) {
                input.call(syn::Ident::parse_any)?;

                let cond_content;
                syn::parenthesized!(cond_content in input);
                let condition: TokenStream = cond_content.parse()?;

                let body_content;
                syn::braced!(body_content in input);
                let body = body_content.parse::<Element>()?;

                branches.push((condition, body));
            } else {
                let body_content;
                syn::braced!(body_content in input);
                else_body = Some(Box::new(body_content.parse::<Element>()?));
                break;
            }
        }

        Ok(Self {
            span: Span::call_site(),
            branches,
            else_body,
        })
    }
}

impl Expand for IfNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        let mut result = TokenStream::new();

        for (i, (cond, body)) in self.branches.iter().enumerate() {
            let body_expanded = body.expand(output, idents);

            if i == 0 {
                result = quote! { if #cond { #body_expanded } };
            } else {
                result = quote! { #result else if #cond { #body_expanded } };
            }
        }

        if let Some(else_body) = &self.else_body {
            let else_expanded = else_body.expand(output, idents);
            result = quote! { #result else { #else_expanded } };
        }

        result
    }
}

fn is_at_else(input: ParseStream) -> bool {
    if !input.peek(Token![@]) {
        return false;
    }

    let fork = input.fork();
    let _ = fork.parse::<Token![@]>();

    match fork.call(syn::Ident::parse_any) {
        Ok(ident) => ident == "else",
        Err(_) => false,
    }
}

fn is_keyword_if(input: ParseStream) -> bool {
    let fork = input.fork();

    match fork.call(syn::Ident::parse_any) {
        Ok(ident) => ident == "if",
        Err(_) => false,
    }
}
