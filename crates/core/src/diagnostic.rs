pub use proc_macro2_diagnostics::Diagnostic;
pub use proc_macro2_diagnostics::Level;
pub use proc_macro2_diagnostics::SpanDiagnosticExt;

/// A specialized [`Result`](std::result::Result) type for zyn diagnostics.
pub type Result<T> = std::result::Result<T, Diagnostics>;

/// An accumulator for compiler diagnostics (errors, warnings, notes, help messages).
///
/// Collects [`Diagnostic`] values during macro expansion and emits them as
/// compiler messages via [`emit`](Self::emit).
#[derive(Debug)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    /// Creates an empty diagnostics accumulator.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a diagnostics accumulator containing a single error.
    pub fn error(span: proc_macro2::Span, msg: impl Into<String>) -> Self {
        Self(vec![Diagnostic::spanned(span, Level::Error, msg.into())])
    }

    /// Pushes a single diagnostic.
    pub fn push(&mut self, diag: Diagnostic) {
        self.0.push(diag);
    }

    /// Appends all diagnostics from `other` into this accumulator.
    pub fn extend(&mut self, other: Diagnostics) {
        self.0.extend(other.0);
    }

    /// Returns `true` if no diagnostics have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of accumulated diagnostics.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if any accumulated diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        self.0.iter().any(|d| d.level() == Level::Error)
    }

    /// Returns the highest severity level among accumulated diagnostics.
    pub fn max_level(&self) -> Option<Level> {
        self.0
            .iter()
            .map(|d| d.level())
            .max_by_key(|l| Self::level_ord(*l))
    }

    /// Returns an iterator over the accumulated diagnostics.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.0.iter()
    }

    /// Consumes the accumulator and emits all diagnostics as compiler messages.
    pub fn emit(self) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();

        for diag in self.0 {
            tokens.extend(diag.emit_as_item_tokens());
        }

        tokens
    }

    fn level_ord(level: Level) -> u8 {
        match level {
            Level::Note => 0,
            Level::Help => 1,
            Level::Warning => 2,
            Level::Error => 3,
            _ => 4,
        }
    }
}

impl std::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, diag) in self.0.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            write!(f, "{:?}", diag)?;
        }

        Ok(())
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(diag: Diagnostic) -> Self {
        Self(vec![diag])
    }
}

impl From<syn::Error> for Diagnostics {
    fn from(error: syn::Error) -> Self {
        let diags = error
            .into_iter()
            .map(|e| Diagnostic::spanned(e.span(), Level::Error, e.to_string()))
            .collect();

        Self(diags)
    }
}

impl IntoIterator for Diagnostics {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Conversion trait for types that can be turned into [`Diagnostics`].
pub trait ToDiagnostics {
    fn to_diagnostics(self) -> Diagnostics;
}

impl ToDiagnostics for syn::Error {
    fn to_diagnostics(self) -> Diagnostics {
        Diagnostics::from(self)
    }
}

impl ToDiagnostics for Diagnostics {
    fn to_diagnostics(self) -> Diagnostics {
        self
    }
}
