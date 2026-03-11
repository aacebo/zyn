use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use crate::Expand;

/// Literal Rust tokens passed through unchanged in a zyn template.
///
/// In `zyn! { fn {{ name }}() {} }`, the tokens `fn`, `(`, `)`, `{`, `}` each
/// contribute to a `TokensNode`.
pub struct TokensNode {
    /// Source span.
    pub span: Span,
    /// The literal token stream.
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
