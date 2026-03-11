//! Proc macro input types.
//!
//! [`Input`] is the unified wrapper for both derive and attribute macro input.
//! It wraps either a [`syn::DeriveInput`] or a [`syn::Item`] and exposes a
//! common accessor surface.
//!
//! # Examples
//!
//! Parsing a struct definition into `Input`:
//!
//! ```ignore
//! use zyn_core::Input;
//!
//! let ts = quote::quote! { pub struct Foo<T> { value: T }};
//! let input: Input = syn::parse2(ts).unwrap();
//!
//! assert_eq!(input.ident().to_string(), "Foo");
//! // input.generics() → <T>
//! // input.vis()      → pub
//! ```
//!
//! `Input` implements `Default` for use in tests — the default is a struct named
//! `__ZynDefault` with no fields or generics:
//!
//! ```ignore
//! let input = Input::default();
//! assert_eq!(input.ident().to_string(), "__ZynDefault");
//! ```

mod derive;
mod item;
mod output;

pub use output::Output;
pub use output::OutputBuilder;

use quote::ToTokens;

/// The proc macro input context. Wraps either a `DeriveInput` or an `Item`.
#[derive(Clone)]
pub enum Input {
    Derive(syn::DeriveInput),
    Item(syn::Item),
}

impl Input {
    /// Returns `true` if the input is a derive macro input.
    pub fn is_derive(&self) -> bool {
        matches!(self, Self::Derive(_))
    }

    /// Returns `true` if the input is an attribute macro item.
    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }
}

impl Input {
    /// Returns the inner `DeriveInput` if this is a derive input.
    pub fn as_derive(&self) -> Option<&syn::DeriveInput> {
        match self {
            Self::Derive(d) => Some(d),
            _ => None,
        }
    }

    /// Returns the inner `Item` if this is an attribute macro item.
    pub fn as_item(&self) -> Option<&syn::Item> {
        match self {
            Self::Item(i) => Some(i),
            _ => None,
        }
    }
}

impl Input {
    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Derive(d) => &d.attrs,
            Self::Item(i) => match i {
                syn::Item::Const(v) => &v.attrs,
                syn::Item::Enum(v) => &v.attrs,
                syn::Item::ExternCrate(v) => &v.attrs,
                syn::Item::Fn(v) => &v.attrs,
                syn::Item::ForeignMod(v) => &v.attrs,
                syn::Item::Impl(v) => &v.attrs,
                syn::Item::Mod(v) => &v.attrs,
                syn::Item::Static(v) => &v.attrs,
                syn::Item::Struct(v) => &v.attrs,
                syn::Item::Trait(v) => &v.attrs,
                syn::Item::Type(v) => &v.attrs,
                syn::Item::Union(v) => &v.attrs,
                syn::Item::Use(v) => &v.attrs,
                _ => &[],
            },
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Derive(d) => &d.ident,
            Self::Item(i) => match i {
                syn::Item::Const(v) => &v.ident,
                syn::Item::Enum(v) => &v.ident,
                syn::Item::ExternCrate(v) => &v.ident,
                syn::Item::Fn(v) => &v.sig.ident,
                syn::Item::Mod(v) => &v.ident,
                syn::Item::Static(v) => &v.ident,
                syn::Item::Struct(v) => &v.ident,
                syn::Item::Trait(v) => &v.ident,
                syn::Item::Type(v) => &v.ident,
                syn::Item::Union(v) => &v.ident,
                _ => panic!("item variant has no ident"),
            },
        }
    }

    pub fn generics(&self) -> &syn::Generics {
        match self {
            Self::Derive(d) => &d.generics,
            Self::Item(i) => match i {
                syn::Item::Enum(v) => &v.generics,
                syn::Item::Fn(v) => &v.sig.generics,
                syn::Item::Impl(v) => &v.generics,
                syn::Item::Struct(v) => &v.generics,
                syn::Item::Trait(v) => &v.generics,
                syn::Item::Type(v) => &v.generics,
                syn::Item::Union(v) => &v.generics,
                _ => panic!("item variant has no generics"),
            },
        }
    }

    pub fn vis(&self) -> &syn::Visibility {
        match self {
            Self::Derive(d) => &d.vis,
            Self::Item(i) => match i {
                syn::Item::Const(v) => &v.vis,
                syn::Item::Enum(v) => &v.vis,
                syn::Item::ExternCrate(v) => &v.vis,
                syn::Item::Fn(v) => &v.vis,
                syn::Item::Mod(v) => &v.vis,
                syn::Item::Static(v) => &v.vis,
                syn::Item::Struct(v) => &v.vis,
                syn::Item::Trait(v) => &v.vis,
                syn::Item::Type(v) => &v.vis,
                syn::Item::Union(v) => &v.vis,
                syn::Item::Use(v) => &v.vis,
                _ => panic!("item variant has no visibility"),
            },
        }
    }

    pub fn span(&self) -> proc_macro2::Span {
        use syn::spanned::Spanned;
        match self {
            Self::Derive(d) => d.span(),
            Self::Item(i) => i.span(),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Derive(syn::parse_str("struct __ZynDefault;").unwrap())
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Derive(d) => d.to_tokens(tokens),
            Self::Item(i) => i.to_tokens(tokens),
        }
    }
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative;

        let fork = input.fork();
        if let Ok(d) = fork.parse::<syn::DeriveInput>() {
            input.advance_to(&fork);
            return Ok(Self::Derive(d));
        }

        Ok(Self::Item(input.parse()?))
    }
}

impl From<syn::DeriveInput> for Input {
    fn from(v: syn::DeriveInput) -> Self {
        Self::Derive(v)
    }
}

impl From<syn::Item> for Input {
    fn from(v: syn::Item) -> Self {
        Self::Item(v)
    }
}
