use zyn::quote::quote;

#[test]
fn prefix_pattern() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({{ name | ident:"get_{}" }});
    let expected = quote!(get_hello);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn suffix_pattern() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({{ name | ident:"{}_impl" }});
    let expected = quote!(hello_impl);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn chained_with_case() {
    let name = zyn::format_ident!("HelloWorld");
    let result = zyn::zyn!({{ name | snake | ident:"get_{}" }});
    let expected = quote!(get_hello_world);
    zyn::assert_tokens!(result, expected);
}
