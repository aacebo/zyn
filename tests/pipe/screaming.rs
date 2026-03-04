#[test]
fn from_lower() {
    let name = zyn::quote::format_ident!("foo");
    let result = zyn::zyn!({ { name | screaming } });
    assert_eq!(result.to_string(), "FOO");
}

#[test]
fn from_snake() {
    let name = zyn::quote::format_ident!("foo_bar");
    let result = zyn::zyn!({ { name | screaming } });
    assert_eq!(result.to_string(), "FOO_BAR");
}

#[test]
fn from_camel() {
    let name = zyn::quote::format_ident!("fooBar");
    let result = zyn::zyn!({ { name | screaming } });
    assert_eq!(result.to_string(), "FOO_BAR");
}

#[test]
fn from_pascal() {
    let name = zyn::quote::format_ident!("FooBar");
    let result = zyn::zyn!({ { name | screaming } });
    assert_eq!(result.to_string(), "FOO_BAR");
}

#[test]
fn consecutive_uppercase() {
    let name = zyn::quote::format_ident!("HTTPResponse");
    let result = zyn::zyn!({ { name | screaming } });
    assert_eq!(result.to_string(), "HTTP_RESPONSE");
}
