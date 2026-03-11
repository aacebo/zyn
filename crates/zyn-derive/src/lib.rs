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
/// zyn! { @for (3) { x, }}
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

/// Defines a reusable template component generating a struct that implements `Render`.
///
/// Function parameters become either **props** (struct fields passed at call site)
/// or **extractors** (marked `#[zyn(input)]`, resolved automatically from `Input`).
///
/// Built-in extractor types: `Extract<T>`, `Attr<T>`, `Fields`, `Variants`, `Data<T>`.
/// A parameter named `children` receives the inner token stream from a children block.
///
/// # Examples
///
/// Simple element with props:
///
/// ```ignore
/// #[zyn::element]
/// fn greeting(name: syn::Ident) -> zyn::TokenStream {
///     zyn::zyn!(pub fn {{ name }}() {})
/// }
///
/// // Invoke inside a template
/// zyn::zyn!(@greeting(name = format_ident!("hello")))
/// // output: pub fn hello() {}
/// ```
///
/// Element with an extractor and a children block:
///
/// ```ignore
/// #[zyn::element]
/// fn wrapper(
///     #[zyn(input)] ident: zyn::Extract<syn::Ident>,
///     children: zyn::TokenStream,
/// ) -> zyn::TokenStream {
///     zyn::zyn!(impl {{ ident }} { {{ children }} })
/// }
///
/// zyn::zyn!(@wrapper { fn new() -> Self { Self } })
/// // output: impl MyStruct { fn new() -> Self { Self } }
/// ```
///
/// Optional custom name alias (defaults to function name):
///
/// ```ignore
/// #[zyn::element("my_alias")]
/// fn internal_name(label: syn::Ident) -> zyn::TokenStream { ... }
///
/// zyn::zyn!(@my_alias(label = format_ident!("x")))
/// ```
///
/// # Debugging
///
/// Add `debug` to inspect the generated code as a compiler `note` diagnostic.
/// Requires the `ZYN_DEBUG` environment variable to match the generated struct
/// name (supports `*` wildcards, e.g., `ZYN_DEBUG="*"`).
///
/// ```ignore
/// #[zyn::element(debug)]
/// #[zyn::element(debug = "pretty")]   // requires `pretty` feature
/// #[zyn::element("alias", debug)]
/// ```
///
/// ```bash
/// ZYN_DEBUG="Greeting" cargo build
/// ```
///
/// Without `ZYN_DEBUG`, the `debug` argument is inert — safe to leave in source.
///
/// The `pretty` format uses `prettyplease` for formatted output. Enable it with:
///
/// ```toml
/// zyn = { version = "0.3", features = ["pretty"] }
/// ```
#[proc_macro_attribute]
pub fn element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::element::expand(args.into(), input.into()).into()
}

/// Defines a custom pipe transform used inside `{{ expr | pipe }}` interpolations.
///
/// Transforms a single-argument function into a struct implementing `Pipe`.
/// The function receives a `String` (the stringified token) and returns any
/// [`quote::ToTokens`] value.
///
/// # Examples
///
/// Custom pipe (name defaults to the function name):
///
/// ```ignore
/// #[zyn::pipe]
/// fn shout(input: String) -> syn::Ident {
///     syn::Ident::new(&input.to_uppercase(), proc_macro2::Span::call_site())
/// }
///
/// let name = format_ident!("hello");
/// zyn::zyn!(static {{ name | shout }}: &str = "hi";)
/// // output: static HELLO: &str = "hi";
/// ```
///
/// With a custom name alias:
///
/// ```ignore
/// #[zyn::pipe("yell")]
/// fn make_loud(input: String) -> syn::Ident { ... }
///
/// zyn::zyn!(fn {{ name | yell }}() {})
/// ```
///
/// Chaining with built-in pipes:
///
/// ```ignore
/// zyn::zyn!(fn {{ name | snake | ident:"get_{}" }}() {})
/// // name = "HelloWorld" → fn get_hello_world() {}
/// ```
///
/// # Debugging
///
/// Add `debug` to inspect the generated code as a compiler `note` diagnostic.
/// Requires the `ZYN_DEBUG` environment variable to match the generated struct
/// name (supports `*` wildcards).
///
/// ```ignore
/// #[zyn::pipe(debug)]
/// #[zyn::pipe(debug = "pretty")]       // requires `pretty` feature
/// #[zyn::pipe("alias", debug)]
/// ```
///
/// Without `ZYN_DEBUG`, the `debug` argument is inert — safe to leave in source.
/// See the [debugging guide](https://aacebo.github.io/zyn/05-reference/debugging.html) for details.
#[proc_macro_attribute]
pub fn pipe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::pipe::expand(args.into(), input.into()).into()
}

/// Defines a derive macro entry point that auto-parses `DeriveInput` into typed inputs.
///
/// All parameters must be marked `#[zyn(input)]`; they are resolved from the
/// annotated type's `DeriveInput` via [`FromInput`](zyn::FromInput). The macro
/// name defaults to the function name in PascalCase, or can be set explicitly.
///
/// # Examples
///
/// ```ignore
/// // In your proc-macro crate (lib.rs):
/// #[zyn::derive]
/// fn my_derive(
///     #[zyn(input)] ident: zyn::Extract<syn::Ident>,
///     #[zyn(input)] fields: zyn::Fields,
/// ) -> zyn::TokenStream {
///     zyn::zyn! {
///         impl MyTrait for {{ ident }} {
///             fn field_count() -> usize { {{ fields.len() }} }
///         }
///     }
/// }
///
/// // Consumers annotate their types:
/// #[derive(MyDerive)]
/// struct Point { x: f64, y: f64 }
/// // output: impl MyTrait for Point { fn field_count() -> usize { 2 } }
/// ```
///
/// With an explicit macro name:
///
/// ```ignore
/// #[zyn::derive("DebugExtra")]
/// fn my_fn(#[zyn(input)] ident: zyn::Extract<syn::Ident>) -> zyn::TokenStream { ... }
/// // Registers as #[derive(DebugExtra)]
/// ```
///
/// # Debugging
///
/// Add `debug` to inspect the generated code as a compiler `note` diagnostic.
/// Requires the `ZYN_DEBUG` environment variable to match the derive name
/// (supports `*` wildcards).
///
/// ```ignore
/// #[zyn::derive("MyDerive", debug)]
/// #[zyn::derive("MyDerive", debug = "pretty")]               // requires `pretty` feature
/// #[zyn::derive("MyDerive", attributes(skip), debug)]
/// ```
///
/// Without `ZYN_DEBUG`, the `debug` argument is inert — safe to leave in source.
/// See the [debugging guide](https://aacebo.github.io/zyn/05-reference/debugging.html) for details.
#[proc_macro_attribute]
pub fn derive(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::derive::expand(args.into(), input.into()).into()
}

/// Defines an attribute macro entry point that auto-parses the annotated item into typed inputs.
///
/// Extractors marked `#[zyn(input)]` are resolved from the annotated item via
/// [`FromInput`](zyn::FromInput). An optional `args: zyn::Args` parameter receives
/// the raw attribute arguments. The macro name defaults to the function name.
///
/// # Examples
///
/// ```ignore
/// // In your proc-macro crate (lib.rs):
/// #[zyn::attribute]
/// fn log_call(
///     #[zyn(input)] item: syn::ItemFn,
///     args: zyn::Args,
/// ) -> zyn::TokenStream {
///     let prefix = args.get("prefix")
///         .and_then(|a| a.value::<String>().ok())
///         .unwrap_or_else(|| "CALL".into());
///     zyn::zyn! {
///         {{ item }}
///     }
/// }
///
/// // Applied to a function:
/// #[log_call(prefix = "DEBUG")]
/// fn my_fn() { ... }
/// ```
///
/// With an explicit macro name:
///
/// ```ignore
/// #[zyn::attribute("trace")]
/// fn trace_impl(#[zyn(input)] item: syn::ItemFn) -> zyn::TokenStream { ... }
/// // Registers as #[trace]
/// ```
///
/// # Debugging
///
/// Add `debug` to inspect the generated code as a compiler `note` diagnostic.
/// Requires the `ZYN_DEBUG` environment variable to match the function name
/// (supports `*` wildcards).
///
/// ```ignore
/// #[zyn::attribute(debug)]
/// #[zyn::attribute(debug = "pretty")]  // requires `pretty` feature
/// ```
///
/// Without `ZYN_DEBUG`, the `debug` argument is inert — safe to leave in source.
/// See the [debugging guide](https://aacebo.github.io/zyn/05-reference/debugging.html) for details.
#[proc_macro_attribute]
pub fn attribute(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::attribute::expand(args.into(), input.into()).into()
}

/// Derives typed attribute parsing from `#[attr(...)]` key-value syntax.
///
/// **Attribute mode** (`#[zyn("name")]`): also implements [`FromInput`](zyn::FromInput)
/// so the struct can be used as an `Attr<T>` extractor in `#[element]` or `#[derive]`.
///
/// **Argument mode** (no `#[zyn("name")]`): implements `FromArgs` and `FromArg` only,
/// suitable for nested argument structures.
///
/// # Field options
///
/// | Attribute | Effect |
/// |-----------|--------|
/// | `#[zyn(default)]` | Uses `Default::default()` when the field is absent |
/// | `#[zyn(default = "val")]` | Uses `Into::into("val")` as the default |
/// | `#[zyn(about = "...")]` | Doc string shown in `about()` output |
///
/// # Examples
///
/// Attribute mode (registers `FromInput` + `FromArgs`):
///
/// ```ignore
/// #[derive(zyn::Attribute)]
/// #[zyn("my_attr", about = "controls behaviour")]
/// struct MyAttr {
///     #[zyn(about = "the item name")]
///     rename: Option<String>,
///     #[zyn(default)]
///     skip: bool,
/// }
///
/// // Used as an extractor in an element:
/// #[zyn::element]
/// fn my_element(#[zyn(input)] attr: zyn::Attr<MyAttr>) -> zyn::TokenStream {
///     zyn::zyn!(@if (attr.skip) { /* nothing */ } @else { /* generate */ })
/// }
///
/// // Applied to a type:
/// #[my_attr(rename = "other_name")]
/// struct Foo;
/// ```
///
/// Argument mode (no `#[zyn("name")]`):
///
/// ```ignore
/// #[derive(zyn::Attribute)]
/// struct Config { level: i64, tag: String }
///
/// let args: zyn::Args = zyn::parse!("level = 3, tag = \"v1\"").unwrap();
/// let cfg = Config::from_args(&args).unwrap();
/// ```
///
/// # Diagnostics
///
/// Errors are accumulated and returned together. Unknown arguments automatically
/// produce "did you mean?" suggestions when a close match exists:
///
/// ```ignore
/// #[derive(zyn::Attribute)]
/// #[zyn("config")]
/// struct Config {
///     enabled: bool,
///     format: String,
/// }
///
/// // #[config(enabed, fromat = "json")]
/// //
/// // error: unknown argument `enabed`
/// //   --> src/lib.rs:10:10
/// //    |
/// // 10 | #[config(enabed, fromat = "json")]
/// //    |          ^^^^^^
/// //    |
/// //    = help: did you mean `enabled`?
/// //
/// // error: unknown argument `fromat`
/// //   --> src/lib.rs:10:18
/// //    |
/// // 10 | #[config(enabed, fromat = "json")]
/// //    |                  ^^^^^^
/// //    |
/// //    = help: did you mean `format`?
/// ```
///
/// Enum derive for variant dispatch:
///
/// ```ignore
/// #[derive(zyn::Attribute)]
/// enum Mode {
///     Fast,
///     Slow,
///     Custom { speed: i64 },
/// }
///
/// let arg: zyn::Arg = zyn::parse!("custom(speed = 10)").unwrap();
/// let mode = Mode::from_arg(&arg).unwrap();
/// // mode == Mode::Custom { speed: 10 }
/// ```
#[proc_macro_derive(Attribute, attributes(zyn))]
pub fn derive_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute::expand(input.into()).into()
}
