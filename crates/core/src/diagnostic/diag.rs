use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote_spanned;

use super::level::Level;
use super::result::Child;
use super::result::ChildKind;

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub(super) level: Level,
    pub(super) span: Span,
    pub(super) message: String,
    pub(super) children: Vec<Child>,
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: Level::Error,
            span,
            message: message.into(),
            children: Vec::new(),
        }
    }

    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: Level::Warning,
            span,
            message: message.into(),
            children: Vec::new(),
        }
    }

    pub fn note(mut self, message: impl Into<String>) -> Self {
        self.children.push(Child {
            kind: ChildKind::Note,
            message: message.into(),
        });
        self
    }

    pub fn help(mut self, message: impl Into<String>) -> Self {
        self.children.push(Child {
            kind: ChildKind::Help,
            message: message.into(),
        });
        self
    }

    fn full_message(&self) -> String {
        let mut msg = self.message.clone();

        for child in &self.children {
            let prefix = match child.kind {
                ChildKind::Note => "note",
                ChildKind::Help => "help",
            };

            msg.push('\n');
            msg.push_str(prefix);
            msg.push_str(": ");
            msg.push_str(&child.message);
        }

        msg
    }
}

impl quote::ToTokens for Diagnostic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let full_msg = self.full_message();
        let span = self.span;

        let ts = match self.level {
            Level::Error => {
                quote_spanned! { span => ::core::compile_error!(#full_msg); }
            }
            Level::Warning => {
                quote_spanned! { span =>
                    {
                        #[allow(dead_code)]
                        #[deprecated(note = #full_msg)]
                        fn _zyn_warning() {}
                        #[allow(deprecated)]
                        _zyn_warning();
                    }
                }
            }
        };

        ts.to_tokens(tokens);
    }
}

impl From<syn::Error> for Diagnostic {
    fn from(e: syn::Error) -> Self {
        Self::error(e.span(), e.to_string())
    }
}

impl From<Diagnostic> for syn::Error {
    fn from(d: Diagnostic) -> Self {
        syn::Error::new(d.span, d.full_message())
    }
}
