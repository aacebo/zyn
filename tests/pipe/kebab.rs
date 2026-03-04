#[test]
fn from_pascal() {
    let name = zyn::quote::format_ident!("FooBar");
    let result = zyn::zyn!({ { name | kebab } });
    assert_eq!(result.to_string(), "\"foo-bar\"");
}

#[test]
fn from_snake() {
    let name = zyn::quote::format_ident!("foo_bar");
    let result = zyn::zyn!({ { name | kebab } });
    assert_eq!(result.to_string(), "\"foo-bar\"");
}

#[test]
fn from_screaming() {
    let name = zyn::quote::format_ident!("FOO_BAR");
    let result = zyn::zyn!({ { name | kebab } });
    assert_eq!(result.to_string(), "\"foo-bar\"");
}

#[test]
fn consecutive_uppercase() {
    let name = zyn::quote::format_ident!("HTTPResponse");
    let result = zyn::zyn!({ { name | kebab } });
    assert_eq!(result.to_string(), "\"http-response\"");
}
