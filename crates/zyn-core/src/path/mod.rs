//! Dot-separated path type for navigating nested `syn` metadata.
//!
//! [`MetaPath`] parses strings like `"serde.rename"` or `"derive[0]"` into
//! a sequence of [`Segment`]s that can be used to drill into nested
//! `syn::Meta` structures.
//!
//! # Syntax
//!
//! - `.` separates key segments
//! - `[N]` is a positional index into list items
//! - The first segment is always a key
//!
//! # Examples
//!
//! ```ignore
//! use zyn::path::MetaPath;
//!
//! let path: MetaPath = "serde.rename".parse().unwrap();
//! assert_eq!(path.segments().len(), 2);
//!
//! let path: MetaPath = "derive[0]".parse().unwrap();
//! // → [Key("derive"), Index(0)]
//!
//! let path: MetaPath = "serde.container[1].value".parse().unwrap();
//! // → [Key("serde"), Key("container"), Index(1), Key("value")]
//! ```

mod error;
mod segment;

pub use error::*;
pub use segment::*;

/// A parsed dot-separated path for navigating nested `syn` metadata.
///
/// Construct via [`FromStr`](std::str::FromStr) or [`MetaPath::parse`].
///
/// # Examples
///
/// ```ignore
/// use zyn::path::MetaPath;
///
/// // Simple dotted path
/// let path: MetaPath = "serde.rename".parse().unwrap();
/// assert_eq!(path.len(), 2);
///
/// // With index access
/// let path: MetaPath = "derive[0]".parse().unwrap();
/// assert_eq!(path.len(), 2);
/// assert_eq!(path.to_string(), "derive[0]");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetaPath {
    segments: Vec<Segment>,
}

impl MetaPath {
    /// Parses a dot-separated path string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::path::MetaPath;
    ///
    /// let path = MetaPath::parse("serde.rename").unwrap();
    /// // → [Key("serde"), Key("rename")]
    ///
    /// let path = MetaPath::parse("a.b[2].c").unwrap();
    /// // → [Key("a"), Key("b"), Index(2), Key("c")]
    /// ```
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }

        let mut segments = Vec::new();
        let mut chars = s.chars().peekable();
        let mut buf = String::new();

        while let Some(&ch) = chars.peek() {
            match ch {
                '.' => {
                    chars.next();

                    if !buf.is_empty() {
                        segments.push(Segment::Key(std::mem::take(&mut buf)));
                    }
                }
                '[' => {
                    chars.next();

                    if !buf.is_empty() {
                        segments.push(Segment::Key(std::mem::take(&mut buf)));
                    }

                    let mut num = String::new();
                    let mut closed = false;

                    while let Some(&c) = chars.peek() {
                        if c == ']' {
                            chars.next();
                            closed = true;
                            break;
                        }

                        num.push(c);
                        chars.next();
                    }

                    if !closed {
                        return Err(ParseError::UnclosedBracket);
                    }

                    let index = num
                        .parse::<usize>()
                        .map_err(|_| ParseError::InvalidIndex(num))?;

                    segments.push(Segment::Index(index));
                }
                _ => {
                    buf.push(ch);
                    chars.next();
                }
            }
        }

        if !buf.is_empty() {
            segments.push(Segment::Key(buf));
        }

        Ok(Self { segments })
    }

    /// Returns the path segments.
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Returns the number of segments.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns `true` if the path has no segments.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the first segment, or `None` if the path is empty.
    pub fn first(&self) -> Option<&Segment> {
        self.segments.first()
    }

    /// Returns a new path with the first segment removed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::path::MetaPath;
    ///
    /// let path = MetaPath::parse("serde.rename.value").unwrap();
    /// let tail = path.tail();
    /// // tail → [Key("rename"), Key("value")]
    /// ```
    pub fn tail(&self) -> Self {
        Self {
            segments: self.segments.get(1..).unwrap_or_default().to_vec(),
        }
    }
}

impl std::str::FromStr for MetaPath {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Display for MetaPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;

        for seg in &self.segments {
            match seg {
                Segment::Key(k) => {
                    if !first {
                        write!(f, ".")?;
                    }

                    write!(f, "{}", k)?;
                }
                Segment::Index(i) => {
                    write!(f, "[{}]", i)?;
                }
            }

            first = false;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn single_key() {
            let path = MetaPath::parse("serde").unwrap();
            assert_eq!(path.segments(), &[Segment::Key("serde".into())]);
        }

        #[test]
        fn dotted_keys() {
            let path = MetaPath::parse("serde.rename").unwrap();
            assert_eq!(
                path.segments(),
                &[Segment::Key("serde".into()), Segment::Key("rename".into()),],
            );
        }

        #[test]
        fn index_after_key() {
            let path = MetaPath::parse("derive[0]").unwrap();
            assert_eq!(
                path.segments(),
                &[Segment::Key("derive".into()), Segment::Index(0)],
            );
        }

        #[test]
        fn mixed_path() {
            let path = MetaPath::parse("a.b[2].c").unwrap();
            assert_eq!(
                path.segments(),
                &[
                    Segment::Key("a".into()),
                    Segment::Key("b".into()),
                    Segment::Index(2),
                    Segment::Key("c".into()),
                ],
            );
        }

        #[test]
        fn empty_string_is_err() {
            assert_eq!(MetaPath::parse(""), Err(ParseError::Empty));
        }

        #[test]
        fn unclosed_bracket_is_err() {
            assert_eq!(MetaPath::parse("a[0"), Err(ParseError::UnclosedBracket));
        }

        #[test]
        fn invalid_index_is_err() {
            assert!(matches!(
                MetaPath::parse("a[abc]"),
                Err(ParseError::InvalidIndex(_)),
            ));
        }
    }

    mod display {
        use super::*;

        #[test]
        fn round_trip_dotted() {
            let path = MetaPath::parse("serde.rename").unwrap();
            assert_eq!(path.to_string(), "serde.rename");
        }

        #[test]
        fn round_trip_indexed() {
            let path = MetaPath::parse("derive[0]").unwrap();
            assert_eq!(path.to_string(), "derive[0]");
        }

        #[test]
        fn round_trip_mixed() {
            let path = MetaPath::parse("a.b[2].c").unwrap();
            assert_eq!(path.to_string(), "a.b[2].c");
        }
    }

    mod tail {
        use super::*;

        #[test]
        fn removes_first_segment() {
            let path = MetaPath::parse("a.b.c").unwrap();
            let tail = path.tail();
            assert_eq!(
                tail.segments(),
                &[Segment::Key("b".into()), Segment::Key("c".into())],
            );
        }

        #[test]
        fn single_segment_gives_empty() {
            let path = MetaPath::parse("a").unwrap();
            let tail = path.tail();
            assert!(tail.is_empty());
        }
    }
}
