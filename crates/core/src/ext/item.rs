//! Extension trait for `syn::Item` inspection.
//!
//! [`ItemExt`] adds variant predicates, conversions, and common field
//! accessors to `syn::Item`.
//!
//! # Examples
//!
//! ```ignore
//! use zyn::ext::ItemExt;
//!
//! if item.is_struct() {
//!     let s = item.as_struct().unwrap();
//! }
//!
//! let attrs = item.attrs();
//! let ident = item.ident();
//! ```

use syn::Item;

/// Extension methods for `syn::Item`.
///
/// Provides variant predicates (`is_struct`, `is_enum`, `is_fn`, etc.),
/// conversions (`as_struct`, `as_enum`, `as_fn`, etc.), and common field
/// accessors (`attrs`, `ident`, `generics`, `vis`) that work across
/// all applicable variants.
///
/// # Examples
///
/// ```ignore
/// use zyn::ext::ItemExt;
///
/// assert!(item.is_struct());
/// let attrs = item.attrs();
/// let ident = item.ident().unwrap();
/// ```
pub trait ItemExt {
    /// Returns `true` if this is `Item::Struct`.
    fn is_struct(&self) -> bool;
    /// Returns `true` if this is `Item::Enum`.
    fn is_enum(&self) -> bool;
    /// Returns `true` if this is `Item::Union`.
    fn is_union(&self) -> bool;
    /// Returns `true` if this is `Item::Fn`.
    fn is_fn(&self) -> bool;
    /// Returns `true` if this is `Item::Trait`.
    fn is_trait(&self) -> bool;
    /// Returns `true` if this is `Item::Impl`.
    fn is_impl(&self) -> bool;
    /// Returns `true` if this is `Item::Mod`.
    fn is_mod(&self) -> bool;
    /// Returns `true` if this is `Item::Type`.
    fn is_type(&self) -> bool;
    /// Returns `true` if this is `Item::Const`.
    fn is_const(&self) -> bool;
    /// Returns `true` if this is `Item::Static`.
    fn is_static(&self) -> bool;
    /// Returns `true` if this is `Item::Use`.
    fn is_use(&self) -> bool;
    /// Returns the inner `syn::ItemStruct` if this is a struct.
    fn as_struct(&self) -> Option<&syn::ItemStruct>;
    /// Returns the inner `syn::ItemEnum` if this is an enum.
    fn as_enum(&self) -> Option<&syn::ItemEnum>;
    /// Returns the inner `syn::ItemUnion` if this is a union.
    fn as_union(&self) -> Option<&syn::ItemUnion>;
    /// Returns the inner `syn::ItemFn` if this is a function.
    fn as_fn(&self) -> Option<&syn::ItemFn>;
    /// Returns the inner `syn::ItemTrait` if this is a trait.
    fn as_trait(&self) -> Option<&syn::ItemTrait>;
    /// Returns the inner `syn::ItemImpl` if this is an impl block.
    fn as_impl(&self) -> Option<&syn::ItemImpl>;
    /// Returns the inner `syn::ItemMod` if this is a module.
    fn as_mod(&self) -> Option<&syn::ItemMod>;
    /// Returns the inner `syn::ItemType` if this is a type alias.
    fn as_type(&self) -> Option<&syn::ItemType>;
    /// Returns the inner `syn::ItemConst` if this is a constant.
    fn as_const(&self) -> Option<&syn::ItemConst>;
    /// Returns the inner `syn::ItemStatic` if this is a static.
    fn as_static(&self) -> Option<&syn::ItemStatic>;
    /// Returns the inner `syn::ItemUse` if this is a use declaration.
    fn as_use(&self) -> Option<&syn::ItemUse>;
    /// Returns the attributes for this item. All variants have attributes.
    fn attrs(&self) -> &[syn::Attribute];
    /// Returns the identifier if this variant has one.
    /// Returns `None` for `Impl`, `Use`, `ForeignMod`, and `Verbatim`.
    fn ident(&self) -> Option<&syn::Ident>;
    /// Returns the generics if this variant has them.
    /// Returns `None` for variants without generic parameters.
    fn generics(&self) -> Option<&syn::Generics>;
    /// Returns the visibility if this variant has one.
    /// Returns `None` for `Impl`, `ForeignMod`, and `Verbatim`.
    fn vis(&self) -> Option<&syn::Visibility>;
    /// Returns the span of this item.
    fn span(&self) -> proc_macro2::Span;
}

impl ItemExt for Item {
    fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    fn is_fn(&self) -> bool {
        matches!(self, Self::Fn(_))
    }

    fn is_trait(&self) -> bool {
        matches!(self, Self::Trait(_))
    }

    fn is_impl(&self) -> bool {
        matches!(self, Self::Impl(_))
    }

    fn is_mod(&self) -> bool {
        matches!(self, Self::Mod(_))
    }

    fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    fn is_use(&self) -> bool {
        matches!(self, Self::Use(_))
    }

    fn as_struct(&self) -> Option<&syn::ItemStruct> {
        match self {
            Self::Struct(v) => Some(v),
            _ => None,
        }
    }

    fn as_enum(&self) -> Option<&syn::ItemEnum> {
        match self {
            Self::Enum(v) => Some(v),
            _ => None,
        }
    }

    fn as_union(&self) -> Option<&syn::ItemUnion> {
        match self {
            Self::Union(v) => Some(v),
            _ => None,
        }
    }

    fn as_fn(&self) -> Option<&syn::ItemFn> {
        match self {
            Self::Fn(v) => Some(v),
            _ => None,
        }
    }

    fn as_trait(&self) -> Option<&syn::ItemTrait> {
        match self {
            Self::Trait(v) => Some(v),
            _ => None,
        }
    }

    fn as_impl(&self) -> Option<&syn::ItemImpl> {
        match self {
            Self::Impl(v) => Some(v),
            _ => None,
        }
    }

    fn as_mod(&self) -> Option<&syn::ItemMod> {
        match self {
            Self::Mod(v) => Some(v),
            _ => None,
        }
    }

    fn as_type(&self) -> Option<&syn::ItemType> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }

    fn as_const(&self) -> Option<&syn::ItemConst> {
        match self {
            Self::Const(v) => Some(v),
            _ => None,
        }
    }

    fn as_static(&self) -> Option<&syn::ItemStatic> {
        match self {
            Self::Static(v) => Some(v),
            _ => None,
        }
    }

    fn as_use(&self) -> Option<&syn::ItemUse> {
        match self {
            Self::Use(v) => Some(v),
            _ => None,
        }
    }

    fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Const(v) => &v.attrs,
            Self::Enum(v) => &v.attrs,
            Self::ExternCrate(v) => &v.attrs,
            Self::Fn(v) => &v.attrs,
            Self::ForeignMod(v) => &v.attrs,
            Self::Impl(v) => &v.attrs,
            Self::Mod(v) => &v.attrs,
            Self::Static(v) => &v.attrs,
            Self::Struct(v) => &v.attrs,
            Self::Trait(v) => &v.attrs,
            Self::Type(v) => &v.attrs,
            Self::Union(v) => &v.attrs,
            Self::Use(v) => &v.attrs,
            _ => &[],
        }
    }

    fn ident(&self) -> Option<&syn::Ident> {
        match self {
            Self::Const(v) => Some(&v.ident),
            Self::Enum(v) => Some(&v.ident),
            Self::ExternCrate(v) => Some(&v.ident),
            Self::Fn(v) => Some(&v.sig.ident),
            Self::Mod(v) => Some(&v.ident),
            Self::Static(v) => Some(&v.ident),
            Self::Struct(v) => Some(&v.ident),
            Self::Trait(v) => Some(&v.ident),
            Self::Type(v) => Some(&v.ident),
            Self::Union(v) => Some(&v.ident),
            _ => None,
        }
    }

    fn generics(&self) -> Option<&syn::Generics> {
        match self {
            Self::Enum(v) => Some(&v.generics),
            Self::Fn(v) => Some(&v.sig.generics),
            Self::Impl(v) => Some(&v.generics),
            Self::Struct(v) => Some(&v.generics),
            Self::Trait(v) => Some(&v.generics),
            Self::Type(v) => Some(&v.generics),
            Self::Union(v) => Some(&v.generics),
            _ => None,
        }
    }

    fn vis(&self) -> Option<&syn::Visibility> {
        match self {
            Self::Const(v) => Some(&v.vis),
            Self::Enum(v) => Some(&v.vis),
            Self::ExternCrate(v) => Some(&v.vis),
            Self::Fn(v) => Some(&v.vis),
            Self::Mod(v) => Some(&v.vis),
            Self::Static(v) => Some(&v.vis),
            Self::Struct(v) => Some(&v.vis),
            Self::Trait(v) => Some(&v.vis),
            Self::Type(v) => Some(&v.vis),
            Self::Union(v) => Some(&v.vis),
            Self::Use(v) => Some(&v.vis),
            _ => None,
        }
    }

    fn span(&self) -> proc_macro2::Span {
        use syn::spanned::Spanned;
        Spanned::span(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item_from(input: &str) -> Item {
        syn::parse_str(input).unwrap()
    }

    mod predicates {
        use super::*;

        #[test]
        fn struct_variant() {
            let item = item_from("struct Foo;");
            assert!(item.is_struct());
            assert!(!item.is_enum());
            assert!(!item.is_fn());
        }

        #[test]
        fn enum_variant() {
            let item = item_from("enum Foo { A, B }");
            assert!(item.is_enum());
            assert!(!item.is_struct());
        }

        #[test]
        fn fn_variant() {
            let item = item_from("fn foo() {}");
            assert!(item.is_fn());
            assert!(!item.is_struct());
        }

        #[test]
        fn impl_variant() {
            let item = item_from("impl Foo {}");
            assert!(item.is_impl());
        }

        #[test]
        fn trait_variant() {
            let item = item_from("trait Foo {}");
            assert!(item.is_trait());
        }

        #[test]
        fn mod_variant() {
            let item = item_from("mod foo {}");
            assert!(item.is_mod());
        }

        #[test]
        fn type_variant() {
            let item = item_from("type Foo = Bar;");
            assert!(item.is_type());
        }

        #[test]
        fn const_variant() {
            let item = item_from("const X: i32 = 0;");
            assert!(item.is_const());
        }

        #[test]
        fn static_variant() {
            let item = item_from("static X: i32 = 0;");
            assert!(item.is_static());
        }

        #[test]
        fn use_variant() {
            let item = item_from("use std::fmt;");
            assert!(item.is_use());
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn as_struct_some() {
            let item = item_from("struct Foo;");
            assert!(item.as_struct().is_some());
        }

        #[test]
        fn as_struct_none() {
            let item = item_from("enum Foo { A }");
            assert!(item.as_struct().is_none());
        }

        #[test]
        fn as_enum_some() {
            let item = item_from("enum Foo { A }");
            assert!(item.as_enum().is_some());
        }

        #[test]
        fn as_fn_some() {
            let item = item_from("fn foo() {}");
            assert!(item.as_fn().is_some());
        }

        #[test]
        fn as_impl_some() {
            let item = item_from("impl Foo {}");
            assert!(item.as_impl().is_some());
        }
    }

    mod accessors {
        use super::*;

        #[test]
        fn attrs_on_struct() {
            let item = item_from("#[derive(Clone)] struct Foo;");
            assert_eq!(item.attrs().len(), 1);
        }

        #[test]
        fn attrs_on_fn() {
            let item = item_from("#[inline] fn foo() {}");
            assert_eq!(item.attrs().len(), 1);
        }

        #[test]
        fn ident_on_struct() {
            let item = item_from("struct Foo;");
            assert_eq!(item.ident().unwrap().to_string(), "Foo");
        }

        #[test]
        fn ident_on_fn() {
            let item = item_from("fn bar() {}");
            assert_eq!(item.ident().unwrap().to_string(), "bar");
        }

        #[test]
        fn ident_on_impl_is_none() {
            let item = item_from("impl Foo {}");
            assert!(item.ident().is_none());
        }

        #[test]
        fn generics_on_struct() {
            let item = item_from("struct Foo<T> { x: T }");
            assert!(item.generics().is_some());
        }

        #[test]
        fn generics_on_const_is_none() {
            let item = item_from("const X: i32 = 0;");
            assert!(item.generics().is_none());
        }

        #[test]
        fn vis_on_pub_struct() {
            let item = item_from("pub struct Foo;");
            let vis = item.vis().unwrap();
            assert!(matches!(vis, syn::Visibility::Public(_)));
        }

        #[test]
        fn vis_on_impl_is_none() {
            let item = item_from("impl Foo {}");
            assert!(item.vis().is_none());
        }
    }
}
