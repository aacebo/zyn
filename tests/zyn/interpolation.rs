use zyn::quote::quote;

#[test]
fn simple_variable() {
    let name = zyn::format_ident!("foo");
    let result = zyn::zyn!(fn {{ name }}() {});
    let expected = quote!(
        fn foo() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}
