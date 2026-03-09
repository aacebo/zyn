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
    /// Returns the attributes on the input item.
    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Derive(d) => &d.attrs,
            Self::Item(i) => item::attrs(i),
        }
    }

    /// Returns the identifier of the input item.
    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Derive(d) => &d.ident,
            Self::Item(i) => item::ident(i),
        }
    }

    /// Returns the generics of the input item.
    pub fn generics(&self) -> &syn::Generics {
        match self {
            Self::Derive(d) => &d.generics,
            Self::Item(i) => item::generics(i),
        }
    }

    /// Returns the visibility of the input item.
    pub fn vis(&self) -> &syn::Visibility {
        match self {
            Self::Derive(d) => &d.vis,
            Self::Item(i) => item::vis(i),
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
