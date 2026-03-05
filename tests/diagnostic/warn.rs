use zyn_core::ast::Element;

#[test]
fn does_not_suppress_output() {
    let result = zyn::parse!("@warn \"test warning\" struct Foo;" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}

#[test]
fn with_note_child_does_not_suppress_output() {
    let result = zyn::parse!(
        "@warn \"deprecated\" { @note \"see migration guide\" } struct Foo;" => Element
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}

#[test]
fn with_help_child_does_not_suppress_output() {
    let result = zyn::parse!(
        "@warn \"deprecated\" { @help \"use new_api() instead\" } struct Foo;" => Element
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}

#[test]
fn with_note_and_help_does_not_suppress_output() {
    let result = zyn::parse!(
        "@warn \"deprecated\" { @note \"removed in v3\" @help \"migrate to new_api()\" } struct Foo;" => Element
    )
    .unwrap()
    .to_token_stream()
    .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}
