use zyn_core::ast::Element;

#[test]
fn does_not_suppress_output() {
    let result = zyn::parse!("@help \"try this instead\" struct Foo;" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}
