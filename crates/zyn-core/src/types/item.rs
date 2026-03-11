use super::Input;
use crate::extract::FromInput;
use crate::mark;

impl FromInput for syn::Item {
    fn from_input(input: &Input) -> crate::Result<Self> {
        match input {
            Input::Item(v) => Ok(v.clone()),
            _ => Err(mark::error("expected item input")
                .span(input.span())
                .build()),
        }
    }
}

macro_rules! impl_from_input_item {
    ($ty:ty, $variant:ident, $msg:literal) => {
        impl FromInput for $ty {
            fn from_input(input: &Input) -> crate::Result<Self> {
                match input {
                    Input::Item(syn::Item::$variant(v)) => Ok(v.clone()),
                    _ => Err(mark::error($msg).span(input.span()).build()),
                }
            }
        }
    };
}

impl_from_input_item!(syn::ItemConst, Const, "expected const item input");
impl_from_input_item!(
    syn::ItemExternCrate,
    ExternCrate,
    "expected extern crate item input"
);
impl_from_input_item!(syn::ItemFn, Fn, "expected fn item input");
impl_from_input_item!(
    syn::ItemForeignMod,
    ForeignMod,
    "expected foreign mod item input"
);
impl_from_input_item!(syn::ItemImpl, Impl, "expected impl item input");
impl_from_input_item!(syn::ItemMod, Mod, "expected mod item input");
impl_from_input_item!(syn::ItemStatic, Static, "expected static item input");
impl_from_input_item!(syn::ItemTrait, Trait, "expected trait item input");
impl_from_input_item!(syn::ItemType, Type, "expected type item input");
impl_from_input_item!(syn::ItemUse, Use, "expected use item input");

impl FromInput for syn::ItemStruct {
    fn from_input(input: &Input) -> crate::Result<Self> {
        match input {
            Input::Item(syn::Item::Struct(v)) => Ok(v.clone()),
            Input::Derive(d) => match &d.data {
                syn::Data::Struct(s) => Ok(syn::ItemStruct {
                    attrs: d.attrs.clone(),
                    vis: d.vis.clone(),
                    struct_token: syn::Token![struct](d.ident.span()),
                    ident: d.ident.clone(),
                    generics: d.generics.clone(),
                    fields: s.fields.clone(),
                    semi_token: s.semi_token,
                }),
                _ => Err(mark::error("expected struct input")
                    .span(d.ident.span())
                    .build()),
            },
            _ => Err(mark::error("expected struct input")
                .span(input.span())
                .build()),
        }
    }
}

impl FromInput for syn::ItemEnum {
    fn from_input(input: &Input) -> crate::Result<Self> {
        match input {
            Input::Item(syn::Item::Enum(v)) => Ok(v.clone()),
            Input::Derive(d) => match &d.data {
                syn::Data::Enum(e) => Ok(syn::ItemEnum {
                    attrs: d.attrs.clone(),
                    vis: d.vis.clone(),
                    enum_token: syn::Token![enum](d.ident.span()),
                    ident: d.ident.clone(),
                    generics: d.generics.clone(),
                    brace_token: syn::token::Brace::default(),
                    variants: e.variants.clone(),
                }),
                _ => Err(mark::error("expected enum input")
                    .span(d.ident.span())
                    .build()),
            },
            _ => Err(mark::error("expected enum input")
                .span(input.span())
                .build()),
        }
    }
}

impl FromInput for syn::ItemUnion {
    fn from_input(input: &Input) -> crate::Result<Self> {
        match input {
            Input::Item(syn::Item::Union(v)) => Ok(v.clone()),
            Input::Derive(d) => match &d.data {
                syn::Data::Union(u) => Ok(syn::ItemUnion {
                    attrs: d.attrs.clone(),
                    vis: d.vis.clone(),
                    union_token: syn::Token![union](d.ident.span()),
                    ident: d.ident.clone(),
                    generics: d.generics.clone(),
                    fields: u.fields.clone(),
                }),
                _ => Err(mark::error("expected union input")
                    .span(d.ident.span())
                    .build()),
            },
            _ => Err(mark::error("expected union input")
                .span(input.span())
                .build()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_fn_from_item() {
        let input = Input::Item(syn::parse_str("fn hello() {}").unwrap());
        let f = syn::ItemFn::from_input(&input).unwrap();
        assert_eq!(f.sig.ident.to_string(), "hello");
    }

    #[test]
    fn item_fn_from_derive_is_err() {
        let input: Input = syn::parse_str("struct Foo;").unwrap();
        assert!(syn::ItemFn::from_input(&input).is_err());
    }

    #[test]
    fn item_struct_from_item() {
        let input = Input::Item(syn::parse_str("struct Foo { x: u32 }").unwrap());
        let s = syn::ItemStruct::from_input(&input).unwrap();
        assert_eq!(s.ident.to_string(), "Foo");
    }

    #[test]
    fn item_struct_from_derive() {
        let input: Input = syn::parse_str("struct Bar { y: i32 }").unwrap();
        let s = syn::ItemStruct::from_input(&input).unwrap();
        assert_eq!(s.ident.to_string(), "Bar");
    }

    #[test]
    fn item_enum_from_derive() {
        let input: Input = syn::parse_str("enum Dir { North, South }").unwrap();
        let e = syn::ItemEnum::from_input(&input).unwrap();
        assert_eq!(e.ident.to_string(), "Dir");
    }

    #[test]
    fn item_from_item() {
        let input = Input::Item(syn::parse_str("fn bar() {}").unwrap());
        let i = syn::Item::from_input(&input).unwrap();
        assert!(matches!(i, syn::Item::Fn(_)));
    }

    #[test]
    fn item_from_derive_is_err() {
        let input: Input = syn::parse_str("struct Foo;").unwrap();
        assert!(syn::Item::from_input(&input).is_err());
    }
}
