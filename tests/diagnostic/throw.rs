use zyn_core::ast::Element;

#[test]
fn generates_compile_error() {
    let result = zyn::parse!("@throw \"bad input\"" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    assert!(
        result.contains("compile_error"),
        "expected compile_error!, got: {result}"
    );
    assert!(
        result.contains("bad input"),
        "expected message, got: {result}"
    );
}

#[test]
fn with_note_and_help() {
    let result = zyn::parse!(
        "@throw \"invalid name\" { @note \"must be lowercase\" @help \"try `foo_bar`\" }" => Element
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(result.contains("compile_error"), "expected compile_error");
    assert!(result.contains("invalid name"), "expected primary message");
    assert!(result.contains("must be lowercase"), "expected note text");
    assert!(result.contains("try `foo_bar`"), "expected help text");
}

#[test]
fn with_note_only() {
    let result =
        zyn::parse!("@throw \"bad value\" { @note \"expected a positive integer\" }" => Element)
            .unwrap()
            .to_token_stream()
            .to_string();
    assert!(result.contains("compile_error"), "expected compile_error");
    assert!(result.contains("bad value"), "expected primary message");
    assert!(
        result.contains("expected a positive integer"),
        "expected note text"
    );
}

#[test]
fn with_help_only() {
    let result =
        zyn::parse!("@throw \"missing field\" { @help \"add a `name` field\" }" => Element)
            .unwrap()
            .to_token_stream()
            .to_string();
    assert!(result.contains("compile_error"), "expected compile_error");
    assert!(result.contains("missing field"), "expected primary message");
    assert!(result.contains("add a `name` field"), "expected help text");
}
