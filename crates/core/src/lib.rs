//! Core library for zyn — AST, traits, types, and utilities for procedural macro development.

/// Template AST node types.
pub mod ast;
/// Case conversion utilities.
pub mod case;
/// Debug formatting and printing for template expansions.
pub mod debug;
/// Diagnostic accumulation and emission.
pub mod diagnostic;
/// Extractors for resolving values from proc macro input.
pub mod extract;
/// Internal identifier generation for template expansion.
pub mod ident;
/// Attribute argument parsing types.
pub mod meta;
/// Dot-separated path type for navigating nested `syn` metadata.
pub mod path;
/// Built-in pipe types for template value transforms.
pub mod pipes;
/// Template parsing and expansion.
pub mod template;
/// Proc macro input types.
pub mod types;

/// Extension traits for common `syn` AST types.
#[cfg(feature = "ext")]
pub mod ext;

pub use diagnostic::*;
pub use extract::*;
pub use meta::*;
pub use template::Template;
pub use types::Input;

/// A specialized [`Result`](std::result::Result) type for zyn diagnostics.
pub type Result<T> = diagnostic::Result<T>;

/// Parses tokens or string literals into a type. Wraps `syn::parse_str` and `syn::parse2`.
#[macro_export]
macro_rules! parse {
    ($s:literal => $ty:ty) => {
        $crate::syn::parse_str::<$ty>($s)
    };
    ($s:literal) => {
        $crate::syn::parse_str($s)
    };
    ($ts:expr => $ty:ty) => {
        $crate::syn::parse2::<$ty>($ts)
    };
    ($ts:expr) => {
        $crate::syn::parse2($ts)
    };
}

/// Parses a `proc_macro::TokenStream` in a proc macro entry point. Wraps `syn::parse_macro_input!`.
#[macro_export]
macro_rules! parse_input {
    ($($tt:tt)*) => { $crate::syn::parse_macro_input!($($tt)*) }
}

pub use proc_macro2::{Span, TokenStream};
pub use quote::{ToTokens, format_ident};

pub use proc_macro2;
pub use quote;
pub use syn;

/// Internal trait for AST node expansion. Not part of the public API.
pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

/// Implemented by `#[zyn::element]` types. Renders the element with the given `Input` context.
pub trait Render {
    fn render(&self, input: &types::Input) -> proc_macro2::TokenStream;
}

/// Implemented by `#[zyn::pipe]` types. Transforms a value in a pipe chain.
pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
