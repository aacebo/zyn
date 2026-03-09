//! Extension trait for `syn::Data` inspection.
//!
//! [`DataExt`] adds variant predicates and conversions to `syn::Data`.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::DataExt;
//!
//! if data.is_struct() {
//!     let s = data.as_struct().unwrap();
//!     // s.fields ...
//! }
//! ```

use syn::Data;

/// Extension methods for `syn::Data`.
///
/// Provides variant predicates (`is_struct`, `is_enum`, `is_union`)
/// and conversions (`as_struct`, `as_enum`, `as_union`).
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::DataExt;
///
/// assert!(data.is_enum());
/// let e = data.as_enum().unwrap();
/// // e.variants ...
/// ```
pub trait DataExt {
    /// Returns `true` if this is `Data::Struct`.
    fn is_struct(&self) -> bool;
    /// Returns `true` if this is `Data::Enum`.
    fn is_enum(&self) -> bool;
    /// Returns `true` if this is `Data::Union`.
    fn is_union(&self) -> bool;
    /// Returns the inner `syn::DataStruct` if this is a struct.
    fn as_struct(&self) -> Option<&syn::DataStruct>;
    /// Returns the inner `syn::DataEnum` if this is an enum.
    fn as_enum(&self) -> Option<&syn::DataEnum>;
    /// Returns the inner `syn::DataUnion` if this is a union.
    fn as_union(&self) -> Option<&syn::DataUnion>;
}

impl DataExt for Data {
    fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    fn as_struct(&self) -> Option<&syn::DataStruct> {
        match self {
            Self::Struct(s) => Some(s),
            _ => None,
        }
    }

    fn as_enum(&self) -> Option<&syn::DataEnum> {
        match self {
            Self::Enum(e) => Some(e),
            _ => None,
        }
    }

    fn as_union(&self) -> Option<&syn::DataUnion> {
        match self {
            Self::Union(u) => Some(u),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data_from(input: &str) -> Data {
        let di: syn::DeriveInput = syn::parse_str(input).unwrap();
        di.data
    }

    mod predicates {
        use super::*;

        #[test]
        fn struct_variant() {
            let data = data_from("struct Foo { x: i32 }");
            assert!(data.is_struct());
            assert!(!data.is_enum());
            assert!(!data.is_union());
        }

        #[test]
        fn enum_variant() {
            let data = data_from("enum Foo { A, B }");
            assert!(!data.is_struct());
            assert!(data.is_enum());
            assert!(!data.is_union());
        }

        #[test]
        fn union_variant() {
            let data = data_from("union Foo { x: i32, y: u32 }");
            assert!(!data.is_struct());
            assert!(!data.is_enum());
            assert!(data.is_union());
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn as_struct_some() {
            let data = data_from("struct Foo { x: i32 }");
            assert!(data.as_struct().is_some());
        }

        #[test]
        fn as_struct_none() {
            let data = data_from("enum Foo { A }");
            assert!(data.as_struct().is_none());
        }

        #[test]
        fn as_enum_some() {
            let data = data_from("enum Foo { A, B }");
            assert!(data.as_enum().is_some());
        }

        #[test]
        fn as_enum_none() {
            let data = data_from("struct Foo;");
            assert!(data.as_enum().is_none());
        }

        #[test]
        fn as_union_some() {
            let data = data_from("union Foo { x: i32, y: u32 }");
            assert!(data.as_union().is_some());
        }

        #[test]
        fn as_union_none() {
            let data = data_from("struct Foo;");
            assert!(data.as_union().is_none());
        }
    }
}
