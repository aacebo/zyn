//! Generates the `error!`, `warn!`, `note!`, `help!`, and `bail!` diagnostic macros available inside `#[zyn::element]`, `#[zyn::derive]`, and `#[zyn::attribute]` bodies.

use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;

pub fn macros() -> TokenStream {
    quote! {
        /// Pushes an error diagnostic. Accepts `format!`-style arguments.
        ///
        /// ```bash
        /// error: expected struct, found enum
        ///  --> src/lib.rs:4:1
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// error!("expected struct, found enum"; span = ident.span());
        /// ```
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
        /// ```bash
        /// warning: field `name` is unused
        ///  --> src/lib.rs:8:5
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// warn!("field `{}` is unused", name; span = field.span());
        /// ```
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
        /// ```bash
        /// note: derived from `MyStruct`
        ///  --> src/lib.rs:2:1
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// note!("derived from `{}`", ident; span = ident.span());
        /// ```
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
        /// ```bash
        /// help: consider adding `#[zyn(skip)]`
        ///  --> src/lib.rs:8:5
        /// ```
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// help!("consider adding `#[zyn(skip)]`"; span = field.span());
        /// ```
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
        /// With no arguments, returns only if errors have been pushed:
        /// ```rust,ignore
        /// bail!(); // no-op if no errors
        /// ```
        ///
        /// With a message, pushes an error and returns immediately:
        /// ```rust,ignore
        /// bail!("unsupported type");
        /// bail!("unsupported type `{}`", name; span = name.span());
        /// ```
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
