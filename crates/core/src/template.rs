//! Template parsing and expansion.
//!
//! In normal use, the `zyn!` proc macro compiles templates at macro-expansion time.
//! [`Template`] is primarily useful for inspecting the parsed AST or building
//! templates dynamically.
//!
//! # Examples
//!
//! Parsing a template and inspecting its nodes:
//!
//! ```ignore
//! use zyn_core::template::Template;
//!
//! let tmpl: Template = syn::parse_str("fn {{ name }}() {}").unwrap();
//! // tmpl.nodes → [Tokens("fn"), Interp(name), Tokens("() {}")]
//! assert_eq!(tmpl.nodes.len(), 3);
//! ```
//!
//! `to_token_stream()` and `render(&input)` produce proc macro output code —
//! `TokenStream` that, when returned from a proc macro, generates the expanded
//! template in the user's crate. `render` additionally binds `input` in scope
//! so templates can reference it directly.

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
use crate::ast::AtNode;
use crate::ast::GroupNode;
use crate::ast::Node;
use crate::ast::TokensNode;
use crate::ident;
use crate::types::Input;

/// A parsed template AST. Created by parsing template syntax via `syn::parse2::<Template>(tokens)`.
pub struct Template {
    pub nodes: Vec<Node>,
}

impl Template {
    /// Returns the source span of the first node, or `Span::call_site()` if empty.
    pub fn span(&self) -> Span {
        self.nodes
            .first()
            .map(|n| n.span())
            .unwrap_or_else(Span::call_site)
    }

    /// Expands the template into a `TokenStream` without an `Input` binding.
    pub fn to_token_stream(&self) -> TokenStream {
        let mut idents = ident::Iter::new();
        let output = idents.next().unwrap();
        let expanded = self.expand(&output, &mut idents);

        quote! {
            {
                let mut #output = ::zyn::proc_macro2::TokenStream::new();
                #expanded
                #output
            }
        }
    }

    /// Expands the template with the given `Input` bound as `input` in the generated code.
    pub fn render(&self, input: &Input) -> TokenStream {
        let expanded = self.to_token_stream();
        quote! {
            {
                let input: ::zyn::Input = ::zyn::parse!(#input).unwrap();
                #expanded
            }
        }
    }

    fn flush(pending: &mut TokenStream, nodes: &mut Vec<Node>) {
        if pending.is_empty() {
            return;
        }

        let stream = std::mem::take(pending);
        let span = stream
            .clone()
            .into_iter()
            .next()
            .map(|tt| tt.span())
            .unwrap_or_else(Span::call_site);

        nodes.push(TokensNode { span, stream }.into());
    }
}

impl Expand for Template {
    fn expand(&self, output: &Ident, idents: &mut ident::Iter) -> TokenStream {
        let mut result = TokenStream::new();

        for node in &self.nodes {
            result.extend(node.expand(output, idents));
        }

        result
    }
}

impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        let mut pending = TokenStream::new();

        while !input.is_empty() {
            if input.peek(Token![@]) {
                Self::flush(&mut pending, &mut nodes);
                nodes.push(input.parse::<AtNode>()?.into());
            } else if input.peek(syn::token::Brace) {
                Self::flush(&mut pending, &mut nodes);
                nodes.push(Node::parse_brace(input)?);
            } else if input.peek(syn::token::Paren) || input.peek(syn::token::Bracket) {
                Self::flush(&mut pending, &mut nodes);
                nodes.push(input.parse::<GroupNode>()?.into());
            } else {
                let tt: TokenTree = input.parse()?;
                tt.to_tokens(&mut pending);
            }
        }

        Self::flush(&mut pending, &mut nodes);
        Ok(Self { nodes })
    }
}
