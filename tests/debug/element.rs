use zyn::quote::quote;

#[zyn::element(debug)]
fn greeting_debug(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[zyn::element("debug_alias", debug)]
fn greeting_debug_named(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn element_with_debug() {
    let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
    let result = zyn::zyn!(@greeting_debug(name = zyn::format_ident!("hello")));
    let expected = quote!(
        fn hello() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn element_with_debug_and_name() {
    let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
    let result = zyn::zyn!(@debug_alias(name = zyn::format_ident!("hello")));
    let expected = quote!(
        fn hello() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[cfg(feature = "pretty")]
mod pretty {
    use zyn::quote::quote;

    #[zyn::element(debug = "pretty")]
    fn greeting_pretty(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(fn {{ name }}() {})
    }

    #[zyn::element("pretty_alias", debug = "pretty")]
    fn greeting_pretty_named(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(fn {{ name }}() {})
    }

    #[test]
    fn element_with_pretty() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@greeting_pretty(name = zyn::format_ident!("hello")));
        let expected = quote!(
            fn hello() {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn element_with_pretty_and_name() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@pretty_alias(name = zyn::format_ident!("hello")));
        let expected = quote!(
            fn hello() {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}
