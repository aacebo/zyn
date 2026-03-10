//! Debug formatting utilities for macro expansions.
//!
//! Provides a pipeline API for formatting generated `TokenStream`s into
//! human-readable strings. Used internally by the `debug` attribute argument
//! on `#[zyn::element]`, `#[zyn::pipe]`, `#[zyn::derive]`, and `#[zyn::attribute]`.
//!
//! # Formats
//!
//! - **Raw** (always available) — calls `TokenStream::to_string()`, producing
//!   a flat single-line string with fully-qualified paths. No extra dependencies.
//!
//! - **Pretty** (requires the `pretty` feature) — parses the token stream into
//!   a [`syn::File`] and formats it with [`prettyplease`], producing properly
//!   indented Rust code. Enable with:
//!
//!   ```toml
//!   zyn = { version = "0.3", features = ["pretty"] }
//!   ```
//!
//! # Usage
//!
//! The [`DebugExt`] trait adds a `.debug()` method to [`proc_macro2::TokenStream`],
//! returning a [`DebugTokens`] builder with `.raw()` and `.pretty()` methods:
//!
//! ```ignore
//! use zyn::debug::DebugExt;
//!
//! let raw: String = tokens.debug().raw();
//!
//! #[cfg(feature = "pretty")]
//! let pretty: String = tokens.debug().pretty();
//! ```
//!
//! # Macro integration
//!
//! In attribute macros, add `debug` or `debug = "pretty"` to emit the generated
//! code as a compiler `note` diagnostic. Requires `ZYN_DEBUG` to be set:
//!
//! ```ignore
//! #[zyn::element(debug)]
//! fn greeting(name: syn::Ident) -> zyn::TokenStream { ... }
//!
//! // ZYN_DEBUG="*" cargo build
//! // → note: zyn::element ─── Greeting
//! //         struct Greeting { pub name : syn :: Ident , } impl ...
//! ```

#[cfg(feature = "pretty")]
mod pretty;

/// Wraps a reference to a [`proc_macro2::TokenStream`] and provides formatting
/// methods for debug output.
///
/// Created by [`DebugExt::debug()`]. Use [`.raw()`](Self::raw) for unformatted
/// output or [`.pretty()`](Self::pretty) for `prettyplease`-formatted output.
pub struct DebugTokens<'a> {
    tokens: &'a proc_macro2::TokenStream,
}

/// Extension trait that adds `.debug()` to [`proc_macro2::TokenStream`].
///
/// # Example
///
/// ```ignore
/// use zyn::debug::DebugExt;
///
/// let output: String = tokens.debug().raw();
/// ```
pub trait DebugExt {
    fn debug(&self) -> DebugTokens<'_>;
}

impl DebugExt for proc_macro2::TokenStream {
    fn debug(&self) -> DebugTokens<'_> {
        DebugTokens { tokens: self }
    }
}

impl DebugTokens<'_> {
    /// Returns the raw `TokenStream::to_string()` output.
    ///
    /// This is a flat, single-line string with fully-qualified paths and spaces
    /// between all tokens. No extra dependencies required.
    pub fn raw(&self) -> String {
        self.tokens.to_string()
    }

    /// Returns the token stream formatted as proper Rust code using `prettyplease`.
    ///
    /// Falls back to `TokenStream::to_string()` if the tokens cannot be parsed
    /// as a valid Rust file.
    ///
    /// Requires the `pretty` feature:
    ///
    /// ```toml
    /// zyn = { version = "0.3", features = ["pretty"] }
    /// ```
    #[cfg(feature = "pretty")]
    pub fn pretty(&self) -> String {
        pretty::pretty(self.tokens)
    }
}
