use zyn::ext::DataExt;
use zyn::syn;

#[test]
fn is_struct_for_struct() {
    let input: syn::DeriveInput = zyn::parse!("struct Foo;" => syn::DeriveInput).unwrap();
    assert!(input.data.is_struct());
    assert!(!input.data.is_enum());
    assert!(!input.data.is_union());
}

#[test]
fn is_enum_for_enum() {
    let input: syn::DeriveInput = zyn::parse!("enum Foo { A, B }" => syn::DeriveInput).unwrap();
    assert!(input.data.is_enum());
    assert!(!input.data.is_struct());
}

#[test]
fn as_struct_returns_data() {
    let input: syn::DeriveInput = zyn::parse!("struct Foo { x: i32 }" => syn::DeriveInput).unwrap();
    assert!(input.data.as_struct().is_some());
}

#[test]
fn as_enum_returns_data() {
    let input: syn::DeriveInput =
        zyn::parse!("enum Color { Red, Green, Blue }" => syn::DeriveInput).unwrap();
    let data = input.data.as_enum();
    assert!(data.is_some());
    assert_eq!(data.unwrap().variants.len(), 3);
}

#[test]
fn as_struct_none_for_enum() {
    let input: syn::DeriveInput = zyn::parse!("enum Foo { A }" => syn::DeriveInput).unwrap();
    assert!(input.data.as_struct().is_none());
}

#[zyn::element]
pub fn describe_data(data: syn::Data, name: syn::Ident) -> zyn::TokenStream {
    use zyn::ext::DataExt;

    if data.is_struct() {
        zyn::zyn!(
            impl {{ name }} {
                fn kind(&self) -> &str { "struct" }
            }
        )
    } else if data.is_enum() {
        zyn::zyn!(
            impl {{ name }} {
                fn kind(&self) -> &str { "enum" }
            }
        )
    } else {
        zyn::TokenStream::new()
    }
}

#[test]
fn element_renders_struct_kind() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @describe_data(
            data = input.as_derive().unwrap().data.clone(),
            name = input.ident().clone(),
        )
    );
    assert!(output.to_string().contains("struct"), "got: {}", output);
}

#[test]
fn element_renders_enum_kind() {
    let input: zyn::Input = zyn::parse!("enum Bar { A }" => syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @describe_data(
            data = input.as_derive().unwrap().data.clone(),
            name = input.ident().clone(),
        )
    );
    assert!(output.to_string().contains("enum"), "got: {}", output);
}
