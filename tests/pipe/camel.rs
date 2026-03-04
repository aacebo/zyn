#[test]
fn from_lower() {
    let name = zyn::quote::format_ident!("foo");
    let result = zyn::zyn!({ { name | camel } });
    assert_eq!(result.to_string(), "foo");
}

#[test]
fn from_snake() {
    let name = zyn::quote::format_ident!("foo_bar");
    let result = zyn::zyn!({ { name | camel } });
    assert_eq!(result.to_string(), "fooBar");
}

#[test]
fn from_pascal() {
    let name = zyn::quote::format_ident!("FooBar");
    let result = zyn::zyn!({ { name | camel } });
    assert_eq!(result.to_string(), "fooBar");
}

#[test]
fn from_camel() {
    let name = zyn::quote::format_ident!("fooBar");
    let result = zyn::zyn!({ { name | camel } });
    assert_eq!(result.to_string(), "fooBar");
}

#[test]
fn from_screaming() {
    let name = zyn::quote::format_ident!("FOO_BAR");
    let result = zyn::zyn!({ { name | camel } });
    assert_eq!(result.to_string(), "fooBar");
}

#[test]
fn single_char() {
    let name = zyn::quote::format_ident!("A");
    let result = zyn::zyn!({ { name | camel } });
    assert_eq!(result.to_string(), "a");
}
