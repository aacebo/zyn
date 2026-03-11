use proc_macro2::Delimiter;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::template::Template;

use crate::Expand;

/// A delimited group in a zyn template: `(...)`, `[...]`, or `{...}`.
///
/// The body is expanded recursively and wrapped in the appropriate delimiter.
pub struct GroupNode {
    /// Source span.
    pub span: Span,
    /// The delimiter kind: parenthesis, bracket, or brace.
    pub delimiter: Delimiter,
    /// The template body inside the delimiters.
    pub body: Box<Template>,
}

impl GroupNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for GroupNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            let paren = syn::parenthesized!(content in input);
            let body = content.parse::<Template>()?;

            Ok(Self {
                span: paren.span.join(),
                delimiter: Delimiter::Parenthesis,
                body: Box::new(body),
            })
        } else if input.peek(syn::token::Bracket) {
            let content;
            let bracket = syn::bracketed!(content in input);
            let body = content.parse::<Template>()?;

            Ok(Self {
                span: bracket.span.join(),
                delimiter: Delimiter::Bracket,
                body: Box::new(body),
            })
        } else {
            Err(input.error("expected a delimited group"))
        }
    }
}

impl Expand for GroupNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        let inner = idents.next().unwrap();
        let body_expanded = self.body.expand(&inner, idents);

        let delim = match self.delimiter {
            Delimiter::Parenthesis => {
                quote! { ::zyn::proc_macro2::Delimiter::Parenthesis }
            }
            Delimiter::Bracket => quote! { ::zyn::proc_macro2::Delimiter::Bracket },
            Delimiter::Brace => quote! { ::zyn::proc_macro2::Delimiter::Brace },
            Delimiter::None => quote! { ::zyn::proc_macro2::Delimiter::None },
        };

        quote! {
            {
                let mut #inner = ::zyn::proc_macro2::TokenStream::new();
                #body_expanded
                ::zyn::quote::ToTokens::to_tokens(
                    &::zyn::proc_macro2::Group::new(#delim, #inner),
                    &mut #output,
                );
            }
        }
    }
}
