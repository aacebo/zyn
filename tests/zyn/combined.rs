use zyn::quote::quote;

#[test]
fn if_with_pipe_and_braces() {
    let name = zyn::format_ident!("hello_world");
    let is_pub = true;
    let result = zyn::zyn!(
        @if (is_pub) { pub }
        fn {{ name | snake }}() {}
    );
    let expected = quote!(
        pub fn hello_world() {}
    );
    zyn::assert_tokens!(result, expected);
}
