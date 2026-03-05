use proc_macro2::Span;

use super::Input;
use crate::extract::FromInput;

impl FromInput for syn::DeriveInput {
    type Error = syn::Error;

    fn from_input(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Derive(d) => Ok(d.clone()),
            _ => Err(syn::Error::new(Span::call_site(), "expected derive input")),
        }
    }
}

impl FromInput for syn::DataStruct {
    type Error = syn::Error;

    fn from_input(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Derive(d) => match &d.data {
                syn::Data::Struct(s) => Ok(s.clone()),
                _ => Err(syn::Error::new(d.ident.span(), "expected struct")),
            },
            _ => Err(syn::Error::new(
                Span::call_site(),
                "expected derive struct input",
            )),
        }
    }
}

impl FromInput for syn::DataEnum {
    type Error = syn::Error;

    fn from_input(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Derive(d) => match &d.data {
                syn::Data::Enum(e) => Ok(e.clone()),
                _ => Err(syn::Error::new(d.ident.span(), "expected enum")),
            },
            _ => Err(syn::Error::new(
                Span::call_site(),
                "expected derive enum input",
            )),
        }
    }
}

impl FromInput for syn::DataUnion {
    type Error = syn::Error;

    fn from_input(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Derive(d) => match &d.data {
                syn::Data::Union(u) => Ok(u.clone()),
                _ => Err(syn::Error::new(d.ident.span(), "expected union")),
            },
            _ => Err(syn::Error::new(
                Span::call_site(),
                "expected derive union input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_input_from_struct() {
        let input: Input = syn::parse_str("struct Foo { x: u32 }").unwrap();
        let d = syn::DeriveInput::from_input(&input).unwrap();
        assert_eq!(d.ident.to_string(), "Foo");
    }

    #[test]
    fn derive_input_from_item_is_err() {
        let input = Input::Item(syn::parse_str("fn foo() {}").unwrap());
        assert!(syn::DeriveInput::from_input(&input).is_err());
    }

    #[test]
    fn data_struct_from_struct() {
        let input: Input = syn::parse_str("struct Foo { x: u32 }").unwrap();
        let s = syn::DataStruct::from_input(&input).unwrap();
        assert_eq!(s.fields.len(), 1);
    }

    #[test]
    fn data_struct_from_enum_is_err() {
        let input: Input = syn::parse_str("enum Foo { A }").unwrap();
        assert!(syn::DataStruct::from_input(&input).is_err());
    }

    #[test]
    fn data_enum_from_enum() {
        let input: Input = syn::parse_str("enum Dir { North, South }").unwrap();
        let e = syn::DataEnum::from_input(&input).unwrap();
        assert_eq!(e.variants.len(), 2);
    }

    #[test]
    fn data_union_from_union() {
        let input: Input = syn::parse_str("union Bits { i: i32, f: f32 }").unwrap();
        let u = syn::DataUnion::from_input(&input).unwrap();
        assert_eq!(u.fields.named.len(), 2);
    }
}
