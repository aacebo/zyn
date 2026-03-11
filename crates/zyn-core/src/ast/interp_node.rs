use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;
use quote::quote;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::PipeNode;

use crate::Expand;

/// An interpolation expression: `{{ expr }}` or `{{ expr | pipe | ... }}`.
///
/// The expression is evaluated at expand time and emitted into the output token
/// stream, optionally transformed through a chain of [`PipeNode`]s.
pub struct InterpNode {
    /// Source span of the `{{ ... }}` delimiters.
    pub span: Span,
    /// The expression to interpolate, e.g. `name` or `field.ty`.
    pub expr: TokenStream,
    /// Pipe transforms applied in order, e.g. `[snake, ident:"get_{}"]`.
    pub pipes: Vec<PipeNode>,
}

impl InterpNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for InterpNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = syn::braced!(content in input);
        let span = brace.span.join();

        let inner;
        syn::braced!(inner in content);

        let mut expr = TokenStream::new();
        let mut pipes = Vec::new();

        while !inner.is_empty() {
            if inner.peek(Token![|]) {
                inner.parse::<Token![|]>()?;
                pipes.push(inner.parse::<PipeNode>()?);
            } else if pipes.is_empty() {
                let tt: TokenTree = inner.parse()?;
                tt.to_tokens(&mut expr);
            } else {
                break;
            }
        }

        if expr.is_empty() {
            return Err(syn::Error::new(span, "empty interpolation"));
        }

        Ok(Self { span, expr, pipes })
    }
}

impl Expand for InterpNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        let expr = &self.expr;

        if self.pipes.is_empty() {
            return quote! {
                ::zyn::quote::ToTokens::to_tokens(&(#expr), &mut #output);
            };
        }

        let mut steps = vec![quote! { let __zyn_val = (#expr).to_string(); }];

        for (i, pipe) in self.pipes.iter().enumerate() {
            if i > 0 {
                steps.push(quote! { let __zyn_val = __zyn_val.to_string(); });
            }

            steps.push(pipe.expand(output, idents));
        }

        quote! {
            {
                #(#steps)*
                ::zyn::quote::ToTokens::to_tokens(&__zyn_val, &mut #output);
            }
        }
    }
}
