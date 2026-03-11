//! Extension trait for `syn::Attribute` parsing and metadata querying.
//!
//! [`AttrExt`] adds name checking, argument parsing, and dot-path metadata
//! navigation to `syn::Attribute`.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::AttrExt;
//!
//! // #[serde(rename = "id", skip)]
//! if attr.is("serde") {
//!     let args = attr.args().unwrap();
//!     let rename = attr.get("rename");   // → Some(NameValue meta)
//!     let has_skip = attr.exists("skip"); // → true
//! }
//! ```

use syn::Attribute;
use syn::Meta;

use crate::meta::Args;
use crate::path::{MetaPath, Segment};

use super::MetaExt;

/// Extension methods for a single `syn::Attribute`.
///
/// Provides name checking, argument parsing, and dot-path metadata querying.
/// The `get`, `exists`, and `merge` methods use [`MetaPath`] syntax to
/// navigate into nested meta structures.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::AttrExt;
///
/// // #[serde(rename = "id", skip)]
/// assert!(attr.is("serde"));
/// assert!(attr.exists("rename"));
///
/// let rename = attr.get("rename"); // → Some(NameValue meta)
/// let all = attr.merge("rename");  // → Vec of matching Meta entries
/// ```
pub trait AttrExt {
    /// Returns `true` if the attribute's path matches the given name.
    fn is(&self, name: &str) -> bool;
    /// Parses the attribute's arguments into an [`Args`] list.
    fn args(&self) -> syn::Result<Args>;
    /// Navigates nested metadata using a dot-separated path with optional index access.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::ext::AttrExt;
    ///
    /// // #[serde(rename = "id")]
    /// let meta = attr.get("rename"); // → Some(NameValue meta)
    ///
    /// // #[derive(Clone, Debug)]
    /// let first = attr.get("[0]"); // → Some(Path meta for Clone)
    /// ```
    fn get(&self, path: &str) -> Option<Meta>;
    /// Returns `true` if metadata at the given dot-path exists.
    fn exists(&self, path: &str) -> bool;
    /// Collects all metadata entries at the top level matching the first path segment.
    fn merge(&self, path: &str) -> Vec<Meta>;
}

impl AttrExt for Attribute {
    fn is(&self, name: &str) -> bool {
        self.path().is_ident(name)
    }

    fn args(&self) -> syn::Result<Args> {
        self.parse_args::<Args>()
    }

    fn get(&self, path: &str) -> Option<Meta> {
        if path.is_empty() {
            return Some(self.meta.clone());
        }

        self.meta.clone().get(path)
    }

    fn exists(&self, path: &str) -> bool {
        self.get(path).is_some()
    }

    fn merge(&self, path: &str) -> Vec<Meta> {
        let parsed = match MetaPath::parse(path) {
            Ok(p) => p,
            Err(_) => return Vec::new(),
        };
        let seg = match parsed.first() {
            Some(seg) => seg,
            None => return Vec::new(),
        };

        let list = match &self.meta {
            Meta::List(list) => list,
            _ => return Vec::new(),
        };

        let nested: Vec<Meta> = list
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
            .unwrap_or_default()
            .into_iter()
            .collect();

        let tail = parsed.tail();

        nested
            .into_iter()
            .filter(|m| match seg {
                Segment::Key(name) => m.path().is_ident(name),
                Segment::Index(_) => false,
            })
            .filter_map(|m| {
                if tail.is_empty() {
                    Some(m)
                } else {
                    m.get(&tail.to_string())
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attr_from(attr_str: &str) -> Attribute {
        let item: syn::ItemStruct = syn::parse_str(&format!("{} struct Foo;", attr_str)).unwrap();
        item.attrs.into_iter().next().unwrap()
    }

    mod is {
        use super::*;

        #[test]
        fn matching_name() {
            let attr = attr_from("#[serde(skip)]");
            assert!(attr.is("serde"));
        }

        #[test]
        fn non_matching_name() {
            let attr = attr_from("#[serde(skip)]");
            assert!(!attr.is("derive"));
        }
    }

    mod args {
        use super::*;

        #[test]
        fn parses_arguments() {
            let attr = attr_from("#[my_attr(skip, rename = \"foo\")]");
            let args = attr.args().unwrap();
            assert!(args.has("skip"));
            assert!(args.has("rename"));
        }
    }

    mod get {
        use super::*;

        #[test]
        fn find_nested_key() {
            let attr = attr_from("#[serde(rename = \"id\", skip)]");
            let found = attr.get("skip").unwrap();
            assert!(found.path().is_ident("skip"));
        }

        #[test]
        fn find_by_index() {
            let attr = attr_from("#[derive(Clone, Debug)]");
            let found = attr.get("[0]").unwrap();
            assert!(found.path().is_ident("Clone"));
        }

        #[test]
        fn missing_key() {
            let attr = attr_from("#[serde(skip)]");
            assert!(attr.get("rename").is_none());
        }

        #[test]
        fn empty_path_returns_meta() {
            let attr = attr_from("#[test]");
            assert!(attr.get("").is_some());
        }
    }

    mod exists {
        use super::*;

        #[test]
        fn existing_key() {
            let attr = attr_from("#[serde(skip)]");
            assert!(attr.exists("skip"));
        }

        #[test]
        fn missing_key() {
            let attr = attr_from("#[serde(skip)]");
            assert!(!attr.exists("rename"));
        }
    }

    mod merge {
        use super::*;

        #[test]
        fn collects_matching_entries() {
            let attr = attr_from("#[cfg(feature = \"a\", feature = \"b\")]");
            let items = attr.merge("feature");
            assert_eq!(items.len(), 2);
        }

        #[test]
        fn no_matches() {
            let attr = attr_from("#[serde(skip)]");
            let items = attr.merge("rename");
            assert!(items.is_empty());
        }

        #[test]
        fn on_path_attr() {
            let attr = attr_from("#[test]");
            let items = attr.merge("anything");
            assert!(items.is_empty());
        }
    }
}
