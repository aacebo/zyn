//! Emission scope for diagnostics.

/// Controls whether a diagnostic is emitted as an item or an expression.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    /// Emit as a top-level item (e.g., inside a module or impl block).
    Item,
    /// Emit as an expression (e.g., inside a function body).
    Expr,
}

impl Scope {
    /// Returns a lowercase string representation (`"item"` or `"expr"`).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Item => "item",
            Self::Expr => "expr",
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
