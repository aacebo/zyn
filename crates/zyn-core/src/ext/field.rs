//! Extension trait for `syn::Field` metadata querying.
//!
//! [`FieldExt`] adds dot-path metadata navigation to individual struct/enum fields.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::FieldExt;
//!
//! // struct Foo {
//! //     #[serde(rename = "id", skip)]
//! //     bar: String,
//! // }
//! let rename = field.get("serde.rename"); // → Some(NameValue meta)
//! let skip = field.get("serde.skip");     // → Some(Path meta)
//! ```

use syn::Meta;

use super::AttrExt;

/// Extension methods for a single `syn::Field`.
///
/// Provides dot-path metadata querying across the field's attributes.
/// The first path segment matches the attribute name, subsequent segments
/// drill into nested metadata.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::FieldExt;
///
/// // #[serde(rename = "user_id")]
/// // pub id: i64,
/// let meta = field.get("serde.rename"); // → Some(NameValue meta)
/// let serde = field.get("serde");       // → Some(List meta)
/// ```
pub trait FieldExt {
    /// Navigates into a field's attributes using a dot-separated path.
    ///
    /// The first segment matches the attribute name, subsequent segments
    /// drill into nested metadata using [`MetaPath`](crate::path::MetaPath) syntax.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::ext::FieldExt;
    ///
    /// // #[serde(rename = "user_id", skip)]
    /// let rename = field.get("serde.rename"); // → Some(NameValue)
    /// let skip = field.get("serde.skip");     // → Some(Path)
    /// ```
    fn get(&self, path: &str) -> Option<Meta>;
}

impl FieldExt for syn::Field {
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

    fn parse_fields(input: &str) -> Vec<syn::Field> {
        let item: syn::ItemStruct = syn::parse_str(input).unwrap();
        item.fields.into_iter().collect()
    }

    mod get {
        use super::*;

        #[test]
        fn dot_path_into_attr() {
            let fields = parse_fields("struct Foo { #[serde(rename = \"id\", skip)] x: i32 }");
            let meta = fields[0].get("serde.skip");
            assert!(meta.is_some());
        }

        #[test]
        fn returns_attr_meta_for_single_segment() {
            let fields = parse_fields("struct Foo { #[serde(skip)] x: i32 }");
            let meta = fields[0].get("serde").unwrap();
            assert!(meta.path().is_ident("serde"));
        }

        #[test]
        fn missing_attr() {
            let fields = parse_fields("struct Foo { x: i32 }");
            assert!(fields[0].get("serde.skip").is_none());
        }

        #[test]
        fn missing_nested_key() {
            let fields = parse_fields("struct Foo { #[serde(skip)] x: i32 }");
            assert!(fields[0].get("serde.rename").is_none());
        }

        #[test]
        fn finds_correct_attr_among_multiple() {
            let fields = parse_fields("struct Foo { #[doc = \"hi\"] #[serde(skip)] x: i32 }");
            let meta = fields[0].get("serde").unwrap();
            assert!(meta.path().is_ident("serde"));
        }
    }
}
