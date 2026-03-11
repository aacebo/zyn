//! Extension trait for `syn::Variant` metadata querying.
//!
//! [`VariantExt`] adds dot-path metadata navigation to enum variants.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::VariantExt;
//!
//! // enum Foo {
//! //     #[serde(rename = "bar")]
//! //     A { x: i32 },
//! // }
//! let rename = variant.get("serde.rename"); // → Some(NameValue meta)
//! ```

use syn::Meta;

use super::AttrExt;

/// Extension methods for a single `syn::Variant`.
///
/// Provides dot-path metadata querying across the variant's attributes.
/// The first path segment matches the attribute name, subsequent segments
/// drill into nested metadata.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::VariantExt;
///
/// // #[serde(rename = "bar")]
/// // A { x: i32 },
/// let meta = variant.get("serde.rename"); // → Some(NameValue meta)
/// let serde = variant.get("serde");       // → Some(List meta)
/// ```
pub trait VariantExt {
    /// Navigates into a variant's attributes using a dot-separated path.
    ///
    /// The first segment matches the attribute name, subsequent segments
    /// drill into nested metadata using [`MetaPath`](crate::path::MetaPath) syntax.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::ext::VariantExt;
    ///
    /// // #[serde(rename = "bar", skip)]
    /// let rename = variant.get("serde.rename"); // → Some(NameValue)
    /// let skip = variant.get("serde.skip");     // → Some(Path)
    /// ```
    fn get(&self, path: &str) -> Option<Meta>;
}

impl VariantExt for syn::Variant {
    fn get(&self, path: &str) -> Option<Meta> {
        let parsed = crate::path::MetaPath::parse(path).ok()?;
        let first = parsed.first()?;

        let attr_name = match first {
            crate::path::Segment::Key(name) => name,
            _ => return None,
        };

        let attr = self.attrs.iter().find(|a| a.is(attr_name))?;
        let tail = parsed.tail();

        if tail.is_empty() {
            Some(attr.meta.clone())
        } else {
            attr.get(&tail.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext::MetaExt;

    fn parse_variant(input: &str) -> syn::Variant {
        let item: syn::ItemEnum = syn::parse_str(input).unwrap();
        item.variants.into_iter().next().unwrap()
    }

    mod get {
        use super::*;

        #[test]
        fn single_segment() {
            let v = parse_variant("enum Foo { #[serde(skip)] A }");
            let meta = v.get("serde").unwrap();
            assert!(meta.path().is_ident("serde"));
        }

        #[test]
        fn dot_path() {
            let v = parse_variant("enum Foo { #[serde(rename = \"bar\", skip)] A }");
            let meta = v.get("serde.skip").unwrap();
            assert!(meta.path().is_ident("skip"));
        }

        #[test]
        fn name_value() {
            let v = parse_variant("enum Foo { #[serde(rename = \"bar\")] A }");
            let meta = v.get("serde.rename").unwrap();
            assert!(meta.is_name_value());
        }

        #[test]
        fn missing_attr() {
            let v = parse_variant("enum Foo { A }");
            assert!(v.get("serde").is_none());
        }

        #[test]
        fn missing_nested() {
            let v = parse_variant("enum Foo { #[serde(skip)] A }");
            assert!(v.get("serde.rename").is_none());
        }

        #[test]
        fn among_multiple_attrs() {
            let v = parse_variant("enum Foo { #[doc = \"hi\"] #[serde(skip)] A }");
            let meta = v.get("serde.skip").unwrap();
            assert!(meta.path().is_ident("skip"));
        }
    }
}
