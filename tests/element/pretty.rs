use zyn::quote::quote;

#[zyn::element]
fn pretty_struct(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(
        impl {{ name }} {
            fn validate(&self) -> bool { true }
        }
    )
}

#[test]
fn assert_tokens_pretty_multi_line() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let result = zyn::zyn!(@pretty_struct(name = zyn::format_ident!("Foo")));
    let expected = quote!(
        impl Foo {
            fn validate(&self) -> bool { true }
        }
    );
    zyn::assert_tokens_pretty!(result, expected);
}

#[test]
fn assert_tokens_contain_pretty_multi_line() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let result = zyn::zyn!(@pretty_struct(name = zyn::format_ident!("Foo")));
    zyn::assert_tokens_contain_pretty!(result, "fn validate");
}
