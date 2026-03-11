use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::mark::Builder;
use crate::mark::Diagnostic;

/// The result of rendering an element or expanding a template.
///
/// Holds both the generated token output and any accumulated diagnostics.
/// Implements [`ToTokens`] by emitting the tokens followed by any diagnostic
/// compiler messages, preserving the same behavior as the previous `TokenStream`
/// return type.
#[derive(Debug, Clone, Default)]
pub struct Output {
    tokens: TokenStream,
    diagnostic: Diagnostic,
}

impl Output {
    /// Creates a new [`OutputBuilder`].
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> OutputBuilder {
        OutputBuilder::default()
    }

    /// Returns the generated token output.
    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }

    /// Returns the accumulated diagnostic.
    pub fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    /// Consumes the output and returns its tokens and diagnostic.
    pub fn into_parts(self) -> (TokenStream, Diagnostic) {
        (self.tokens, self.diagnostic)
    }

    /// Returns `true` if the diagnostic contains an error.
    pub fn is_error(&self) -> bool {
        self.diagnostic.is_error()
    }
}

impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);
        self.diagnostic.clone().emit().to_tokens(tokens);
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}

impl From<Output> for TokenStream {
    fn from(o: Output) -> Self {
        o.to_token_stream()
    }
}

impl From<Output> for Diagnostic {
    fn from(o: Output) -> Self {
        o.diagnostic
    }
}

impl From<TokenStream> for Output {
    fn from(tokens: TokenStream) -> Self {
        Self {
            tokens,
            diagnostic: Diagnostic::default(),
        }
    }
}

impl From<Diagnostic> for Output {
    fn from(diagnostic: Diagnostic) -> Self {
        Self {
            tokens: TokenStream::new(),
            diagnostic,
        }
    }
}

/// Builder for constructing [`Output`] instances.
///
/// All methods are builder-pattern (consume and return `Self`).
/// Call [`build`](Self::build) to finalize into an immutable `Output`.
#[derive(Debug, Clone, Default)]
pub struct OutputBuilder {
    tokens: TokenStream,
    diagnostic: Builder,
}

impl OutputBuilder {
    /// Sets the generated token output.
    pub fn tokens(mut self, tokens: impl Into<TokenStream>) -> Self {
        self.tokens = tokens.into();
        self
    }

    /// Appends tokens to the output.
    pub fn extend(mut self, tokens: impl ToTokens) -> Self {
        tokens.to_tokens(&mut self.tokens);
        self
    }

    /// Sets the diagnostic builder.
    pub fn diagnostic(mut self, diagnostic: Builder) -> Self {
        self.diagnostic = diagnostic;
        self
    }

    /// Finalizes the builder into an immutable [`Output`].
    pub fn build(self) -> Output {
        Output {
            tokens: self.tokens,
            diagnostic: self.diagnostic.build(),
        }
    }
}

impl From<Output> for OutputBuilder {
    fn from(o: Output) -> Self {
        Self {
            tokens: o.tokens,
            diagnostic: Builder::from(o.diagnostic),
        }
    }
}
