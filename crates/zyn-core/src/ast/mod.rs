//! Template AST node types.
//!
//! A [`crate::template::Template`] is a flat sequence of [`Node`]s. The node type
//! is determined by the template syntax:
//!
//! | Template syntax | Node variant |
//! |----------------|-------------|
//! | `fn foo() {}` (literal tokens) | [`Node::Tokens`] |
//! | `{{ name \| snake }}` | [`Node::Interp`] |
//! | `@if`, `@for`, `@match`, `@element` | [`Node::At`] |
//! | `(...)`, `[...]`, `{...}` groups | [`Node::Group`] |
//!
//! These types are public for inspection but are not typically used directly —
//! the `zyn!` macro handles parsing and expansion internally.

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

use syn::Token;
use syn::parse::ParseStream;

use crate::Expand;
use crate::ident;

/// A single node in a parsed zyn template.
///
/// See the [`ast`](crate::ast) module for the full node taxonomy.
pub enum Node {
    /// Literal Rust tokens passed through unchanged.
    Tokens(TokensNode),
    /// An interpolation expression: `{{ expr }}` or `{{ expr | pipe | ... }}`.
    Interp(InterpNode),
    /// An `@`-prefixed control-flow or element statement.
    At(AtNode),
    /// A delimited group: `(...)`, `[...]`, or `{...}`.
    Group(GroupNode),
}

impl Node {
    /// Returns `true` if this is a [`Node::Tokens`] variant.
    pub fn is_tokens(&self) -> bool {
        matches!(self, Self::Tokens(_))
    }

    /// Returns `true` if this is a [`Node::Interp`] variant.
    pub fn is_interp(&self) -> bool {
        matches!(self, Self::Interp(_))
    }

    /// Returns `true` if this is a [`Node::At`] variant.
    pub fn is_at(&self) -> bool {
        matches!(self, Self::At(_))
    }

    /// Returns `true` if this is a [`Node::Group`] variant.
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }
}

impl Node {
    /// Returns the inner [`TokensNode`]. Panics if not a `Tokens` variant.
    pub fn as_tokens(&self) -> &TokensNode {
        match self {
            Self::Tokens(v) => v,
            _ => panic!("called as_tokens on non-Tokens node"),
        }
    }

    /// Returns the inner [`InterpNode`]. Panics if not an `Interp` variant.
    pub fn as_interp(&self) -> &InterpNode {
        match self {
            Self::Interp(v) => v,
            _ => panic!("called as_interp on non-Interp node"),
        }
    }

    /// Returns the inner [`AtNode`]. Panics if not an `At` variant.
    pub fn as_at(&self) -> &AtNode {
        match self {
            Self::At(v) => v,
            _ => panic!("called as_at on non-At node"),
        }
    }

    /// Returns the inner [`GroupNode`]. Panics if not a `Group` variant.
    pub fn as_group(&self) -> &GroupNode {
        match self {
            Self::Group(v) => v,
            _ => panic!("called as_group on non-Group node"),
        }
    }
}

impl Node {
    /// Returns the source span of this node.
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
            let body = content.parse::<crate::template::Template>()?;
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
