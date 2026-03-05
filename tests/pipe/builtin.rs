use zyn::quote::quote;

#[test]
fn upper() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | upper } });
    let expected = quote!(HELLO);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn lower() {
    let name = zyn::format_ident!("HELLO");
    let result = zyn::zyn!({ { name | lower } });
    let expected = quote!(hello);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn chained() {
    let name = zyn::format_ident!("HelloWorld");
    let result = zyn::zyn!({ { name | snake | upper } });
    let expected = quote!(HELLO_WORLD);
    assert_eq!(result.to_string(), expected.to_string());
}
