#[test]
fn string_format() {
    let name = zyn::quote::format_ident!("hello");
    let result = zyn::zyn!({{ name | fmt:"get_{}" }});
    assert_eq!(result.to_string(), "\"get_hello\"");
}

#[test]
fn chained_with_case() {
    let name = zyn::quote::format_ident!("HelloWorld");
    let result = zyn::zyn!({{ name | snake | fmt:"{}-component" }});
    assert_eq!(result.to_string(), "\"hello_world-component\"");
}
