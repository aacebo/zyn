use zyn::quote::quote;

#[test]
fn upper() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | upper } });
    let expected = quote!(HELLO);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn lower() {
    let name = zyn::format_ident!("HELLO");
    let result = zyn::zyn!({ { name | lower } });
    let expected = quote!(hello);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn chained() {
    let name = zyn::format_ident!("HelloWorld");
    let result = zyn::zyn!({ { name | snake | upper } });
    let expected = quote!(HELLO_WORLD);
    zyn::assert_tokens!(result, expected);
}
