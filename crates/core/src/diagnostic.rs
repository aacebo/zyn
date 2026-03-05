pub use proc_macro2_diagnostics::Diagnostic;
pub use proc_macro2_diagnostics::Level;
pub use proc_macro2_diagnostics::SpanDiagnosticExt;

pub type Result<T> = std::result::Result<T, Diagnostics>;

#[derive(Debug)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn error(span: proc_macro2::Span, msg: impl Into<String>) -> Self {
        Self(vec![Diagnostic::spanned(span, Level::Error, msg.into())])
    }

    pub fn push(&mut self, diag: Diagnostic) {
        self.0.push(diag);
    }

    pub fn extend(&mut self, other: Diagnostics) {
        self.0.extend(other.0);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn has_errors(&self) -> bool {
        self.0.iter().any(|d| d.level() == Level::Error)
    }

    pub fn max_level(&self) -> Option<Level> {
        self.0
            .iter()
            .map(|d| d.level())
            .max_by_key(|l| level_ord(*l))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.0.iter()
    }

    pub fn emit(self) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();

        for diag in self.0 {
            tokens.extend(diag.emit_as_item_tokens());
        }

        tokens
    }
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
