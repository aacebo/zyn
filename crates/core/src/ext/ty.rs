//! Extension traits for `syn::Type` inspection.
//!
//! [`TypeExt`] detects common wrapper types like `Option<T>` and `Result<T, E>`
//! and extracts their inner type parameter.
//!
//! Implemented for `syn::Type` and `syn::Field`.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::TypeExt;
//!
//! if field.is_option() {
//!     let inner_ty = field.inner_type().unwrap();
//! }
//! ```

use syn::Type;

/// Extension methods for detecting and unwrapping common type wrappers.
///
/// Recognizes `Option<T>` and `Result<T, E>` by their last path segment.
/// Implemented for `syn::Type` and `syn::Field`.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::TypeExt;
///
/// let ty: syn::Type = syn::parse_str("Option<String>").unwrap();
/// assert!(ty.is_option());
/// assert!(!ty.is_result());
/// let inner = ty.inner_type().unwrap(); // → String
/// ```
pub trait TypeExt {
    /// Returns `true` if the type is `Option<T>`.
    fn is_option(&self) -> bool;
    /// Returns `true` if the type is `Result<T, E>`.
    fn is_result(&self) -> bool;
    /// Extracts the first type parameter from `Option<T>` or `Result<T, E>`.
    /// Returns `None` if the type is neither.
    fn inner_type(&self) -> Option<&Type>;
    /// Returns the `syn::Path` if this is a `Type::Path`.
    fn as_path(&self) -> Option<&syn::Path>;
    /// Returns the inner type for wrapper variants (Array, Group, Paren, Ptr, Reference, Slice).
    /// Returns `None` for all other variants.
    fn inner(&self) -> Option<&Type>;
}

impl TypeExt for Type {
    fn is_option(&self) -> bool {
        self.last_segment_is("Option")
    }

    fn is_result(&self) -> bool {
        self.last_segment_is("Result")
    }

    fn inner_type(&self) -> Option<&Type> {
        self.first_type_arg()
    }

    fn as_path(&self) -> Option<&syn::Path> {
        match self {
            Self::Path(tp) => Some(&tp.path),
            _ => None,
        }
    }

    fn inner(&self) -> Option<&Type> {
        match self {
            Self::Array(t) => Some(&t.elem),
            Self::Group(t) => Some(&t.elem),
            Self::Paren(t) => Some(&t.elem),
            Self::Ptr(t) => Some(&t.elem),
            Self::Reference(t) => Some(&t.elem),
            Self::Slice(t) => Some(&t.elem),
            _ => None,
        }
    }
}

impl TypeExt for syn::Field {
    fn is_option(&self) -> bool {
        self.ty.is_option()
    }

    fn is_result(&self) -> bool {
        self.ty.is_result()
    }

    fn inner_type(&self) -> Option<&Type> {
        self.ty.inner_type()
    }

    fn as_path(&self) -> Option<&syn::Path> {
        self.ty.as_path()
    }

    fn inner(&self) -> Option<&Type> {
        self.ty.inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod type_ext {
        use super::*;

        #[test]
        fn is_option_true() {
            let ty: Type = syn::parse_str("Option<String>").unwrap();
            assert!(ty.is_option());
        }

        #[test]
        fn is_option_false() {
            let ty: Type = syn::parse_str("String").unwrap();
            assert!(!ty.is_option());
        }

        #[test]
        fn is_result_true() {
            let ty: Type = syn::parse_str("Result<String, Error>").unwrap();
            assert!(ty.is_result());
        }

        #[test]
        fn is_result_false() {
            let ty: Type = syn::parse_str("Option<String>").unwrap();
            assert!(!ty.is_result());
        }

        #[test]
        fn inner_type_option() {
            let ty: Type = syn::parse_str("Option<String>").unwrap();
            let inner = ty.inner_type().unwrap();
            let inner_path: Type = syn::parse_str("String").unwrap();
            assert_eq!(*inner, inner_path);
        }

        #[test]
        fn inner_type_result() {
            let ty: Type = syn::parse_str("Result<String, Error>").unwrap();
            let inner = ty.inner_type().unwrap();
            let expected: Type = syn::parse_str("String").unwrap();
            assert_eq!(*inner, expected);
        }

        #[test]
        fn inner_type_plain() {
            let ty: Type = syn::parse_str("String").unwrap();
            assert!(ty.inner_type().is_none());
        }

        #[test]
        fn as_path_on_path_type() {
            let ty: Type = syn::parse_str("std::string::String").unwrap();
            assert!(ty.as_path().is_some());
        }

        #[test]
        fn as_path_on_reference() {
            let ty: Type = syn::parse_str("&str").unwrap();
            assert!(ty.as_path().is_none());
        }

        #[test]
        fn inner_on_reference() {
            let ty: Type = syn::parse_str("&String").unwrap();
            let inner = ty.inner().unwrap();
            let expected: Type = syn::parse_str("String").unwrap();
            assert_eq!(*inner, expected);
        }

        #[test]
        fn inner_on_slice() {
            let ty: Type = syn::parse_str("[u8]").unwrap();
            let inner = ty.inner().unwrap();
            let expected: Type = syn::parse_str("u8").unwrap();
            assert_eq!(*inner, expected);
        }

        #[test]
        fn inner_on_path_type() {
            let ty: Type = syn::parse_str("String").unwrap();
            assert!(ty.inner().is_none());
        }
    }

    mod field_ext {
        use super::*;

        fn parse_field(input: &str) -> syn::Field {
            let item: syn::ItemStruct = syn::parse_str(input).unwrap();
            item.fields.into_iter().next().unwrap()
        }

        #[test]
        fn is_option_through_field() {
            let field = parse_field("struct Foo { x: Option<String> }");
            assert!(field.is_option());
        }

        #[test]
        fn is_result_through_field() {
            let field = parse_field("struct Foo { x: Result<String, Error> }");
            assert!(field.is_result());
        }

        #[test]
        fn inner_type_through_field() {
            let field = parse_field("struct Foo { x: Option<i32> }");
            let inner = field.inner_type().unwrap();
            let expected: Type = syn::parse_str("i32").unwrap();
            assert_eq!(*inner, expected);
        }

        #[test]
        fn as_path_through_field() {
            let field = parse_field("struct Foo { x: String }");
            assert!(field.as_path().is_some());
        }

        #[test]
        fn inner_through_field() {
            let field = parse_field("struct Foo { x: &String }");
            assert!(field.inner().is_some());
        }
    }
}

trait TypeExtPrivate {
    fn last_segment_is(&self, name: &str) -> bool;
    fn first_type_arg(&self) -> Option<&Type>;
}

impl TypeExtPrivate for Type {
    fn last_segment_is(&self, name: &str) -> bool {
        match self {
            Self::Path(tp) => tp.path.segments.last().is_some_and(|s| s.ident == name),
            _ => false,
        }
    }

    fn first_type_arg(&self) -> Option<&Type> {
        let seg = match self {
            Self::Path(tp) => tp.path.segments.last()?,
            _ => return None,
        };

        if seg.ident != "Option" && seg.ident != "Result" {
            return None;
        }

        match &seg.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                args.args.first().and_then(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })
            }
            _ => None,
        }
    }
}
