#[test]
fn leading_underscores() {
    let name = zyn::format_ident!("__foo");
    let result = zyn::zyn!({{ name | trim:"_" }});
    assert_eq!(result.to_string(), "foo");
}

#[test]
fn trailing_underscores() {
    let name = zyn::format_ident!("bar__");
    let result = zyn::zyn!({{ name | trim:"_" }});
    assert_eq!(result.to_string(), "bar");
}

#[test]
fn both_sides() {
    let name = zyn::format_ident!("__baz__");
    let result = zyn::zyn!({{ name | trim:"_" }});
    assert_eq!(result.to_string(), "baz");
}

#[test]
fn no_underscores() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({{ name | trim:"_" }});
    assert_eq!(result.to_string(), "hello");
}

#[test]
fn default_args_trims_nothing_on_ident() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | trim } });
    assert_eq!(result.to_string(), "hello");
}

#[test]
fn two_arg_different_sides() {
    let name = zyn::format_ident!("xhellox");
    let result = zyn::zyn!({{ name | trim:"x":"x" }});
    assert_eq!(result.to_string(), "hello");
}

#[test]
fn two_arg_asymmetric() {
    let name = zyn::format_ident!("xhelloy");
    let result = zyn::zyn!({{ name | trim:"x":"y" }});
    assert_eq!(result.to_string(), "hello");
}
