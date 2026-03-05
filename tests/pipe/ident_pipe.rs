use zyn::quote::quote;

#[test]
fn prefix_pattern() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({{ name | ident:"get_{}" }});
    let expected = quote!(get_hello);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn suffix_pattern() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({{ name | ident:"{}_impl" }});
    let expected = quote!(hello_impl);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn chained_with_case() {
    let name = zyn::format_ident!("HelloWorld");
    let result = zyn::zyn!({{ name | snake | ident:"get_{}" }});
    let expected = quote!(get_hello_world);
    assert_eq!(result.to_string(), expected.to_string());
}
