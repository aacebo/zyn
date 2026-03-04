#[test]
fn from_lower() {
    let name = zyn::quote::format_ident!("foo");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo");
}

#[test]
fn from_snake() {
    let name = zyn::quote::format_ident!("foo_bar");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo_bar");
}

#[test]
fn from_camel() {
    let name = zyn::quote::format_ident!("fooBar");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo_bar");
}

#[test]
fn from_pascal() {
    let name = zyn::quote::format_ident!("FooBar");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo_bar");
}

#[test]
fn from_screaming() {
    let name = zyn::quote::format_ident!("FOO_BAR");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo_bar");
}

#[test]
fn consecutive_uppercase() {
    let name = zyn::quote::format_ident!("HTTPResponse");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "http_response");
}

#[test]
fn consecutive_uppercase_xml() {
    let name = zyn::quote::format_ident!("XMLParser");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "xml_parser");
}

#[test]
fn all_uppercase() {
    let name = zyn::quote::format_ident!("FOO");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo");
}

#[test]
fn single_char() {
    let name = zyn::quote::format_ident!("A");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "a");
}

#[test]
fn trailing_uppercase() {
    let name = zyn::quote::format_ident!("fooBAR");
    let result = zyn::zyn!({ { name | snake } });
    assert_eq!(result.to_string(), "foo_bar");
}
