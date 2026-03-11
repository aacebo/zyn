use zyn::quote::quote;

#[test]
fn plain_tokens() {
    let result = zyn::zyn!(
        struct Foo;
    );
    let expected = quote!(
        struct Foo;
    );
    zyn::assert_tokens!(result, expected);
}

#[test]
fn multiple_tokens() {
    let result = zyn::zyn!(let x: i32 = 42;);
    let expected = quote!(let x: i32 = 42;);
    zyn::assert_tokens!(result, expected);
}
