use zyn::ext::ItemExt;
use zyn::syn;

#[test]
fn is_struct() {
    assert!(zyn::parse!("struct Foo;" => syn::Item).unwrap().is_struct());
    assert!(
        !zyn::parse!("enum Bar { A }" => syn::Item)
            .unwrap()
            .is_struct()
    );
}

#[test]
fn is_enum() {
    assert!(
        zyn::parse!("enum Foo { A, B }" => syn::Item)
            .unwrap()
            .is_enum()
    );
    assert!(!zyn::parse!("struct Bar;" => syn::Item).unwrap().is_enum());
}

#[test]
fn is_fn() {
    assert!(zyn::parse!("fn foo() {}" => syn::Item).unwrap().is_fn());
    assert!(!zyn::parse!("struct Bar;" => syn::Item).unwrap().is_fn());
}

#[test]
fn ident_returns_name() {
    let item: syn::Item = zyn::parse!("struct MyStruct;" => syn::Item).unwrap();
    assert_eq!(item.ident().unwrap().to_string(), "MyStruct");
}

#[test]
fn ident_for_fn() {
    let item: syn::Item = zyn::parse!("fn my_func() {}" => syn::Item).unwrap();
    assert_eq!(item.ident().unwrap().to_string(), "my_func");
}

#[test]
fn attrs_from_item() {
    let item: syn::Item = zyn::parse!("#[derive(Debug)] struct Foo;" => syn::Item).unwrap();
    assert_eq!(item.attrs().len(), 1);
}

#[test]
fn vis_from_pub_struct() {
    let item: syn::Item = zyn::parse!("pub struct Foo;" => syn::Item).unwrap();
    let vis = item.vis().unwrap();
    assert!(matches!(vis, syn::Visibility::Public(_)));
}

#[test]
fn as_struct_converts() {
    let item: syn::Item = zyn::parse!("struct Foo { x: i32 }" => syn::Item).unwrap();
    let s = item.as_struct();
    assert!(s.is_some());
    assert_eq!(s.unwrap().ident.to_string(), "Foo");
}

#[test]
fn as_enum_converts() {
    let item: syn::Item = zyn::parse!("enum Color { Red, Green }" => syn::Item).unwrap();
    let e = item.as_enum();
    assert!(e.is_some());
    assert_eq!(e.unwrap().variants.len(), 2);
}

#[test]
fn generics_from_generic_struct() {
    let item: syn::Item = zyn::parse!("struct Foo<T, U> { x: T, y: U }" => syn::Item).unwrap();
    let generics = item.generics().unwrap();
    assert_eq!(generics.params.len(), 2);
}
