//! A template engine and framework for Rust procedural macros.
//!
//! zyn replaces the typical `syn` + `quote` + `heck` + `proc-macro-error` stack with
//! a single dependency. It provides a `zyn!` template macro with interpolation, control
//! flow, and pipes — plus reusable elements, typed attribute parsing, and proc macro
//! entry points.
//!
//! # Quick start
//!
//! ```sh
//! cargo add zyn
//! ```
//!
//! A derive macro that generates getters:
//!
//! ```ignore
//! #[zyn::derive]
//! fn my_getters(
//!     #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
//!     #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
//! ) -> zyn::TokenStream {
//!     zyn::zyn! {
//!         impl {{ ident }} {
//!             @for (field in fields.named.iter()) {
//!                 pub fn {{ field.ident | snake | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
//!                     &self.{{ field.ident }}
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Applied to `struct User { first_name: String, age: u32 }` generates:
//!
//! ```ignore
//! impl User {
//!     pub fn get_first_name(&self) -> &String { &self.first_name }
//!     pub fn get_age(&self) -> &u32 { &self.age }
//! }
//! ```
//!
//! # Template syntax
//!
//! | Syntax | Purpose |
//! |--------|---------|
//! | `{{ expr }}` | Interpolate any [`quote::ToTokens`] value |
//! | `{{ expr \| pipe }}` | Transform value through a pipe before inserting |
//! | `@if (cond) { ... }` | Conditional token emission |
//! | `@for (x in iter) { ... }` | Loop over an iterator |
//! | `@for (N) { ... }` | Repeat N times |
//! | `@match (expr) { pat => { ... } }` | Pattern-based code generation |
//! | `@element_name(prop = val)` | Invoke a `#[zyn::element]` component |
//!
//! See [`zyn!`] for the full syntax reference with examples.
//!
//! # Pipes
//!
//! 13 built-in pipes: `snake`, `pascal`, `camel`, `screaming`, `kebab`, `upper`,
//! `lower`, `str`, `trim`, `plural`, `singular`, `ident`, `fmt`. They chain:
//!
//! ```ignore
//! zyn::zyn! { fn {{ name | snake | ident:"get_{}" }}() {} }
//! // name = "HelloWorld" → fn get_hello_world() {}
//! ```
//!
//! Custom pipes via [`pipe`].
//!
//! # Elements
//!
//! ```ignore
//! #[zyn::element]
//! fn getter(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
//!     zyn::zyn! {
//!         pub fn {{ name | snake | ident:"get_{}" }}(&self) -> &{{ ty }} {
//!             &self.{{ name }}
//!         }
//!     }
//! }
//!
//! // invoked as:
//! zyn::zyn! { @getter(name = field.ident.clone().unwrap(), ty = field.ty.clone()) }
//! ```
//!
//! # Typed attribute parsing
//!
//! ```ignore
//! #[derive(zyn::Attribute)]
//! #[zyn("builder")]
//! struct BuilderConfig {
//!     #[zyn(default)]
//!     skip: bool,
//!     #[zyn(default = "build".to_string())]
//!     method: String,
//! }
//! // users write: #[builder(skip)] or #[builder(method = "create")]
//! ```

pub use zyn_core::*;

#[cfg(feature = "derive")]
pub use zyn_derive::*;

/// The zyn prelude. Re-exports all built-in pipes, core traits, and proc macros.
pub mod prelude {
    pub use crate::pipes::*;
    pub use crate::{Pipe, Render};

    #[cfg(feature = "derive")]
    pub use zyn_derive::*;

    #[cfg(feature = "ext")]
    pub use zyn_core::ext::*;
}
