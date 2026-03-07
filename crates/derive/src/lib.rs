//! Procedural macros for the zyn framework.
//!
//! Re-exported through the root `zyn` crate. All macros are accessed as
//! `zyn::zyn!`, `#[zyn::element]`, etc.
//!
//! # Quick reference
//!
//! ```ignore
//! // Template expansion
//! zyn::zyn! { fn {{ name | snake }}() {} }
//!
//! // Reusable component
//! #[zyn::element]
//! fn my_getter(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream { ... }
//!
//! // Derive macro entry point
//! #[zyn::derive]
//! fn my_derive(
//!     #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
//!     #[zyn(input)] fields: zyn::Fields,
//! ) -> zyn::TokenStream { ... }
//!
//! // Typed attribute parsing
//! #[derive(zyn::Attribute)]
//! #[zyn("my_attr")]
//! struct MyAttr { skip: bool, rename: Option<String> }
//! ```

mod attribute;
mod common;
mod macros;

/// Expands a zyn template into a [`proc_macro2::TokenStream`].
///
/// Everything outside `{{ }}` and `@` directives passes through as literal tokens.
///
/// # Interpolation
///
/// `{{ expr }}` inserts any [`quote::ToTokens`] value:
///
/// ```ignore
/// let name = format_ident!("my_fn");
/// zyn! { fn {{ name }}() {} }
/// // output: fn my_fn() {}
/// ```
///
/// # Pipes
///
/// `{{ expr | pipe }}` transforms the value before inserting it. Pipes chain left to right:
///
/// ```ignore
/// zyn! { fn {{ name | snake }}() {} }
/// // name = "HelloWorld" → fn hello_world() {}
///
/// zyn! { fn {{ name | snake | ident:"get_{}" }}() {} }
/// // name = "HelloWorld" → fn get_hello_world() {}
/// ```
///
/// Built-in pipes:
///
/// | Pipe | Input | Output |
/// |------|-------|--------|
/// | `snake` | `HelloWorld` | `hello_world` |
/// | `pascal` | `hello_world` | `HelloWorld` |
/// | `camel` | `hello_world` | `helloWorld` |
/// | `screaming` | `HelloWorld` | `HELLO_WORLD` |
/// | `kebab` | `HelloWorld` | `"hello-world"` (string literal) |
/// | `upper` | `hello` | `HELLO` |
/// | `lower` | `HELLO` | `hello` |
/// | `str` | `hello` | `"hello"` (string literal) |
/// | `plural` | `user` | `users` |
/// | `singular` | `users` | `user` |
/// | `ident:"pattern_{}"` | `hello` | `pattern_hello` (ident) |
/// | `fmt:"pattern_{}"` | `hello` | `"pattern_hello"` (string literal) |
/// | `trim` | `__foo__` | `foo` |
///
/// # `@if`
///
/// ```ignore
/// zyn! {
///     @if (is_async) {
///         async fn {{ name }}() {}
///     } @else if (is_unsafe) {
///         unsafe fn {{ name }}() {}
///     } @else {
///         fn {{ name }}() {}
///     }
/// }
/// ```
///
/// Without `@else`, emits nothing when false:
///
/// ```ignore
/// zyn! { @if (is_pub) { pub } fn {{ name }}() {} }
/// // is_pub = true  → pub fn my_fn() {}
/// // is_pub = false →     fn my_fn() {}
/// ```
///
/// # `@for`
///
/// Iterator form:
///
/// ```ignore
/// zyn! {
///     @for (field in fields.iter()) {
///         pub {{ field.ident }}: {{ field.ty }},
///     }
/// }
/// // output: pub x: f64, pub y: f64,
/// ```
///
/// Count form (no binding, repeats N times):
///
/// ```ignore
/// zyn! { @for (3) { x, } }
/// // output: x, x, x,
/// ```
///
/// # `@match`
///
/// ```ignore
/// zyn! {
///     @match (kind) {
///         Kind::Struct => { struct {{ name }} {} }
///         Kind::Enum   => { enum {{ name }} {} }
///         _            => {}
///     }
/// }
/// ```
///
/// # Element invocation
///
/// Call a `#[zyn::element]` component with named props:
///
/// ```ignore
/// zyn! {
///     @for (field in fields.iter()) {
///         @getter(name = field.ident.clone().unwrap(), ty = field.ty.clone())
///     }
/// }
/// ```
///
/// With a children block:
///
/// ```ignore
/// zyn! {
///     @wrapper(title = "hello") { inner content }
/// }
/// ```
#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::template::expand(input.into()).into()
}

/// Expands a zyn template with diagnostic output for debugging.
#[proc_macro]
pub fn debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::debug::expand(input.into()).into()
}

/// Defines a reusable template component that generates a struct implementing `Render`.
#[proc_macro_attribute]
pub fn element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::element::expand(args.into(), input.into()).into()
}

/// Defines a custom pipe transform that generates a struct implementing `Pipe`.
#[proc_macro_attribute]
pub fn pipe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::pipe::expand(args.into(), input.into()).into()
}

/// Defines a derive macro entry point that auto-parses `DeriveInput` into `Input`.
#[proc_macro_attribute]
pub fn derive(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::derive::expand(args.into(), input.into()).into()
}

/// Defines an attribute macro entry point that auto-parses the annotated item into `Input`.
#[proc_macro_attribute]
pub fn attribute(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::attribute::expand(args.into(), input.into()).into()
}

/// Derives the `Attribute` trait for typed attribute parsing from `#[attr(...)]` syntax.
#[proc_macro_derive(Attribute, attributes(zyn))]
pub fn derive_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute::expand(input.into()).into()
}
