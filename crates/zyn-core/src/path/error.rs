/// An error that occurs when parsing a [`MetaPath`](super::MetaPath).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The path string is empty.
    Empty,
    /// An index bracket was not closed (missing `]`).
    UnclosedBracket,
    /// The contents of `[...]` could not be parsed as a number.
    InvalidIndex(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty path"),
            Self::UnclosedBracket => write!(f, "unclosed bracket"),
            Self::InvalidIndex(s) => write!(f, "invalid index: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}
