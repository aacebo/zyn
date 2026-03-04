#[test]
fn leading_underscores() {
    let name = zyn::quote::format_ident!("__foo");
    let result = zyn::zyn!({ { name | trim } });
    assert_eq!(result.to_string(), "foo");
}

#[test]
fn trailing_underscores() {
    let name = zyn::quote::format_ident!("bar__");
    let result = zyn::zyn!({ { name | trim } });
    assert_eq!(result.to_string(), "bar");
}

#[test]
fn both_sides() {
    let name = zyn::quote::format_ident!("__baz__");
    let result = zyn::zyn!({ { name | trim } });
    assert_eq!(result.to_string(), "baz");
}

#[test]
fn no_underscores() {
    let name = zyn::quote::format_ident!("hello");
    let result = zyn::zyn!({ { name | trim } });
    assert_eq!(result.to_string(), "hello");
}
