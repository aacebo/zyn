pub mod ast;
pub mod case;
pub mod ident;
pub mod input;
pub mod pipes;

pub use input::*;
pub use pipes::*;

/// Internal trait for expanding AST nodes into generated code.
///
/// Each AST node implements this to produce a `TokenStream` that builds
/// the output incrementally using unique identifier variables.
pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

/// Trait for renderable elements invoked via `@element_name(props)` in templates.
///
/// Implement this on a struct to make it usable as a template element.
/// The struct fields become the element's props, and `render()` produces
/// the output token stream.
///
/// # Example
///
/// ```ignore
/// struct FieldDecl {
///     vis: syn::Visibility,
///     name: syn::Ident,
///     ty: syn::Type,
/// }
///
/// impl zyn::Render for FieldDecl {
///     fn render(&self) -> syn::Result<proc_macro2::TokenStream> {
///         let vis = &self.vis;
///         let name = &self.name;
///         let ty = &self.ty;
///         Ok(zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, })
///     }
/// }
/// ```
///
/// Or use the `#[zyn::element]` attribute macro to generate this automatically.
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

/// Trait for pipe transforms used via `{{ expr | pipe_name }}` in templates.
///
/// Pipes transform interpolated values before they are emitted as tokens.
/// The `Input` type is what the pipe receives (typically `String` from `.to_string()`),
/// and `Output` is what gets emitted to the token stream.
///
/// # Example
///
/// ```ignore
/// struct Prefix;
///
/// impl zyn::Pipe for Prefix {
///     type Input = String;
///     type Output = proc_macro2::Ident;
///
///     fn pipe(&self, input: String) -> proc_macro2::Ident {
///         proc_macro2::Ident::new(
///             &format!("pfx_{}", input),
///             proc_macro2::Span::call_site(),
///         )
///     }
/// }
/// ```
///
/// Or use the `#[zyn::pipe]` attribute macro to generate this automatically.
pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
