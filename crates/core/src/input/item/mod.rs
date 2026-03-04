mod impl_item_fn;
mod item_const;
mod item_enum;
mod item_extern_crate;
mod item_fn;
mod item_foreign_mod;
mod item_impl;
mod item_mod;
mod item_static;
mod item_struct;
mod item_trait;
mod item_type;
mod item_union;
mod item_use;
mod trait_item_fn;

pub use impl_item_fn::*;
pub use item_const::*;
pub use item_enum::*;
pub use item_extern_crate::*;
pub use item_fn::*;
pub use item_foreign_mod::*;
pub use item_impl::*;
pub use item_mod::*;
pub use item_static::*;
pub use item_struct::*;
pub use item_trait::*;
pub use item_type::*;
pub use item_union::*;
pub use item_use::*;
pub use trait_item_fn::*;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::discouraged::Speculative;

pub enum ItemInput {
    Struct(ItemStruct),
    Enum(ItemEnum),
    Union(ItemUnion),
    Fn(ItemFn),
    Impl(ItemImpl),
    Trait(ItemTrait),
    Type(ItemType),
    Mod(ItemMod),
    Const(ItemConst),
    Static(ItemStatic),
    Use(ItemUse),
    ExternCrate(ItemExternCrate),
    ForeignMod(ItemForeignMod),
    ImplFn(ImplItemFn),
    TraitFn(TraitItemFn),
}

impl ItemInput {
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }
    pub fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }
    pub fn is_fn(&self) -> bool {
        matches!(self, Self::Fn(_))
    }
    pub fn is_impl(&self) -> bool {
        matches!(self, Self::Impl(_))
    }
    pub fn is_trait(&self) -> bool {
        matches!(self, Self::Trait(_))
    }
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }
    pub fn is_mod(&self) -> bool {
        matches!(self, Self::Mod(_))
    }
    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }
    pub fn is_use(&self) -> bool {
        matches!(self, Self::Use(_))
    }
    pub fn is_extern_crate(&self) -> bool {
        matches!(self, Self::ExternCrate(_))
    }
    pub fn is_foreign_mod(&self) -> bool {
        matches!(self, Self::ForeignMod(_))
    }
    pub fn is_impl_fn(&self) -> bool {
        matches!(self, Self::ImplFn(_))
    }
    pub fn is_trait_fn(&self) -> bool {
        matches!(self, Self::TraitFn(_))
    }

    pub fn as_struct(&self) -> &ItemStruct {
        match self {
            Self::Struct(s) => s,
            _ => panic!("called `ItemInput::as_struct()` on a non-Struct variant"),
        }
    }

    pub fn as_enum(&self) -> &ItemEnum {
        match self {
            Self::Enum(e) => e,
            _ => panic!("called `ItemInput::as_enum()` on a non-Enum variant"),
        }
    }

    pub fn as_union(&self) -> &ItemUnion {
        match self {
            Self::Union(u) => u,
            _ => panic!("called `ItemInput::as_union()` on a non-Union variant"),
        }
    }

    pub fn as_fn(&self) -> &ItemFn {
        match self {
            Self::Fn(f) => f,
            _ => panic!("called `ItemInput::as_fn()` on a non-Fn variant"),
        }
    }

    pub fn as_impl(&self) -> &ItemImpl {
        match self {
            Self::Impl(i) => i,
            _ => panic!("called `ItemInput::as_impl()` on a non-Impl variant"),
        }
    }

    pub fn as_trait(&self) -> &ItemTrait {
        match self {
            Self::Trait(t) => t,
            _ => panic!("called `ItemInput::as_trait()` on a non-Trait variant"),
        }
    }

    pub fn as_type(&self) -> &ItemType {
        match self {
            Self::Type(t) => t,
            _ => panic!("called `ItemInput::as_type()` on a non-Type variant"),
        }
    }

    pub fn as_mod(&self) -> &ItemMod {
        match self {
            Self::Mod(m) => m,
            _ => panic!("called `ItemInput::as_mod()` on a non-Mod variant"),
        }
    }

    pub fn as_const(&self) -> &ItemConst {
        match self {
            Self::Const(c) => c,
            _ => panic!("called `ItemInput::as_const()` on a non-Const variant"),
        }
    }

    pub fn as_static(&self) -> &ItemStatic {
        match self {
            Self::Static(s) => s,
            _ => panic!("called `ItemInput::as_static()` on a non-Static variant"),
        }
    }

    pub fn as_use(&self) -> &ItemUse {
        match self {
            Self::Use(u) => u,
            _ => panic!("called `ItemInput::as_use()` on a non-Use variant"),
        }
    }

    pub fn as_extern_crate(&self) -> &ItemExternCrate {
        match self {
            Self::ExternCrate(e) => e,
            _ => panic!("called `ItemInput::as_extern_crate()` on a non-ExternCrate variant"),
        }
    }

    pub fn as_foreign_mod(&self) -> &ItemForeignMod {
        match self {
            Self::ForeignMod(f) => f,
            _ => panic!("called `ItemInput::as_foreign_mod()` on a non-ForeignMod variant"),
        }
    }

    pub fn as_impl_fn(&self) -> &ImplItemFn {
        match self {
            Self::ImplFn(f) => f,
            _ => panic!("called `ItemInput::as_impl_fn()` on a non-ImplFn variant"),
        }
    }

    pub fn as_trait_fn(&self) -> &TraitItemFn {
        match self {
            Self::TraitFn(f) => f,
            _ => panic!("called `ItemInput::as_trait_fn()` on a non-TraitFn variant"),
        }
    }
}

impl Parse for ItemInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(item) = fork.parse::<syn::Item>() {
            let result = match item {
                syn::Item::Struct(s) => Some(Self::Struct(ItemStruct(s))),
                syn::Item::Enum(e) => Some(Self::Enum(ItemEnum(e))),
                syn::Item::Union(u) => Some(Self::Union(ItemUnion(u))),
                syn::Item::Fn(f) => Some(Self::Fn(ItemFn(f))),
                syn::Item::Impl(i) => Some(Self::Impl(ItemImpl(i))),
                syn::Item::Trait(t) => Some(Self::Trait(ItemTrait(t))),
                syn::Item::Type(t) => Some(Self::Type(ItemType(t))),
                syn::Item::Mod(m) => Some(Self::Mod(ItemMod(m))),
                syn::Item::Const(c) => Some(Self::Const(ItemConst(c))),
                syn::Item::Static(s) => Some(Self::Static(ItemStatic(s))),
                syn::Item::Use(u) => Some(Self::Use(ItemUse(u))),
                syn::Item::ExternCrate(e) => Some(Self::ExternCrate(ItemExternCrate(e))),
                syn::Item::ForeignMod(f) => Some(Self::ForeignMod(ItemForeignMod(f))),
                _ => None,
            };
            if let Some(r) = result {
                input.advance_to(&fork);
                return Ok(r);
            }
        }

        let fork = input.fork();
        if let Ok(f) = fork.parse::<syn::ImplItemFn>() {
            input.advance_to(&fork);
            return Ok(Self::ImplFn(ImplItemFn(f)));
        }

        let fork = input.fork();
        if let Ok(f) = fork.parse::<syn::TraitItemFn>() {
            input.advance_to(&fork);
            return Ok(Self::TraitFn(TraitItemFn(f)));
        }

        Err(input.error("expected a supported item"))
    }
}

impl ToTokens for ItemInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(s) => s.to_tokens(tokens),
            Self::Enum(e) => e.to_tokens(tokens),
            Self::Union(u) => u.to_tokens(tokens),
            Self::Fn(f) => f.to_tokens(tokens),
            Self::Impl(i) => i.to_tokens(tokens),
            Self::Trait(t) => t.to_tokens(tokens),
            Self::Type(t) => t.to_tokens(tokens),
            Self::Mod(m) => m.to_tokens(tokens),
            Self::Const(c) => c.to_tokens(tokens),
            Self::Static(s) => s.to_tokens(tokens),
            Self::Use(u) => u.to_tokens(tokens),
            Self::ExternCrate(e) => e.to_tokens(tokens),
            Self::ForeignMod(f) => f.to_tokens(tokens),
            Self::ImplFn(f) => f.to_tokens(tokens),
            Self::TraitFn(f) => f.to_tokens(tokens),
        }
    }
}

impl From<ItemStruct> for ItemInput {
    fn from(v: ItemStruct) -> Self {
        Self::Struct(v)
    }
}

impl From<ItemEnum> for ItemInput {
    fn from(v: ItemEnum) -> Self {
        Self::Enum(v)
    }
}

impl From<ItemUnion> for ItemInput {
    fn from(v: ItemUnion) -> Self {
        Self::Union(v)
    }
}

impl From<ItemFn> for ItemInput {
    fn from(v: ItemFn) -> Self {
        Self::Fn(v)
    }
}

impl From<ItemImpl> for ItemInput {
    fn from(v: ItemImpl) -> Self {
        Self::Impl(v)
    }
}

impl From<ItemTrait> for ItemInput {
    fn from(v: ItemTrait) -> Self {
        Self::Trait(v)
    }
}

impl From<ItemType> for ItemInput {
    fn from(v: ItemType) -> Self {
        Self::Type(v)
    }
}

impl From<ItemMod> for ItemInput {
    fn from(v: ItemMod) -> Self {
        Self::Mod(v)
    }
}

impl From<ItemConst> for ItemInput {
    fn from(v: ItemConst) -> Self {
        Self::Const(v)
    }
}

impl From<ItemStatic> for ItemInput {
    fn from(v: ItemStatic) -> Self {
        Self::Static(v)
    }
}

impl From<ItemUse> for ItemInput {
    fn from(v: ItemUse) -> Self {
        Self::Use(v)
    }
}

impl From<ItemExternCrate> for ItemInput {
    fn from(v: ItemExternCrate) -> Self {
        Self::ExternCrate(v)
    }
}

impl From<ItemForeignMod> for ItemInput {
    fn from(v: ItemForeignMod) -> Self {
        Self::ForeignMod(v)
    }
}

impl From<ImplItemFn> for ItemInput {
    fn from(v: ImplItemFn) -> Self {
        Self::ImplFn(v)
    }
}

impl From<TraitItemFn> for ItemInput {
    fn from(v: TraitItemFn) -> Self {
        Self::TraitFn(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn struct_variant() {
            let i: ItemInput = syn::parse_str("struct Foo { x: u32 }").unwrap();
            assert!(i.is_struct());
        }

        #[test]
        fn enum_variant() {
            let i: ItemInput = syn::parse_str("enum Bar { A, B }").unwrap();
            assert!(i.is_enum());
        }

        #[test]
        fn union_variant() {
            let i: ItemInput = syn::parse_str("union Bits { x: i32, y: f32 }").unwrap();
            assert!(i.is_union());
        }

        #[test]
        fn fn_variant() {
            let i: ItemInput = syn::parse_str("fn foo() {}").unwrap();
            assert!(i.is_fn());
        }

        #[test]
        fn impl_variant() {
            let i: ItemInput = syn::parse_str("impl Foo { fn bar(&self) {} }").unwrap();
            assert!(i.is_impl());
        }

        #[test]
        fn trait_variant() {
            let i: ItemInput = syn::parse_str("trait Greet { fn hello(&self); }").unwrap();
            assert!(i.is_trait());
        }

        #[test]
        fn type_variant() {
            let i: ItemInput = syn::parse_str("type Meters = f64;").unwrap();
            assert!(i.is_type());
        }

        #[test]
        fn mod_variant() {
            let i: ItemInput = syn::parse_str("mod utils {}").unwrap();
            assert!(i.is_mod());
        }

        #[test]
        fn const_variant() {
            let i: ItemInput = syn::parse_str("const MAX: u32 = 100;").unwrap();
            assert!(i.is_const());
        }

        #[test]
        fn static_variant() {
            let i: ItemInput = syn::parse_str("static GREETING: &str = \"hello\";").unwrap();
            assert!(i.is_static());
        }

        #[test]
        fn use_variant() {
            let i: ItemInput = syn::parse_str("use std::collections::HashMap;").unwrap();
            assert!(i.is_use());
        }

        #[test]
        fn extern_crate_variant() {
            let i: ItemInput = syn::parse_str("extern crate serde;").unwrap();
            assert!(i.is_extern_crate());
        }

        #[test]
        fn foreign_mod_variant() {
            let i: ItemInput = syn::parse_str("extern \"C\" { fn abs(x: i32) -> i32; }").unwrap();
            assert!(i.is_foreign_mod());
        }

        #[test]
        fn impl_fn_falls_through_to_fn() {
            let i: ItemInput = syn::parse_str("fn bar(&self) -> u32 { 42 }").unwrap();
            assert!(i.is_fn());
        }

        #[test]
        fn trait_fn_variant() {
            let i: ItemInput = syn::parse_str("fn greet(&self);").unwrap();
            assert!(i.is_trait_fn());
        }
    }

    mod from {
        use super::*;

        #[test]
        fn from_item_struct() {
            let s: ItemStruct = syn::parse_str("struct Foo {}").unwrap();
            let i: ItemInput = s.into();
            assert!(i.is_struct());
        }

        #[test]
        fn from_item_fn() {
            let f: ItemFn = syn::parse_str("fn foo() {}").unwrap();
            let i: ItemInput = f.into();
            assert!(i.is_fn());
        }
    }
}
