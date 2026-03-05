use zyn::quote::quote;

#[test]
fn parenthesized() {
    let ty = zyn::format_ident!("i32");
    let result = zyn::zyn!(fn foo(x: {{ ty }}));
    let expected = quote!(fn foo(x: i32));
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn bracketed() {
    let ty = zyn::format_ident!("u8");
    let result = zyn::zyn!(type Foo = [{{ ty }}; 4];);
    let expected = quote!(
        type Foo = [u8; 4];
    );
    assert_eq!(result.to_string(), expected.to_string());
}
