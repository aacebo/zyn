use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;

pub struct ThrowNode {
    pub span: Span,
    pub message: TokenStream,
}

impl ThrowNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ThrowNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let msg_content;
        syn::parenthesized!(msg_content in input);
        let message: TokenStream = msg_content.parse()?;

        Ok(Self {
            span: Span::call_site(),
            message,
        })
    }
}

impl Expand for ThrowNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let message = &self.message;
        quote! {
            ::core::compile_error!(#message);
        }
    }
}
