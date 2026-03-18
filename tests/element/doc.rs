use zyn::quote::quote;

/// A documented element.
#[zyn::element]
fn documented() -> zyn::TokenStream {
    zyn::zyn!(
        struct Foo;
    )
}

/// Multi-line doc comment.
///
/// Second paragraph.
#[zyn::element]
fn documented_multi(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn doc_comment_element_compiles() {
    let input: zyn::Input = zyn::syn::parse_str("struct Bar;").unwrap();
    let result = zyn::zyn!(@documented);
    let expected = quote!(
        struct Foo;
    );
    zyn::assert_tokens!(result, expected);
}

#[test]
fn multi_line_doc_comment_element_compiles() {
    let input: zyn::Input = zyn::syn::parse_str("struct Bar;").unwrap();
    let result = zyn::zyn!(@documented_multi(name = zyn::format_ident!("hello")));
    let expected = quote!(
        fn hello() {}
    );
    zyn::assert_tokens!(result, expected);
}
