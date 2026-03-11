//! ![zyn](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/banner.svg)
//!
//! A proc macro framework with templates, composable elements, and built-in diagnostics.
//!
//! ```sh
//! cargo add zyn
//! ```
//!
//! # Table of Contents
//!
//! - [Templates](#templates)
//! - [Elements](#elements)
//! - [Pipes](#pipes)
//! - [Attributes](#attributes)
//!   - [Auto-suggest](#auto-suggest)
//! - [Features](#features)
//!   - [ext](#ext)
//!   - [pretty](#pretty)
//!   - [diagnostics](#diagnostics)
//!
//! ---
//!
//! # Templates
//!
//! The [`zyn!`] macro is the core of zyn. Write token output as if it were source code,
//! with `{{ }}` interpolation and `@` control flow directives.
//!
//! **Interpolation** — any [`quote::ToTokens`] value:
//!
//! ```ignore
//! let name = zyn::format_ident!("hello_world");
//! zyn::zyn!(fn {{ name }}() {})
//! // → fn hello_world() {}
//! ```
//!
//! **Pipes** — transform values inline:
//!
//! ```ignore
//! zyn::zyn!(fn {{ name | pascal }}() {})
//! // name = "hello_world" → fn HelloWorld() {}
//! ```
//!
//! **Control flow:**
//!
//! ```ignore
//! zyn::zyn!(
//!     @if (is_pub) { pub }
//!     @for (field in fields.named.iter()) {
//!         fn {{ field.ident }}(&self) -> &{{ field.ty }} {
//!             &self.{{ field.ident }}
//!         }
//!     }
//! )
//! ```
//!
//! **Full template syntax:**
//!
//! | Syntax | Purpose |
//! |--------|---------|
//! | `{{ expr }}` | Interpolate any [`quote::ToTokens`] value |
//! | `{{ expr \| pipe }}` | Transform value through a [pipe](#pipes) before inserting |
//! | `@if (cond) { ... }` | Conditional token emission |
//! | `@else { ... }` | Else branch |
//! | `@else if (cond) { ... }` | Else-if branch |
//! | `@for (x in iter) { ... }` | Loop over an iterator |
//! | `@for (N) { ... }` | Repeat N times |
//! | `@match (expr) { pat => { ... } }` | Pattern-based emission |
//! | `@element_name(prop = val)` | Invoke a [`#[element]`](macro@element) component |
//!
//! See [`zyn!`] for the full syntax reference.
//!
//! > Templates are fully type-checked at compile time — errors appear inline, just like regular Rust code.
//!
//! ![Compile-time type safety](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-1.png)
//!
//! ---
//!
//! # Elements
//!
//! Elements are reusable template components defined with [`#[zyn::element]`](macro@element).
//! They encapsulate a fragment of token output and accept typed props.
//!
//! **Define an element:**
//!
//! ```ignore
//! #[zyn::element]
//! fn getter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
//!     zyn::zyn! {
//!         pub fn {{ name | snake | ident:"get_{}" }}(&self) -> &{{ ty }} {
//!             &self.{{ name }}
//!         }
//!     }
//! }
//! ```
//!
//! **Invoke it inside any template with `@`:**
//!
//! ```ignore
//! zyn::zyn! {
//!     impl {{ ident }} {
//!         @for (field in fields.named.iter()) {
//!             @getter(name = field.ident.clone().unwrap(), ty = field.ty.clone())
//!         }
//!     }
//! }
//! ```
//!
//! Elements can also receive **extractors** — values resolved automatically from proc macro
//! input — by marking a param with `#[zyn(input)]`:
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
//! // Applied to: struct User { first_name: String, age: u32 }
//! // Generates:
//! // impl User {
//! //     pub fn get_first_name(&self) -> &String { &self.first_name }
//! //     pub fn get_age(&self) -> &u32 { &self.age }
//! // }
//! ```
//!
//! See [`element`](macro@element) and [`derive`](macro@derive) for the full reference.
//!
//! ---
//!
//! # Pipes
//!
//! Pipes transform interpolated values: `{{ expr | pipe }}`. They chain left to right:
//!
//! ```ignore
//! zyn::zyn!(fn {{ name | snake | ident:"get_{}" }}() {})
//! // name = "HelloWorld" → fn get_hello_world() {}
//! ```
//!
//! **Built-in pipes:**
//!
//! | Pipe | Input example | Output |
//! |------|--------------|--------|
//! | `snake` | `HelloWorld` | `hello_world` |
//! | `pascal` | `hello_world` | `HelloWorld` |
//! | `camel` | `hello_world` | `helloWorld` |
//! | `screaming` | `HelloWorld` | `HELLO_WORLD` |
//! | `kebab` | `HelloWorld` | `"hello-world"` |
//! | `upper` | `hello` | `HELLO` |
//! | `lower` | `HELLO` | `hello` |
//! | `str` | `hello` | `"hello"` |
//! | `trim` | `__foo__` | `foo` |
//! | `plural` | `user` | `users` |
//! | `singular` | `users` | `user` |
//! | `ident:"pattern_{}"` | `hello` | `pattern_hello` (ident) |
//! | `fmt:"pattern_{}"` | `hello` | `"pattern_hello"` (string) |
//!
//! All built-in pipes are in the [`pipes`] module and re-exported by [`prelude`].
//!
//! **Custom pipes** via [`#[zyn::pipe]`](macro@pipe):
//!
//! ```ignore
//! #[zyn::pipe]
//! fn shout(input: String) -> zyn::syn::Ident {
//!     zyn::syn::Ident::new(&format!("{}_BANG", input.to_uppercase()), zyn::Span::call_site())
//! }
//!
//! zyn::zyn!(fn {{ name | shout }}() {})
//! // name = "hello" → fn HELLO_BANG() {}
//! ```
//!
//! See [`pipe`](macro@pipe) and the [`Pipe`] trait for the full reference.
//!
//! ---
//!
//! # Attributes
//!
//! zyn provides two tools for attribute handling: a derive macro for typed parsing and a
//! proc macro attribute for writing attribute macros.
//!
//! **Typed attribute structs** via [`#[derive(Attribute)]`](derive@Attribute):
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
//!
//! The derive generates `from_args`, `FromArg`, and `FromInput` implementations, as well as
//! a human-readable `about()` string for error messages.
//!
//! **Auto-suggest** — when a user misspells an argument name, zyn automatically suggests
//! the closest known field. No extra setup required:
//!
//! ```text
//! error: unknown argument `skiip`
//!   --> src/main.rs:5:12
//!    |
//! 5  | #[builder(skiip)]
//!    |           ^^^^^
//!    |
//!    = help: did you mean `skip`?
//! ```
//!
//! Suggestions are offered when the edit distance is ≤ 3 characters. Distant or completely
//! unknown keys produce only the "unknown argument" error without a suggestion.
//!
//! **Attribute proc macros** via [`#[zyn::attribute]`](macro@attribute):
//!
//! ```ignore
//! #[zyn::attribute]
//! fn my_attr(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> zyn::TokenStream {
//!     // args: parsed key=value arguments from the attribute invocation
//!     zyn::zyn!({ { item } })
//! }
//! ```
//!
//! See [`Attribute`](derive@Attribute) and [`attribute`](macro@attribute) for the full reference.
//!
//! ---
//!
//! # Features
//!
//! | Feature | Default | Description |
//! |---------|:-------:|-------------|
//! | `derive` | ✓ | All proc macro attributes: [`element`](macro@element), [`pipe`](macro@pipe), [`derive`](macro@derive), [`attribute`](macro@attribute), and [`Attribute`](derive@Attribute) |
//! | `ext` | | Extension traits for common `syn` AST types (`AttrExt`, `FieldExt`, `TypeExt`, etc.) — see [`ext`] |
//! | `pretty` | | Pretty-printed token output in debug mode — see [`debug`] |
//! | `diagnostics` | | Error accumulation — collect multiple errors before aborting — see [`mark`] |
//!
//! ## ext
//!
//! The [`ext`] module adds ergonomic extension traits for navigating `syn` AST types.
//!
//! ```toml
//! zyn = { features = ["ext"] }
//! ```
//!
//! ```ignore
//! use zyn::ext::{AttrExt, TypeExt};
//!
//! // check and read attribute arguments
//! if attr.is("serde") {
//!     let rename: Option<_> = attr.get("rename"); // → Some(Meta::NameValue)
//!     let skip: bool = attr.exists("skip");
//! }
//!
//! // inspect field types
//! if field.is_option() {
//!     let inner = field.inner_type().unwrap();
//! }
//! ```
//!
//! See the [`ext`] module for all available traits.
//!
//! ## pretty
//!
//! The `pretty` feature enables pretty-printed output in [`debug`] mode, formatting
//! generated token streams as readable Rust source code via `prettyplease`.
//!
//! ```toml
//! zyn = { features = ["pretty"] }
//! ```
//!
//! Enable debug output per-element with the `debug` or `debug = "pretty"` argument,
//! then set `ZYN_DEBUG="*"` at build time:
//!
//! ```ignore
//! #[zyn::element(debug = "pretty")]
//! fn my_element(name: zyn::syn::Ident) -> zyn::TokenStream {
//!     zyn::zyn!(struct {{ name }} {})
//! }
//! ```
//!
//! ```sh
//! ZYN_DEBUG="*" cargo build
//! ```
//!
//! ```text
//! note: zyn::element ─── my_element
//!
//!     struct MyElement {
//!         pub name: zyn::syn::Ident,
//!     }
//!     impl ::zyn::Render for MyElement {
//!         fn render(&self, input: &::zyn::Input) -> ::zyn::proc_macro2::TokenStream {
//!             ...
//!         }
//!     }
//! ```
//!
//! > An element annotated with `debug` — the argument is inert until `ZYN_DEBUG` is set.
//!
//! ![Element with debug arg](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-2.png)
//!
//! > Raw debug output shown as an inline compiler diagnostic in the editor.
//!
//! ![Raw debug output — inline diagnostic](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-3.png)
//!
//! > The same raw output surfaced in the Problems panel for easy navigation.
//!
//! ![Raw debug output — Problems panel](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-4.png)
//!
//! > Pretty-printed debug output — formatted with `prettyplease` for readable, indented Rust code.
//!
//! ![Pretty debug output](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-5.png)
//!
//! See the [`debug`] module for programmatic access via `DebugExt`.
//!
//! ## diagnostics
//!
//! The `diagnostics` feature enables error accumulation — collecting multiple compiler
//! errors before aborting, so users see all problems at once instead of one at a time.
//!
//! ```toml
//! zyn = { features = ["diagnostics"] }
//! ```
//!
//! Inside any `#[zyn::element]`, `#[zyn::derive]`, or `#[zyn::attribute]` body, use the
//! built-in diagnostic macros directly — no setup required:
//!
//! ```ignore
//! #[zyn::element]
//! fn my_element(name: zyn::syn::Ident) -> zyn::TokenStream {
//!     if name == "forbidden" {
//!         bail!("reserved identifier `{}`", name);
//!     }
//!
//!     if name.to_string().starts_with('_') {
//!         warn!("identifiers starting with `_` are conventionally unused");
//!     }
//!
//!     zyn::zyn!(fn {{ name }}() {})
//! }
//! ```
//!
//! | Macro | Level | Behaviour |
//! |-------|-------|-----------|
//! | `error!(msg)` | error | accumulates, does not stop execution |
//! | `warn!(msg)` | warning | accumulates, does not stop execution |
//! | `note!(msg)` | note | accumulates, does not stop execution |
//! | `help!(msg)` | help | accumulates, does not stop execution |
//! | `bail!(msg)` | error | accumulates and immediately returns |
//!
//! All accumulated diagnostics are emitted together at the end of the element or macro body,
//! so users see every error at once instead of fixing them one by one.
//!
//! ```text
//! error: reserved identifier `forbidden`
//!  --> src/main.rs:3:1
//!
//! error: reserved identifier `forbidden`
//!  --> src/main.rs:7:1
//! ```
//!
//! See the [`mark`] module for the lower-level diagnostic builder API.

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
