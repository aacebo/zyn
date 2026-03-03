mod element_node;
mod for_node;
mod if_node;
mod match_node;
mod throw_node;

pub use element_node::ElementNode;
pub use for_node::ForNode;
pub use if_node::IfNode;
pub use match_node::MatchNode;
pub use throw_node::ThrowNode;

use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use syn::Token;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;

pub enum AtNode {
    If(IfNode),
    For(ForNode),
    Match(MatchNode),
    Throw(ThrowNode),
    Element(ElementNode),
}

impl AtNode {
    pub fn is_if(&self) -> bool {
        matches!(self, Self::If(_))
    }

    pub fn is_for(&self) -> bool {
        matches!(self, Self::For(_))
    }

    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    pub fn is_throw(&self) -> bool {
        matches!(self, Self::Throw(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element(_))
    }
}

impl AtNode {
    pub fn as_if(&self) -> &IfNode {
        match self {
            Self::If(v) => v,
            _ => panic!("called as_if on non-If node"),
        }
    }

    pub fn as_for(&self) -> &ForNode {
        match self {
            Self::For(v) => v,
            _ => panic!("called as_for on non-For node"),
        }
    }

    pub fn as_match(&self) -> &MatchNode {
        match self {
            Self::Match(v) => v,
            _ => panic!("called as_match on non-Match node"),
        }
    }

    pub fn as_throw(&self) -> &ThrowNode {
        match self {
            Self::Throw(v) => v,
            _ => panic!("called as_throw on non-Throw node"),
        }
    }

    pub fn as_element(&self) -> &ElementNode {
        match self {
            Self::Element(v) => v,
            _ => panic!("called as_element on non-Element node"),
        }
    }
}

impl AtNode {
    pub fn span(&self) -> Span {
        match self {
            Self::If(v) => v.span(),
            Self::For(v) => v.span(),
            Self::Match(v) => v.span(),
            Self::Throw(v) => v.span(),
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

impl From<ThrowNode> for AtNode {
    fn from(v: ThrowNode) -> Self {
        Self::Throw(v)
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
            Self::Throw(v) => v.expand(output, idents),
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
            "throw" => {
                let mut v = input.parse::<ThrowNode>()?;
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
