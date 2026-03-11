//! Diagnostic severity levels.

/// Diagnostic severity level, ordered from least to most severe.
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// No diagnostic (default). Not emitted.
    #[default]
    None,
    /// Informational note.
    Note,
    /// Helpful suggestion.
    Help,
    /// Compiler warning.
    Warning,
    /// Hard compile error.
    Error,
}

impl Level {
    /// Returns the numeric value of this level.
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Returns a lowercase string representation (`"error"`, `"warning"`, etc.).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "diagnostics")]
impl From<proc_macro2_diagnostics::Level> for Level {
    fn from(value: proc_macro2_diagnostics::Level) -> Self {
        match value {
            proc_macro2_diagnostics::Level::Error => Self::Error,
            proc_macro2_diagnostics::Level::Warning => Self::Warning,
            proc_macro2_diagnostics::Level::Note => Self::Note,
            proc_macro2_diagnostics::Level::Help => Self::Help,
            _ => Self::None,
        }
    }
}

#[cfg(feature = "diagnostics")]
impl From<Level> for proc_macro2_diagnostics::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Error,
            Level::Warning => Self::Warning,
            Level::Note => Self::Note,
            Level::Help => Self::Help,
            Level::None => panic!("unsupported diagnostic level"),
        }
    }
}
