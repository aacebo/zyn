//! Generates the `error!`, `warn!`, `note!`, `help!`, and `bail!` diagnostic macros available inside `#[zyn::element]`, `#[zyn::derive]`, and `#[zyn::attribute]` bodies.

use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;

pub fn macros() -> TokenStream {
    quote! {
        /// Pushes an error diagnostic. Accepts `format!`-style arguments.
        ///
        /// ```rust,ignore
        /// #[zyn::element]
        /// fn validated(#[zyn(input)] ident: syn::Ident) -> zyn::TokenStream {
        ///     if ident == "forbidden" {
        ///         error!("reserved identifier"; span = ident.span());
        ///     }
        ///     bail!();
        ///     zyn::zyn! { fn {{ ident }}() {} }
        /// }
        /// ```
        ///
        /// ```text
        /// error: reserved identifier
        ///  --> src/lib.rs:8:24
        ///   |
        /// 8 |     @validated(name = forbidden)
        ///   |                       ^^^^^^^^^
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// error!("expected struct, found enum"; span = ident.span());
        /// ```
        #[allow(unused)]
        macro_rules! error {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ))
            };
        }

        /// Pushes a warning diagnostic. Accepts `format!`-style arguments.
        ///
        /// ```rust,ignore
        /// #[zyn::element]
        /// fn legacy(#[zyn(input)] ident: syn::Ident) -> zyn::TokenStream {
        ///     warn!("this element is deprecated, use `new_api` instead");
        ///     zyn::zyn! { fn {{ ident }}() {} }
        /// }
        /// ```
        ///
        /// ```text
        /// warning: this element is deprecated, use `new_api` instead
        ///  --> src/lib.rs:5:5
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// warn!("field `{}` is unused", name; span = field.span());
        /// ```
        #[allow(unused)]
        macro_rules! warn {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Warning, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Warning, format!($fmt $(, $arg)*)
                ))
            };
        }

        /// Pushes a note diagnostic. Accepts `format!`-style arguments.
        ///
        /// ```rust,ignore
        /// #[zyn::element]
        /// fn validated(name: syn::Ident) -> zyn::TokenStream {
        ///     if name == "forbidden" {
        ///         error!("reserved identifier"; span = name.span());
        ///         note!("this name is reserved by the compiler");
        ///     }
        ///     bail!();
        ///     zyn::zyn! { fn {{ name }}() {} }
        /// }
        /// ```
        ///
        /// ```text
        /// error: reserved identifier
        ///  --> src/lib.rs:8:24
        ///   |
        /// 8 |     @validated(name = forbidden)
        ///   |                       ^^^^^^^^^
        ///   = note: this name is reserved by the compiler
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// note!("derived from `{}`", ident; span = ident.span());
        /// ```
        #[allow(unused)]
        macro_rules! note {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Note, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Note, format!($fmt $(, $arg)*)
                ))
            };
        }

        /// Pushes a help diagnostic. Accepts `format!`-style arguments.
        ///
        /// ```rust,ignore
        /// #[zyn::element]
        /// fn validated(name: syn::Ident) -> zyn::TokenStream {
        ///     if name == "bad" {
        ///         error!("name is bad");
        ///         help!("use a different name");
        ///     }
        ///     bail!();
        ///     zyn::zyn! { fn {{ name }}() {} }
        /// }
        /// ```
        ///
        /// ```text
        /// error: name is bad
        ///  --> src/lib.rs:4:10
        ///   = help: use a different name
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// help!("consider adding `#[zyn(skip)]`"; span = field.span());
        /// ```
        #[allow(unused)]
        macro_rules! help {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Help, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Help, format!($fmt $(, $arg)*)
                ))
            };
        }

        /// Returns early with accumulated diagnostics.
        ///
        /// ```rust,ignore
        /// #[zyn::element]
        /// fn format_error(name: syn::Ident) -> zyn::TokenStream {
        ///     if name == "foo" {
        ///         bail!("field `{}` is invalid", name);
        ///     }
        ///     zyn::zyn! { fn {{ name }}() {} }
        /// }
        /// ```
        ///
        /// ```text
        /// error: field `foo` is invalid
        ///  --> src/lib.rs:4:10
        /// ```
        ///
        /// With no arguments, returns only if errors have been pushed:
        /// ```rust,ignore
        /// bail!(); // no-op if no errors
        /// ```
        ///
        /// With a span:
        /// ```rust,ignore
        /// bail!("unsupported type `{}`", name; span = name.span());
        /// ```
        #[allow(unused)]
        macro_rules! bail {
            () => {
                if diagnostics.has_errors() {
                    return diagnostics.emit();
                }
            };
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {{
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ));

                return diagnostics.emit();
            }};
            ($fmt:literal $(, $arg:expr)* $(,)?) => {{
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ));

                return diagnostics.emit();
            }};
        }
    }
}
