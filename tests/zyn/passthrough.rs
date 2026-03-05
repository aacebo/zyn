use zyn::quote::quote;

#[test]
fn plain_tokens() {
    let result = zyn::zyn!(
        struct Foo;
    );
    let expected = quote!(
        struct Foo;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn multiple_tokens() {
    let result = zyn::zyn!(let x: i32 = 42;);
    let expected = quote!(let x: i32 = 42;);
    assert_eq!(result.to_string(), expected.to_string());
}
