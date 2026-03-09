//! Extension trait for `syn::Meta` inspection and nested navigation.
//!
//! [`MetaExt`] adds variant predicates, conversions, name checking, and
//! dot-path querying to `syn::Meta`.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::MetaExt;
//!
//! // #[serde(rename = "foo", skip)]
//! if meta.is_list() {
//!     let rename = meta.get("rename"); // → Some(NameValue meta)
//!     let skip = meta.get("skip");     // → Some(Path meta)
//! }
//! ```

use syn::Meta;

use crate::path::{MetaPath, Segment};

/// Extension methods for a single `syn::Meta`.
///
/// Provides variant predicates (`is_path`, `is_list`, `is_name_value`),
/// conversions (`as_path`, `as_list`, `as_name_value`), name checking,
/// and dot-path querying into nested meta lists.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::MetaExt;
///
/// // Given #[serde(rename = "id", skip)]
/// assert!(meta.is("serde"));
/// assert!(meta.is_list());
///
/// let rename = meta.get("rename"); // → Some(NameValue meta)
/// let skip = meta.get("skip");     // → Some(Path meta)
/// ```
pub trait MetaExt {
    /// Returns `true` if this is a `Meta::Path` (bare path like `#[foo]`).
    fn is_path(&self) -> bool;
    /// Returns `true` if this is a `Meta::List` (e.g., `#[foo(...)]`).
    fn is_list(&self) -> bool;
    /// Returns `true` if this is a `Meta::NameValue` (e.g., `#[foo = expr]`).
    fn is_name_value(&self) -> bool;
    /// Returns the inner `syn::Path` if this is a `Meta::Path`.
    fn as_path(&self) -> Option<&syn::Path>;
    /// Returns the inner `syn::MetaList` if this is a `Meta::List`.
    fn as_list(&self) -> Option<&syn::MetaList>;
    /// Returns the inner `syn::MetaNameValue` if this is a `Meta::NameValue`.
    fn as_name_value(&self) -> Option<&syn::MetaNameValue>;
    /// Returns `true` if the meta's path matches the given name.
    fn is(&self, name: &str) -> bool;
    /// Navigates nested meta using a dot-separated path with optional index access.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::ext::MetaExt;
    ///
    /// // Given meta for #[serde(rename = "id", skip)]
    /// let rename = meta.get("rename"); // → Some(NameValue)
    ///
    /// // Given meta for #[derive(Clone, Debug)]
    /// let first = meta.get("[0]"); // → Some(Path for Clone)
    /// ```
    fn get(&self, path: &str) -> Option<Meta>;
    /// Parses the contents of a `Meta::List` as a `Vec<Meta>`.
    /// Returns `None` if this is not a list variant.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::ext::MetaExt;
    ///
    /// // Given meta for #[derive(Clone, Debug)]
    /// let items = meta.nested().unwrap();
    /// // items → [Meta::Path(Clone), Meta::Path(Debug)]
    /// ```
    fn nested(&self) -> Option<Vec<Meta>>;
}

impl MetaExt for Meta {
    fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    fn is_name_value(&self) -> bool {
        matches!(self, Self::NameValue(_))
    }

    fn as_path(&self) -> Option<&syn::Path> {
        match self {
            Self::Path(p) => Some(p),
            _ => None,
        }
    }

    fn as_list(&self) -> Option<&syn::MetaList> {
        match self {
            Self::List(l) => Some(l),
            _ => None,
        }
    }

    fn as_name_value(&self) -> Option<&syn::MetaNameValue> {
        match self {
            Self::NameValue(nv) => Some(nv),
            _ => None,
        }
    }

    fn is(&self, name: &str) -> bool {
        self.path().is_ident(name)
    }

    fn get(&self, path: &str) -> Option<Meta> {
        let parsed = MetaPath::parse(path).ok()?;
        self.resolve(&parsed)
    }

    fn nested(&self) -> Option<Vec<Meta>> {
        let list = match self {
            Self::List(list) => list,
            _ => return None,
        };

        list.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
            .ok()
            .map(|p| p.into_iter().collect())
    }
}

trait MetaExtPrivate {
    fn resolve(&self, path: &MetaPath) -> Option<Meta>;
}

impl MetaExtPrivate for Meta {
    fn resolve(&self, path: &MetaPath) -> Option<Meta> {
        let seg = path.first()?;
        let tail = path.tail();

        let list = match self {
            Self::List(list) => list,
            _ => return None,
        };

        let nested: Vec<Meta> = list
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
            .ok()?
            .into_iter()
            .collect();

        let found = match seg {
            Segment::Key(name) => nested.iter().find(|m| m.path().is_ident(name))?.clone(),
            Segment::Index(i) => nested.get(*i)?.clone(),
        };

        if tail.is_empty() {
            Some(found)
        } else {
            found.resolve(&tail)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta_from(attr_str: &str) -> Meta {
        let item: syn::ItemStruct = syn::parse_str(&format!("{} struct Foo;", attr_str)).unwrap();
        item.attrs.into_iter().next().unwrap().meta
    }

    mod predicates {
        use super::*;

        #[test]
        fn path_variant() {
            let meta = meta_from("#[test]");
            assert!(meta.is_path());
            assert!(!meta.is_list());
            assert!(!meta.is_name_value());
        }

        #[test]
        fn list_variant() {
            let meta = meta_from("#[derive(Clone)]");
            assert!(!meta.is_path());
            assert!(meta.is_list());
            assert!(!meta.is_name_value());
        }

        #[test]
        fn name_value_variant() {
            let meta = meta_from("#[path = \"foo\"]");
            assert!(!meta.is_path());
            assert!(!meta.is_list());
            assert!(meta.is_name_value());
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn as_path_some() {
            let meta = meta_from("#[test]");
            assert!(meta.as_path().is_some());
        }

        #[test]
        fn as_path_none() {
            let meta = meta_from("#[derive(Clone)]");
            assert!(meta.as_path().is_none());
        }

        #[test]
        fn as_list_some() {
            let meta = meta_from("#[derive(Clone)]");
            assert!(meta.as_list().is_some());
        }

        #[test]
        fn as_list_none() {
            let meta = meta_from("#[test]");
            assert!(meta.as_list().is_none());
        }

        #[test]
        fn as_name_value_some() {
            let meta = meta_from("#[path = \"foo\"]");
            assert!(meta.as_name_value().is_some());
        }

        #[test]
        fn as_name_value_none() {
            let meta = meta_from("#[test]");
            assert!(meta.as_name_value().is_none());
        }
    }

    mod is {
        use super::*;

        #[test]
        fn matching_name() {
            let meta = meta_from("#[serde(skip)]");
            assert!(meta.is("serde"));
        }

        #[test]
        fn non_matching_name() {
            let meta = meta_from("#[serde(skip)]");
            assert!(!meta.is("derive"));
        }
    }

    mod get {
        use super::*;

        #[test]
        fn find_by_key() {
            let meta = meta_from("#[serde(rename = \"id\", skip)]");
            let found = meta.get("skip").unwrap();
            assert!(found.is_path());
        }

        #[test]
        fn find_name_value() {
            let meta = meta_from("#[serde(rename = \"id\")]");
            let found = meta.get("rename").unwrap();
            assert!(found.is_name_value());
        }

        #[test]
        fn find_by_index() {
            let meta = meta_from("#[derive(Clone, Debug)]");
            let found = meta.get("[1]").unwrap();
            assert!(found.path().is_ident("Debug"));
        }

        #[test]
        fn missing_key() {
            let meta = meta_from("#[serde(skip)]");
            assert!(meta.get("rename").is_none());
        }

        #[test]
        fn get_on_path_variant() {
            let meta = meta_from("#[test]");
            assert!(meta.get("anything").is_none());
        }

        #[test]
        fn deep_nested_key_path() {
            let meta = meta_from("#[cfg(all(feature = \"a\", feature = \"b\"))]");
            let all = meta.get("all").unwrap();
            assert!(all.is_list());
        }

        #[test]
        fn deep_nested_dot_path() {
            let meta = meta_from("#[cfg(all(feature = \"a\", feature = \"b\"))]");
            let feature = meta.get("all.feature").unwrap();
            assert!(feature.is_name_value());
        }

        #[test]
        fn deep_nested_index_path() {
            let meta = meta_from("#[cfg(all(feature = \"a\", target_os = \"linux\"))]");
            let second = meta.get("all.[1]").unwrap();
            assert!(second.is_name_value());
            assert!(second.path().is_ident("target_os"));
        }

        #[test]
        fn three_levels_deep() {
            let meta = meta_from("#[cfg(all(not(feature = \"a\")))]");
            let not = meta.get("all.not").unwrap();
            assert!(not.is_list());
            let feature = meta.get("all.not.feature").unwrap();
            assert!(feature.is_name_value());
        }

        #[test]
        fn deep_missing_intermediate() {
            let meta = meta_from("#[cfg(all(feature = \"a\"))]");
            assert!(meta.get("all.nonexistent.feature").is_none());
        }

        #[test]
        fn deep_index_out_of_bounds() {
            let meta = meta_from("#[cfg(all(feature = \"a\"))]");
            assert!(meta.get("all.[5]").is_none());
        }

        #[test]
        fn deep_index_then_key() {
            let meta = meta_from("#[cfg(all(not(feature = \"a\"), target_os = \"linux\"))]");
            let feature = meta.get("all.[0].feature").unwrap();
            assert!(feature.is_name_value());
        }
    }

    mod nested {
        use super::*;

        #[test]
        fn list_returns_items() {
            let meta = meta_from("#[derive(Clone, Debug)]");
            let items = meta.nested().unwrap();
            assert_eq!(items.len(), 2);
        }

        #[test]
        fn path_returns_none() {
            let meta = meta_from("#[test]");
            assert!(meta.nested().is_none());
        }

        #[test]
        fn name_value_returns_none() {
            let meta = meta_from("#[path = \"foo\"]");
            assert!(meta.nested().is_none());
        }
    }
}
