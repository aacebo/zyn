#[test]
fn converts_to_string_literal() {
    let name = zyn::quote::format_ident!("hello_world");
    let result = zyn::zyn!({ { name | str } });
    assert_eq!(result.to_string(), "\"hello_world\"");
}

#[test]
fn from_pascal() {
    let name = zyn::quote::format_ident!("HelloWorld");
    let result = zyn::zyn!({ { name | str } });
    assert_eq!(result.to_string(), "\"HelloWorld\"");
}
