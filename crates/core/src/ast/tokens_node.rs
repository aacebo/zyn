use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use crate::Expand;

pub struct TokensNode {
    pub span: Span,
    pub stream: TokenStream,
}

impl TokensNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Expand for TokensNode {
    fn expand(&self, output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let stream = &self.stream;
        quote! {
            #output.extend(::zyn::quote::quote!(#stream));
        }
    }
}
