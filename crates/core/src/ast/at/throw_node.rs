use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::ToTokens;
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
        let message: syn::LitStr = input.parse()?;

        Ok(Self {
            span: message.span(),
            message: message.into_token_stream(),
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
