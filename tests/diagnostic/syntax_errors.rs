use zyn_core::ast::Element;

macro_rules! expect_err {
    ($s:literal) => {
        match zyn::parse!($s => Element) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("expected parse error for: {}", $s),
        }
    };
}

#[test]
fn empty_interpolation() {
    let msg = expect_err!("{{ }}");
    assert!(msg.contains("empty interpolation"), "got: {msg}");
}

#[test]
fn throw_missing_message() {
    let msg = expect_err!("@throw");
    assert!(msg.contains("expected string literal"), "got: {msg}");
}

#[test]
fn warn_missing_message() {
    let msg = expect_err!("@warn");
    assert!(msg.contains("expected string literal"), "got: {msg}");
}

#[test]
fn note_missing_message() {
    let msg = expect_err!("@note");
    assert!(msg.contains("expected string literal"), "got: {msg}");
}

#[test]
fn help_missing_message() {
    let msg = expect_err!("@help");
    assert!(msg.contains("expected string literal"), "got: {msg}");
}

#[test]
fn throw_non_string_message() {
    let msg = expect_err!("@throw 42");
    assert!(msg.contains("expected string literal"), "got: {msg}");
}

#[test]
fn else_without_if() {
    let msg = expect_err!("@else { foo }");
    assert!(msg.contains("unexpected @else without @if"), "got: {msg}");
}

#[test]
fn element_no_parens() {
    assert!(zyn::parse!("@my_element" => Element).is_ok());
}

#[test]
fn element_empty_parens() {
    assert!(zyn::parse!("@my_element()" => Element).is_ok());
}

#[test]
fn invalid_child_in_throw_body() {
    let msg = expect_err!("@throw \"msg\" { @if (x) { } }");
    assert!(msg.contains("expected `note` or `help`"), "got: {msg}");
}
