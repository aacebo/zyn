pub mod at;
mod group_node;
mod interp_node;
mod pipe_node;
mod tokens_node;

pub use at::*;
pub use group_node::GroupNode;
pub use interp_node::InterpNode;
pub use pipe_node::PipeNode;
pub use tokens_node::TokensNode;

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
use crate::ident;

pub enum Node {
    Tokens(TokensNode),
    Interp(InterpNode),
    At(AtNode),
    Group(GroupNode),
}

impl Node {
    pub fn is_tokens(&self) -> bool {
        matches!(self, Self::Tokens(_))
    }

    pub fn is_interp(&self) -> bool {
        matches!(self, Self::Interp(_))
    }

    pub fn is_at(&self) -> bool {
        matches!(self, Self::At(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }
}

impl Node {
    pub fn as_tokens(&self) -> &TokensNode {
        match self {
            Self::Tokens(v) => v,
            _ => panic!("called as_tokens on non-Tokens node"),
        }
    }

    pub fn as_interp(&self) -> &InterpNode {
        match self {
            Self::Interp(v) => v,
            _ => panic!("called as_interp on non-Interp node"),
        }
    }

    pub fn as_at(&self) -> &AtNode {
        match self {
            Self::At(v) => v,
            _ => panic!("called as_at on non-At node"),
        }
    }

    pub fn as_group(&self) -> &GroupNode {
        match self {
            Self::Group(v) => v,
            _ => panic!("called as_group on non-Group node"),
        }
    }
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Self::Tokens(v) => v.span(),
            Self::Interp(v) => v.span(),
            Self::At(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}

impl From<TokensNode> for Node {
    fn from(v: TokensNode) -> Self {
        Self::Tokens(v)
    }
}

impl From<InterpNode> for Node {
    fn from(v: InterpNode) -> Self {
        Self::Interp(v)
    }
}

impl From<AtNode> for Node {
    fn from(v: AtNode) -> Self {
        Self::At(v)
    }
}

impl From<GroupNode> for Node {
    fn from(v: GroupNode) -> Self {
        Self::Group(v)
    }
}

impl Node {
    pub fn parse_brace(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = syn::braced!(content in input);
        let span = brace.span.join();

        let fork = content.fork();
        let is_interp = if fork.peek(syn::token::Brace) {
            let inner;
            syn::braced!(inner in fork);
            fork.is_empty()
        } else {
            false
        };

        if is_interp {
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

            Ok(InterpNode { span, expr, pipes }.into())
        } else {
            let body = content.parse::<Element>()?;
            Ok(GroupNode {
                span,
                delimiter: proc_macro2::Delimiter::Brace,
                body: Box::new(body),
            }
            .into())
        }
    }
}

impl Expand for Node {
    fn expand(&self, output: &Ident, idents: &mut ident::Iter) -> TokenStream {
        match self {
            Self::Tokens(v) => v.expand(output, idents),
            Self::Interp(v) => v.expand(output, idents),
            Self::At(v) => v.expand(output, idents),
            Self::Group(v) => v.expand(output, idents),
        }
    }
}

pub struct Element {
    pub nodes: Vec<Node>,
}

impl Element {
    pub fn span(&self) -> Span {
        self.nodes
            .first()
            .map(|n| n.span())
            .unwrap_or_else(Span::call_site)
    }
}

impl Expand for Element {
    fn expand(&self, output: &Ident, idents: &mut ident::Iter) -> TokenStream {
        let mut result = TokenStream::new();

        for node in &self.nodes {
            result.extend(node.expand(output, idents));
        }

        result
    }
}

impl Element {
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
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        let mut pending = TokenStream::new();

        while !input.is_empty() {
            if input.peek(Token![@]) {
                flush(&mut pending, &mut nodes);
                nodes.push(input.parse::<AtNode>()?.into());
            } else if input.peek(syn::token::Brace) {
                flush(&mut pending, &mut nodes);
                nodes.push(Node::parse_brace(input)?);
            } else if input.peek(syn::token::Paren) || input.peek(syn::token::Bracket) {
                flush(&mut pending, &mut nodes);
                nodes.push(input.parse::<GroupNode>()?.into());
            } else {
                let tt: TokenTree = input.parse()?;
                tt.to_tokens(&mut pending);
            }
        }

        flush(&mut pending, &mut nodes);
        Ok(Self { nodes })
    }
}

fn flush(pending: &mut TokenStream, nodes: &mut Vec<Node>) {
    if pending.is_empty() {
        return;
    }

    let span = pending
        .clone()
        .into_iter()
        .next()
        .map(|tt| tt.span())
        .unwrap_or_else(Span::call_site);

    nodes.push(
        TokensNode {
            span,
            stream: pending.clone(),
        }
        .into(),
    );

    *pending = TokenStream::new();
}
