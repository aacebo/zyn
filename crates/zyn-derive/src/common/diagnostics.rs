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
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// error!("expected struct, found enum"; span = ident.span());
        /// ```
        #[allow(unused)]
        macro_rules! error {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::error(format!($fmt $(, $arg)*)).span($span)
                )
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::error(format!($fmt $(, $arg)*)).span(input.span())
                )
            };
        }

        /// Pushes a warning diagnostic. Accepts `format!`-style arguments.
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// warn!("field `{}` is unused", name; span = field.span());
        /// ```
        #[allow(unused)]
        macro_rules! warn {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::warning(format!($fmt $(, $arg)*)).span($span)
                )
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::warning(format!($fmt $(, $arg)*)).span(input.span())
                )
            };
        }

        /// Pushes a note diagnostic. Accepts `format!`-style arguments.
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// note!("derived from `{}`", ident; span = ident.span());
        /// ```
        #[allow(unused)]
        macro_rules! note {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::note(format!($fmt $(, $arg)*)).span($span)
                )
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::note(format!($fmt $(, $arg)*)).span(input.span())
                )
            };
        }

        /// Pushes a help diagnostic. Accepts `format!`-style arguments.
        ///
        /// Attach a span with `; span = expr`:
        /// ```rust,ignore
        /// help!("consider adding `#[zyn(skip)]`"; span = field.span());
        /// ```
        #[allow(unused)]
        macro_rules! help {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::help(format!($fmt $(, $arg)*)).span($span)
                )
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics = diagnostics.add(
                    ::zyn::mark::help(format!($fmt $(, $arg)*)).span(input.span())
                )
            };
        }

        /// Returns early with accumulated diagnostics.
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
            () => {{
                let __built = diagnostics.build();
                if __built.is_error() {
                    return ::zyn::Output::new()
                        .diagnostic(::zyn::mark::Builder::from(__built))
                        .build();
                }
                diagnostics = ::zyn::mark::Builder::from(__built);
            }};
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {{
                diagnostics = diagnostics.add(
                    ::zyn::mark::error(format!($fmt $(, $arg)*)).span($span)
                );

                return ::zyn::Output::new()
                    .diagnostic(diagnostics)
                    .build();
            }};
            ($fmt:literal $(, $arg:expr)* $(,)?) => {{
                diagnostics = diagnostics.add(
                    ::zyn::mark::error(format!($fmt $(, $arg)*)).span(input.span())
                );

                return ::zyn::Output::new()
                    .diagnostic(diagnostics)
                    .build();
            }};
        }
    }
}
