use zyn_core::ast::Element;

#[test]
fn does_not_suppress_output() {
    let result = zyn::parse!("@note \"additional context\" struct Foo;" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    assert!(
        result.contains("Foo"),
        "struct Foo should still be emitted, got: {result}"
    );
}
