use zyn::ext::TypeExt;
use zyn::syn;

#[test]
fn is_option() {
    assert!(
        zyn::parse!("Option<String>" => syn::Type)
            .unwrap()
            .is_option()
    );
    assert!(!zyn::parse!("String" => syn::Type).unwrap().is_option());
    assert!(
        !zyn::parse!("Result<String, Error>" => syn::Type)
            .unwrap()
            .is_option()
    );
}

#[test]
fn is_result() {
    assert!(
        zyn::parse!("Result<String, Error>" => syn::Type)
            .unwrap()
            .is_result()
    );
    assert!(!zyn::parse!("String" => syn::Type).unwrap().is_result());
    assert!(
        !zyn::parse!("Option<String>" => syn::Type)
            .unwrap()
            .is_result()
    );
}

#[test]
fn inner_type_extracts_from_option() {
    let ty: syn::Type = zyn::parse!("Option<String>" => syn::Type).unwrap();
    let inner = ty.inner_type().unwrap();
    let inner_str = quote::quote!(#inner).to_string();
    assert_eq!(inner_str, "String");
}

#[test]
fn inner_type_extracts_from_result() {
    let ty: syn::Type = zyn::parse!("Result<u32, Error>" => syn::Type).unwrap();
    let inner = ty.inner_type().unwrap();
    let inner_str = quote::quote!(#inner).to_string();
    assert_eq!(inner_str, "u32");
}

#[test]
fn inner_type_none_for_plain() {
    assert!(
        zyn::parse!("String" => syn::Type)
            .unwrap()
            .inner_type()
            .is_none()
    );
}

#[test]
fn as_path_returns_path() {
    assert!(
        zyn::parse!("std::string::String" => syn::Type)
            .unwrap()
            .as_path()
            .is_some()
    );
}

#[test]
fn field_is_option() {
    let input: syn::DeriveInput =
        zyn::parse!("struct Foo { name: Option<String>, age: u32 }" => syn::DeriveInput).unwrap();
    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("expected struct"),
    };

    let name_field = fields.iter().next().unwrap();
    let age_field = fields.iter().nth(1).unwrap();
    assert!(name_field.is_option());
    assert!(!age_field.is_option());
}
