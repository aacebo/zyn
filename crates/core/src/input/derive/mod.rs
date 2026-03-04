mod derive_enum;
mod derive_struct;
mod derive_union;

pub use derive_enum::*;
pub use derive_struct::*;
pub use derive_union::*;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

pub enum DeriveInput {
    Struct(DeriveStruct),
    Enum(DeriveEnum),
    Union(DeriveUnion),
}

impl DeriveInput {
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }
    pub fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    pub fn as_struct(&self) -> &DeriveStruct {
        match self {
            Self::Struct(s) => s,
            _ => panic!("called `DeriveInput::as_struct()` on a non-Struct variant"),
        }
    }

    pub fn as_enum(&self) -> &DeriveEnum {
        match self {
            Self::Enum(e) => e,
            _ => panic!("called `DeriveInput::as_enum()` on a non-Enum variant"),
        }
    }

    pub fn as_union(&self) -> &DeriveUnion {
        match self {
            Self::Union(u) => u,
            _ => panic!("called `DeriveInput::as_union()` on a non-Union variant"),
        }
    }

    pub fn attrs(&self) -> &Vec<syn::Attribute> {
        match self {
            Self::Struct(s) => &s.attrs,
            Self::Enum(e) => &e.attrs,
            Self::Union(u) => &u.attrs,
        }
    }

    pub fn vis(&self) -> &syn::Visibility {
        match self {
            Self::Struct(s) => &s.vis,
            Self::Enum(e) => &e.vis,
            Self::Union(u) => &u.vis,
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Struct(s) => &s.ident,
            Self::Enum(e) => &e.ident,
            Self::Union(u) => &u.ident,
        }
    }

    pub fn generics(&self) -> &syn::Generics {
        match self {
            Self::Struct(s) => &s.generics,
            Self::Enum(e) => &e.generics,
            Self::Union(u) => &u.generics,
        }
    }
}

impl Parse for DeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let di: syn::DeriveInput = input.parse()?;
        match di.data {
            syn::Data::Struct(data) => Ok(Self::Struct(DeriveStruct {
                attrs: di.attrs,
                vis: di.vis,
                ident: di.ident,
                generics: di.generics,
                data,
            })),
            syn::Data::Enum(data) => Ok(Self::Enum(DeriveEnum {
                attrs: di.attrs,
                vis: di.vis,
                ident: di.ident,
                generics: di.generics,
                data,
            })),
            syn::Data::Union(data) => Ok(Self::Union(DeriveUnion {
                attrs: di.attrs,
                vis: di.vis,
                ident: di.ident,
                generics: di.generics,
                data,
            })),
        }
    }
}

impl ToTokens for DeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(s) => s.to_tokens(tokens),
            Self::Enum(e) => e.to_tokens(tokens),
            Self::Union(u) => u.to_tokens(tokens),
        }
    }
}

impl From<DeriveStruct> for DeriveInput {
    fn from(s: DeriveStruct) -> Self {
        Self::Struct(s)
    }
}

impl From<DeriveEnum> for DeriveInput {
    fn from(e: DeriveEnum) -> Self {
        Self::Enum(e)
    }
}

impl From<DeriveUnion> for DeriveInput {
    fn from(u: DeriveUnion) -> Self {
        Self::Union(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn struct_variant() {
            let d: DeriveInput = syn::parse_str("struct Point { x: f32, y: f32 }").unwrap();
            assert!(d.is_struct());
            assert_eq!(d.ident().to_string(), "Point");
        }

        #[test]
        fn enum_variant() {
            let d: DeriveInput = syn::parse_str("enum Dir { North, South }").unwrap();
            assert!(d.is_enum());
            assert_eq!(d.ident().to_string(), "Dir");
        }

        #[test]
        fn union_variant() {
            let d: DeriveInput = syn::parse_str("union Bits { i: i32, f: f32 }").unwrap();
            assert!(d.is_union());
            assert_eq!(d.ident().to_string(), "Bits");
        }

        #[test]
        fn fn_rejected() {
            let result: syn::Result<DeriveInput> = syn::parse_str("fn foo() {}");
            assert!(result.is_err());
        }
    }

    mod from {
        use super::*;

        #[test]
        fn from_derive_struct() {
            let s: DeriveStruct = syn::parse_str("struct Foo {}").unwrap();
            let d: DeriveInput = s.into();
            assert!(d.is_struct());
        }

        #[test]
        fn from_derive_enum() {
            let e: DeriveEnum = syn::parse_str("enum Foo {}").unwrap();
            let d: DeriveInput = e.into();
            assert!(d.is_enum());
        }

        #[test]
        fn from_derive_union() {
            let u: DeriveUnion = syn::parse_str("union Foo { x: i32 }").unwrap();
            let d: DeriveInput = u.into();
            assert!(d.is_union());
        }
    }

    mod accessors {
        use super::*;

        #[test]
        #[should_panic(expected = "non-Struct")]
        fn as_struct_panics_on_enum() {
            let d: DeriveInput = syn::parse_str("enum Foo {}").unwrap();
            d.as_struct();
        }

        #[test]
        #[should_panic(expected = "non-Enum")]
        fn as_enum_panics_on_struct() {
            let d: DeriveInput = syn::parse_str("struct Foo {}").unwrap();
            d.as_enum();
        }

        #[test]
        #[should_panic(expected = "non-Union")]
        fn as_union_panics_on_struct() {
            let d: DeriveInput = syn::parse_str("struct Foo {}").unwrap();
            d.as_union();
        }
    }
}
