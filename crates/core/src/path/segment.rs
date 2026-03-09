/// A single segment in a [`MetaPath`](super::MetaPath).
///
/// # Examples
///
/// ```ignore
/// use zyn::path::Segment;
///
/// let key: Segment = "serde".into();
/// let index: Segment = 0usize.into();
///
/// assert!(key.is_key());
/// assert!(index.is_index());
/// assert_eq!(key.to_string(), "serde");
/// assert_eq!(index.to_string(), "0");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    /// A named key segment (e.g., `serde` in `"serde.rename"`).
    Key(String),
    /// A positional index segment (e.g., `0` in `"derive[0]"`).
    Index(usize),
}

impl Segment {
    /// Returns `true` if this is a key segment.
    pub fn is_key(&self) -> bool {
        matches!(self, Self::Key(_))
    }

    /// Returns `true` if this is an index segment.
    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    /// Returns the key string if this is a key segment.
    pub fn as_key(&self) -> Option<&str> {
        match self {
            Self::Key(k) => Some(k),
            _ => None,
        }
    }

    /// Returns the index value if this is an index segment.
    pub fn as_index(&self) -> Option<usize> {
        match self {
            Self::Index(i) => Some(*i),
            _ => None,
        }
    }
}

impl From<String> for Segment {
    fn from(key: String) -> Self {
        Self::Key(key)
    }
}

impl From<&str> for Segment {
    fn from(key: &str) -> Self {
        Self::Key(key.to_owned())
    }
}

impl From<usize> for Segment {
    fn from(index: usize) -> Self {
        Self::Index(index)
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(k) => write!(f, "{}", k),
            Self::Index(i) => write!(f, "{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod predicates {
        use super::*;

        #[test]
        fn key_is_key() {
            let seg: Segment = "foo".into();
            assert!(seg.is_key());
            assert!(!seg.is_index());
        }

        #[test]
        fn index_is_index() {
            let seg: Segment = 0usize.into();
            assert!(seg.is_index());
            assert!(!seg.is_key());
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn as_key_some() {
            let seg: Segment = "foo".into();
            assert_eq!(seg.as_key(), Some("foo"));
        }

        #[test]
        fn as_key_none() {
            let seg: Segment = 0usize.into();
            assert!(seg.as_key().is_none());
        }

        #[test]
        fn as_index_some() {
            let seg: Segment = 3usize.into();
            assert_eq!(seg.as_index(), Some(3));
        }

        #[test]
        fn as_index_none() {
            let seg: Segment = "foo".into();
            assert!(seg.as_index().is_none());
        }
    }

    mod display {
        use super::*;

        #[test]
        fn key_display() {
            let seg: Segment = "serde".into();
            assert_eq!(seg.to_string(), "serde");
        }

        #[test]
        fn index_display() {
            let seg: Segment = 42usize.into();
            assert_eq!(seg.to_string(), "42");
        }
    }

    mod from {
        use super::*;

        #[test]
        fn from_str() {
            let seg: Segment = "hello".into();
            assert_eq!(seg, Segment::Key("hello".into()));
        }

        #[test]
        fn from_string() {
            let seg: Segment = String::from("hello").into();
            assert_eq!(seg, Segment::Key("hello".into()));
        }

        #[test]
        fn from_usize() {
            let seg: Segment = 5usize.into();
            assert_eq!(seg, Segment::Index(5));
        }
    }
}
