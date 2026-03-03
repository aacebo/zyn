use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;
use quote::quote;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;

pub struct PipeNode {
    pub span: Span,
    pub name: syn::Ident,
    pub args: Vec<TokenStream>,
}

impl PipeNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for PipeNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let span = name.span();

        let mut args = Vec::new();

        while input.peek(Token![:]) {
            input.parse::<Token![:]>()?;

            let mut arg = TokenStream::new();

            while !input.is_empty() && !input.peek(Token![:]) && !input.peek(Token![|]) {
                let tt: TokenTree = input.parse()?;
                tt.to_tokens(&mut arg);
            }

            args.push(arg);
        }

        Ok(Self { span, name, args })
    }
}

impl Expand for PipeNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let name = &self.name;
        quote! { let __zyn_val = ::zyn::Pipe::pipe(&(#name), __zyn_val); }
    }
}
