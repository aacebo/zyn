use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::ToTokens;
use quote::quote_spanned;

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;

pub struct WarnNode {
    pub span: Span,
    pub message: TokenStream,
}

impl WarnNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for WarnNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let message: syn::LitStr = input.parse()?;

        Ok(Self {
            span: message.span(),
            message: message.into_token_stream(),
        })
    }
}

impl Expand for WarnNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let span = self.span;
        let message = &self.message;

        quote_spanned! { span =>
            {
                #[allow(dead_code)]
                #[deprecated(note = #message)]
                fn _zyn_warning() {}
                #[allow(deprecated)]
                _zyn_warning();
            }
        }
    }
}
