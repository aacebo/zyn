//! Extension traits for `syn::Fields` and a unified [`FieldKey`] lookup type.
//!
//! [`FieldsExt`] adds variant predicates and field lookup to any type that
//! contains fields — `syn::Fields`, `syn::ItemStruct`, `syn::DataStruct`,
//! and `syn::Variant`.
//!
//! [`FieldKey`] unifies named and indexed field access so callers don't need
//! to branch on field kind.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::{FieldsExt, FieldKey};
//!
//! if fields.is_named() {
//!     let id_field = fields.get(&"id".into());
//! }
//!
//! let first = fields.get(&0usize.into());
//! let exists = fields.exists(&"name".into());
//! ```

use proc_macro2::Span;
use quote::ToTokens;
use syn::Field;

/// Identifies a struct field by name or position.
///
/// Construct from a `syn::Ident`, `syn::Index`, `&str`, or `usize`.
/// Implements [`ToTokens`] and [`Display`](std::fmt::Display) for use in
/// generated code and error messages.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::FieldKey;
///
/// let named = "id".into();
/// let indexed = 0usize.into();
///
/// assert!(named.is_named());
/// assert!(indexed.is_index());
/// ```
pub enum FieldKey {
    /// A named field identified by its `syn::Ident`.
    Named(syn::Ident),
    /// A positional field identified by its `syn::Index`.
    Index(syn::Index),
}

impl FieldKey {
    /// Returns `true` if this key refers to a named field.
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    /// Returns `true` if this key refers to a positional field.
    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    /// Returns the identifier if this is a named key.
    pub fn as_named(&self) -> Option<&syn::Ident> {
        match self {
            Self::Named(ident) => Some(ident),
            _ => None,
        }
    }

    /// Returns the index if this is a positional key.
    pub fn as_index(&self) -> Option<&syn::Index> {
        match self {
            Self::Index(index) => Some(index),
            _ => None,
        }
    }
}

impl From<syn::Ident> for FieldKey {
    fn from(ident: syn::Ident) -> Self {
        Self::Named(ident)
    }
}

impl From<syn::Index> for FieldKey {
    fn from(index: syn::Index) -> Self {
        Self::Index(index)
    }
}

impl From<usize> for FieldKey {
    fn from(index: usize) -> Self {
        Self::Index(syn::Index {
            index: index as u32,
            span: Span::call_site(),
        })
    }
}

impl From<&str> for FieldKey {
    fn from(name: &str) -> Self {
        Self::Named(syn::Ident::new(name, Span::call_site()))
    }
}

impl std::fmt::Display for FieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(ident) => write!(f, "{}", ident),
            Self::Index(index) => write!(f, "{}", index.index),
        }
    }
}

impl ToTokens for FieldKey {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Named(ident) => ident.to_tokens(tokens),
            Self::Index(index) => index.to_tokens(tokens),
        }
    }
}

/// Extension methods for types that contain `syn::Fields`.
///
/// Provides variant predicates (`is_named`, `is_unnamed`, `is_unit`),
/// conversions (`as_named`, `as_unnamed`), and field lookup via [`FieldKey`].
///
/// Implemented for `syn::Fields`, `syn::ItemStruct`, `syn::DataStruct`,
/// and `syn::Variant`.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::{FieldsExt, FieldKey};
///
/// #[zyn::element]
/// fn my_element(fields: zyn::Fields) -> zyn::TokenStream {
///     if fields.is_named() {
///         let f = fields.get(&"id".into());
///     }
///     // ...
/// }
/// ```
pub trait FieldsExt {
    /// Returns `true` if the fields are named (struct `{ a: T, b: U }`).
    fn is_named(&self) -> bool;
    /// Returns `true` if the fields are unnamed (tuple `(T, U)`).
    fn is_unnamed(&self) -> bool;
    /// Returns `true` if there are no fields (unit struct).
    fn is_unit(&self) -> bool;
    /// Returns the inner `syn::FieldsNamed` if the fields are named.
    fn as_named(&self) -> Option<&syn::FieldsNamed>;
    /// Returns the inner `syn::FieldsUnnamed` if the fields are unnamed.
    fn as_unnamed(&self) -> Option<&syn::FieldsUnnamed>;
    /// Returns `true` if a field matching the given [`FieldKey`] exists.
    fn exists(&self, key: &FieldKey) -> bool;
    /// Returns the first field matching the given [`FieldKey`], or `None`.
    fn get(&self, key: &FieldKey) -> Option<&Field>;
    /// Returns all fields as `(FieldKey, &Field)` pairs.
    ///
    /// Named fields yield [`FieldKey::Named`], unnamed fields yield
    /// [`FieldKey::Index`], and unit fields yield an empty vec.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use zyn::ext::{FieldsExt, FieldKey};
    ///
    /// for (key, field) in fields.keyed() {
    ///     println!("{}: {:?}", key, field.ty);
    /// }
    /// ```
    fn keyed(&self) -> impl Iterator<Item = (FieldKey, &Field)>;
}

impl FieldsExt for syn::Fields {
    fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    fn is_unnamed(&self) -> bool {
        matches!(self, Self::Unnamed(_))
    }

    fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    fn as_named(&self) -> Option<&syn::FieldsNamed> {
        match self {
            Self::Named(f) => Some(f),
            _ => None,
        }
    }

    fn as_unnamed(&self) -> Option<&syn::FieldsUnnamed> {
        match self {
            Self::Unnamed(f) => Some(f),
            _ => None,
        }
    }

    fn exists(&self, key: &FieldKey) -> bool {
        self.get(key).is_some()
    }

    fn keyed(&self) -> impl Iterator<Item = (FieldKey, &Field)> {
        let pairs: Vec<_> = match self {
            Self::Named(f) => f
                .named
                .iter()
                .map(|field| {
                    let ident = field.ident.clone().unwrap();
                    (FieldKey::Named(ident), field)
                })
                .collect(),
            Self::Unnamed(f) => f
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| (FieldKey::from(i), field))
                .collect(),
            Self::Unit => Vec::new(),
        };

        pairs.into_iter()
    }

    fn get(&self, key: &FieldKey) -> Option<&Field> {
        match key {
            FieldKey::Named(ident) => {
                let named = self.as_named()?;
                named.named.iter().find(|f| f.ident.as_ref() == Some(ident))
            }
            FieldKey::Index(index) => {
                let mut iter: Box<dyn Iterator<Item = &Field>> = match self {
                    Self::Named(f) => Box::new(f.named.iter()),
                    Self::Unnamed(f) => Box::new(f.unnamed.iter()),
                    Self::Unit => return None,
                };
                iter.nth(index.index as usize)
            }
        }
    }
}

macro_rules! delegate_fields_ext {
    ($ty:ty, $field:ident) => {
        impl FieldsExt for $ty {
            fn is_named(&self) -> bool {
                self.$field.is_named()
            }

            fn is_unnamed(&self) -> bool {
                self.$field.is_unnamed()
            }

            fn is_unit(&self) -> bool {
                self.$field.is_unit()
            }

            fn as_named(&self) -> Option<&syn::FieldsNamed> {
                self.$field.as_named()
            }

            fn as_unnamed(&self) -> Option<&syn::FieldsUnnamed> {
                self.$field.as_unnamed()
            }

            fn exists(&self, key: &FieldKey) -> bool {
                self.$field.exists(key)
            }

            fn keyed(&self) -> impl Iterator<Item = (FieldKey, &Field)> {
                self.$field.keyed()
            }

            fn get(&self, key: &FieldKey) -> Option<&Field> {
                self.$field.get(key)
            }
        }
    };
}

delegate_fields_ext!(syn::ItemStruct, fields);
delegate_fields_ext!(syn::DataStruct, fields);
delegate_fields_ext!(syn::Variant, fields);

#[cfg(test)]
mod tests {
    use super::*;

    fn named_fields() -> syn::Fields {
        let item: syn::ItemStruct = syn::parse_str("struct Foo { x: i32, y: String }").unwrap();
        item.fields
    }

    fn unnamed_fields() -> syn::Fields {
        let item: syn::ItemStruct = syn::parse_str("struct Foo(i32, String);").unwrap();
        item.fields
    }

    fn unit_fields() -> syn::Fields {
        let item: syn::ItemStruct = syn::parse_str("struct Foo;").unwrap();
        item.fields
    }

    mod field_key {
        use super::*;

        #[test]
        fn from_str() {
            let key: FieldKey = "id".into();
            assert!(key.is_named());
            assert!(!key.is_index());
        }

        #[test]
        fn from_usize() {
            let key: FieldKey = 0usize.into();
            assert!(key.is_index());
            assert!(!key.is_named());
        }

        #[test]
        fn as_named_some() {
            let key: FieldKey = "id".into();
            assert!(key.as_named().is_some());
        }

        #[test]
        fn as_named_none() {
            let key: FieldKey = 0usize.into();
            assert!(key.as_named().is_none());
        }

        #[test]
        fn as_index_some() {
            let key: FieldKey = 0usize.into();
            assert!(key.as_index().is_some());
        }

        #[test]
        fn as_index_none() {
            let key: FieldKey = "id".into();
            assert!(key.as_index().is_none());
        }

        #[test]
        fn display_named() {
            let key: FieldKey = "id".into();
            assert_eq!(key.to_string(), "id");
        }

        #[test]
        fn display_index() {
            let key: FieldKey = 3usize.into();
            assert_eq!(key.to_string(), "3");
        }
    }

    mod predicates {
        use super::*;

        #[test]
        fn named_struct() {
            let fields = named_fields();
            assert!(fields.is_named());
            assert!(!fields.is_unnamed());
            assert!(!fields.is_unit());
        }

        #[test]
        fn tuple_struct() {
            let fields = unnamed_fields();
            assert!(!fields.is_named());
            assert!(fields.is_unnamed());
            assert!(!fields.is_unit());
        }

        #[test]
        fn unit_struct() {
            let fields = unit_fields();
            assert!(!fields.is_named());
            assert!(!fields.is_unnamed());
            assert!(fields.is_unit());
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn as_named_some() {
            let fields = named_fields();
            assert!(fields.as_named().is_some());
        }

        #[test]
        fn as_named_none() {
            let fields = unnamed_fields();
            assert!(fields.as_named().is_none());
        }

        #[test]
        fn as_unnamed_some() {
            let fields = unnamed_fields();
            assert!(fields.as_unnamed().is_some());
        }

        #[test]
        fn as_unnamed_none() {
            let fields = named_fields();
            assert!(fields.as_unnamed().is_none());
        }
    }

    mod get {
        use super::*;

        #[test]
        fn by_name() {
            let fields = named_fields();
            let key: FieldKey = "x".into();
            assert!(fields.get(&key).is_some());
        }

        #[test]
        fn by_name_missing() {
            let fields = named_fields();
            let key: FieldKey = "z".into();
            assert!(fields.get(&key).is_none());
        }

        #[test]
        fn by_index_named() {
            let fields = named_fields();
            let key: FieldKey = 0usize.into();
            assert!(fields.get(&key).is_some());
        }

        #[test]
        fn by_index_unnamed() {
            let fields = unnamed_fields();
            let key: FieldKey = 1usize.into();
            assert!(fields.get(&key).is_some());
        }

        #[test]
        fn by_index_out_of_bounds() {
            let fields = named_fields();
            let key: FieldKey = 10usize.into();
            assert!(fields.get(&key).is_none());
        }

        #[test]
        fn on_unit() {
            let fields = unit_fields();
            let key: FieldKey = 0usize.into();
            assert!(fields.get(&key).is_none());
        }
    }

    mod exists {
        use super::*;

        #[test]
        fn existing_field() {
            let fields = named_fields();
            let key: FieldKey = "x".into();
            assert!(fields.exists(&key));
        }

        #[test]
        fn missing_field() {
            let fields = named_fields();
            let key: FieldKey = "z".into();
            assert!(!fields.exists(&key));
        }
    }

    mod keyed {
        use super::*;

        #[test]
        fn named_yields_named_keys() {
            let fields = named_fields();
            let pairs: Vec<_> = fields.keyed().collect();
            assert_eq!(pairs.len(), 2);
            assert!(pairs[0].0.is_named());
            assert_eq!(pairs[0].0.to_string(), "x");
            assert_eq!(pairs[1].0.to_string(), "y");
        }

        #[test]
        fn unnamed_yields_index_keys() {
            let fields = unnamed_fields();
            let pairs: Vec<_> = fields.keyed().collect();
            assert_eq!(pairs.len(), 2);
            assert!(pairs[0].0.is_index());
            assert_eq!(pairs[0].0.to_string(), "0");
            assert_eq!(pairs[1].0.to_string(), "1");
        }

        #[test]
        fn unit_yields_empty() {
            let fields = unit_fields();
            let pairs: Vec<_> = fields.keyed().collect();
            assert!(pairs.is_empty());
        }
    }
}
