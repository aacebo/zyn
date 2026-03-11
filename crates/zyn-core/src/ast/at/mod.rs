//! `@`-prefixed control-flow and component nodes.
//!
//! An [`AtNode`] is produced whenever the parser encounters `@` in a template.
//! The keyword or identifier following `@` determines the variant:
//!
//! | Syntax | Variant |
//! |--------|---------|
//! | `@if (cond) { ... }` | [`AtNode::If`] |
//! | `@for (x in iter) { ... }` | [`AtNode::For`] |
//! | `@match (expr) { ... }` | [`AtNode::Match`] |
//! | `@my_element(...)` | [`AtNode::Element`] |

mod element_node;
mod for_node;
mod if_node;
mod match_node;

pub use element_node::ElementNode;
pub use for_node::ForNode;
pub use if_node::IfNode;
pub use match_node::MatchNode;

use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use syn::Token;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;

/// An `@`-prefixed statement in a zyn template.
///
/// See the [`at`](crate::ast::at) module for the syntax of each variant.
pub enum AtNode {
    /// `@if (cond) { ... } @else if (...) { ... } @else { ... }`
    If(IfNode),
    /// `@for (item in expr) { ... }` or `@for (count) { ... }`
    For(ForNode),
    /// `@match (expr) { pattern => { ... }, ... }`
    Match(MatchNode),
    /// `@component_name(prop = val, ...) { children }`
    Element(ElementNode),
}

impl AtNode {
    /// Returns `true` if this is an [`AtNode::If`] variant.
    pub fn is_if(&self) -> bool {
        matches!(self, Self::If(_))
    }

    /// Returns `true` if this is an [`AtNode::For`] variant.
    pub fn is_for(&self) -> bool {
        matches!(self, Self::For(_))
    }

    /// Returns `true` if this is an [`AtNode::Match`] variant.
    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    /// Returns `true` if this is an [`AtNode::Element`] variant.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element(_))
    }
}

impl AtNode {
    /// Returns the inner [`IfNode`]. Panics if not an `If` variant.
    pub fn as_if(&self) -> &IfNode {
        match self {
            Self::If(v) => v,
            _ => panic!("called as_if on non-If node"),
        }
    }

    /// Returns the inner [`ForNode`]. Panics if not a `For` variant.
    pub fn as_for(&self) -> &ForNode {
        match self {
            Self::For(v) => v,
            _ => panic!("called as_for on non-For node"),
        }
    }

    /// Returns the inner [`MatchNode`]. Panics if not a `Match` variant.
    pub fn as_match(&self) -> &MatchNode {
        match self {
            Self::Match(v) => v,
            _ => panic!("called as_match on non-Match node"),
        }
    }

    /// Returns the inner [`ElementNode`]. Panics if not an `Element` variant.
    pub fn as_element(&self) -> &ElementNode {
        match self {
            Self::Element(v) => v,
            _ => panic!("called as_element on non-Element node"),
        }
    }
}

impl AtNode {
    /// Returns the source span of this node.
    pub fn span(&self) -> Span {
        match self {
            Self::If(v) => v.span(),
            Self::For(v) => v.span(),
            Self::Match(v) => v.span(),
            Self::Element(v) => v.span(),
        }
    }
}

impl From<IfNode> for AtNode {
    fn from(v: IfNode) -> Self {
        Self::If(v)
    }
}

impl From<ForNode> for AtNode {
    fn from(v: ForNode) -> Self {
        Self::For(v)
    }
}

impl From<MatchNode> for AtNode {
    fn from(v: MatchNode) -> Self {
        Self::Match(v)
    }
}

impl From<ElementNode> for AtNode {
    fn from(v: ElementNode) -> Self {
        Self::Element(v)
    }
}

impl Expand for AtNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        match self {
            Self::If(v) => v.expand(output, idents),
            Self::For(v) => v.expand(output, idents),
            Self::Match(v) => v.expand(output, idents),
            Self::Element(v) => v.expand(output, idents),
        }
    }
}

impl Parse for AtNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let at_span = input.parse::<Token![@]>()?.span;
        let ident: syn::Ident = input.call(syn::Ident::parse_any)?;
        let name = ident.to_string();

        match name.as_str() {
            "if" => {
                let mut v = input.parse::<IfNode>()?;
                v.span = at_span;
                Ok(v.into())
            }
            "for" => {
                let mut v = input.parse::<ForNode>()?;
                v.span = at_span;
                Ok(v.into())
            }
            "match" => {
                let mut v = input.parse::<MatchNode>()?;
                v.span = at_span;
                Ok(v.into())
            }
            "else" => Err(syn::Error::new(at_span, "unexpected @else without @if")),
            _ => {
                let mut v = ElementNode::parse_with_ident(input, ident)?;
                v.span = at_span;
                Ok(v.into())
            }
        }
    }
}
