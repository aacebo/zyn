use proc_macro2::Span;
use proc_macro2::TokenStream;

use crate::mark::Level;
use crate::mark::MultiSpan;

/// A compiler diagnostic (error, warning, note, or help message).
///
/// Immutable once built. Create instances via [`DiagnosticBuilder`] or the
/// free functions in [`crate::mark`].
#[derive(Debug, Clone, Default)]
pub struct Diagnostic {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>,
}

/// A specialized [`Result`](std::result::Result) type for zyn diagnostics.
pub type Result<T> = std::result::Result<T, Diagnostic>;

impl Diagnostic {
    /// Returns the joined span of this diagnostic, or `call_site` if no spans are attached.
    pub fn span(&self) -> Span {
        let mut value = self.spans.first().copied().unwrap_or_else(Span::call_site);

        for item in &self.spans {
            value = value.join(*item).unwrap_or_else(Span::call_site);
        }

        value
    }

    /// Returns the highest severity level among this diagnostic and its children.
    pub fn level(&self) -> Level {
        self.children
            .iter()
            .map(|d| d.level)
            .max()
            .unwrap_or(self.level)
            .max(self.level)
    }

    /// Returns an iterator over the direct children of this diagnostic.
    pub fn children(&self) -> impl Iterator<Item = &Diagnostic> {
        self.children.iter()
    }

    /// Alias for [`children`](Self::children).
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.children()
    }

    /// Returns `true` if the highest severity level is [`Level::Error`] or above.
    pub fn is_error(&self) -> bool {
        self.level() >= Level::Error
    }

    /// Returns `true` if no level and no children are set.
    pub fn is_empty(&self) -> bool {
        self.level == Level::None && self.children.is_empty()
    }

    /// Returns the number of direct children.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    // Emission

    #[cfg(feature = "diagnostics")]
    pub fn emit_as_item_tokens(self) -> TokenStream {
        let diag: proc_macro2_diagnostics::Diagnostic = self.into();
        diag.emit_as_item_tokens()
    }

    #[cfg(not(feature = "diagnostics"))]
    pub fn emit_as_item_tokens(self) -> TokenStream {
        self.emit_fallback()
    }

    #[cfg(feature = "diagnostics")]
    pub fn emit_as_expr_tokens(self) -> TokenStream {
        let diag: proc_macro2_diagnostics::Diagnostic = self.into();
        diag.emit_as_expr_tokens()
    }

    #[cfg(not(feature = "diagnostics"))]
    pub fn emit_as_expr_tokens(self) -> TokenStream {
        self.emit_fallback()
    }

    /// Emits all accumulated diagnostics as compiler messages.
    pub fn emit(self) -> TokenStream {
        let mut tokens = TokenStream::new();

        if self.level != Level::None {
            tokens.extend(self.clone().emit_as_item_tokens());
        }

        for child in self.children {
            tokens.extend(child.emit());
        }

        tokens
    }

    #[cfg(not(feature = "diagnostics"))]
    fn emit_fallback(self) -> TokenStream {
        let span = self.span();
        let msg = &self.message;
        let mut tokens = if self.level != Level::None && !msg.is_empty() {
            let prefix = match self.level {
                Level::Warning => "warning: ",
                Level::Note => "note: ",
                Level::Help => "help: ",
                _ => "",
            };
            let full_msg = format!("{prefix}{msg}");
            quote::quote_spanned! { span => compile_error!(#full_msg); }
        } else {
            TokenStream::new()
        };

        for child in self.children {
            tokens.extend(child.emit_fallback());
        }

        tokens
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.message.is_empty() {
            write!(f, "{}: {}", self.level, self.message)?;
        }

        for (i, child) in self.children.iter().enumerate() {
            if !self.message.is_empty() || i > 0 {
                writeln!(f)?;
            }

            write!(f, "{child}")?;
        }

        Ok(())
    }
}

impl From<syn::Error> for Diagnostic {
    fn from(error: syn::Error) -> Self {
        let children: Vec<_> = error
            .into_iter()
            .map(|e| Diagnostic {
                level: Level::Error,
                message: e.to_string(),
                spans: e.span().into_spans(),
                children: Vec::new(),
            })
            .collect();

        Self {
            children,
            ..Default::default()
        }
    }
}

#[cfg(feature = "diagnostics")]
impl From<Diagnostic> for proc_macro2_diagnostics::Diagnostic {
    fn from(value: Diagnostic) -> Self {
        let span = value.span();
        let mut diag = Self::spanned(span, value.level.into(), value.message);

        for child in value.children {
            diag = diag.spanned_child(child.span(), child.level.into(), child.message);
        }

        diag
    }
}

impl IntoIterator for Diagnostic {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diagnostic {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

// ── DiagnosticBuilder ──────────────────────────────────────────────────

/// Builder for constructing [`Diagnostic`] instances.
///
/// All methods are builder-pattern (consume and return `Self`).
/// Call [`build`](Self::build) to finalize into an immutable `Diagnostic`.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticBuilder {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>,
}

impl DiagnosticBuilder {
    /// Sets the severity level.
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Sets the diagnostic message.
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }

    /// Sets the source span(s). Accepts a [`Span`], `Vec<Span>`, or `&[Span]`.
    pub fn span(mut self, spans: impl MultiSpan) -> Self {
        self.spans = spans.into_spans();
        self
    }

    /// Adds a child diagnostic. Accepts a `DiagnosticBuilder` (built automatically)
    /// or a `Diagnostic` (via `Into<DiagnosticBuilder>`).
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, child: impl Into<DiagnosticBuilder>) -> Self {
        self.children.push(child.into().build());
        self
    }

    /// Finalizes the builder into an immutable [`Diagnostic`].
    pub fn build(self) -> Diagnostic {
        Diagnostic {
            level: self.level,
            message: self.message,
            spans: self.spans,
            children: self.children,
        }
    }
}

impl From<Diagnostic> for DiagnosticBuilder {
    fn from(d: Diagnostic) -> Self {
        Self {
            level: d.level,
            message: d.message,
            spans: d.spans,
            children: d.children,
        }
    }
}
