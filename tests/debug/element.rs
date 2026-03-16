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
    zyn::assert_tokens!(result, expected);
}

#[test]
fn element_with_debug_and_name() {
    let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
    let result = zyn::zyn!(@debug_alias(name = zyn::format_ident!("hello")));
    let expected = quote!(
        fn hello() {}
    );
    zyn::assert_tokens!(result, expected);
}

#[cfg(feature = "pretty")]
mod pretty {
    use zyn::quote::quote;

    #[zyn::element(debug(pretty))]
    fn greeting_pretty(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(
            impl {{ name }} {
                fn greet(&self) -> &str {
                    "hello"
                }
            }
        )
    }

    #[zyn::element("pretty_alias", debug(pretty))]
    fn greeting_pretty_named(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(
            impl {{ name }} {
                fn greet(&self) -> &str {
                    "hello"
                }
            }
        )
    }

    #[test]
    fn element_with_pretty() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@greeting_pretty(name = zyn::format_ident!("Foo")));
        let expected = quote!(
            impl Foo {
                fn greet(&self) -> &str {
                    "hello"
                }
            }
        );
        zyn::assert_tokens_pretty!(result, expected);
    }

    #[test]
    fn element_with_pretty_and_name() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@pretty_alias(name = zyn::format_ident!("Foo")));
        let expected = quote!(
            impl Foo {
                fn greet(&self) -> &str {
                    "hello"
                }
            }
        );
        zyn::assert_tokens_pretty!(result, expected);
    }

    #[test]
    fn element_pretty_contain() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@greeting_pretty(name = zyn::format_ident!("Foo")));
        zyn::assert_tokens_contain_pretty!(result, "fn greet");
    }
}

mod inject {
    use zyn::quote::quote;

    #[zyn::element(debug(name = "Foo"))]
    fn greeting_inject_ident(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(fn {{ name }}() {})
    }

    #[zyn::element(debug(ty = "Vec<u8>"))]
    fn greeting_inject_type(ty: zyn::syn::Type) -> zyn::TokenStream {
        zyn::zyn!(fn value(&self) -> {{ ty }} { todo!() })
    }

    #[zyn::element(debug(name = "Foo", ty = "String"))]
    fn greeting_inject_multi(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
        zyn::zyn!(pub fn {{ name }}(&self) -> {{ ty }} { todo!() })
    }

    #[zyn::element(debug(name = "HelloWorld"))]
    fn greeting_inject_piped(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(fn {{ name | snake }}() {})
    }

    #[zyn::element(debug(name = "HelloWorld"))]
    fn greeting_inject_chained(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(fn {{ name | snake | upper }}() {})
    }

    #[test]
    fn element_with_ident_injection() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@greeting_inject_ident(name = zyn::format_ident!("hello")));
        let expected = quote!(
            fn hello() {}
        );
        zyn::assert_tokens!(result, expected);
    }

    #[test]
    fn element_with_type_injection() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let ty: zyn::syn::Type = zyn::syn::parse_str("Vec<u8>").unwrap();
        let result = zyn::zyn!(@greeting_inject_type(ty = ty));
        let expected = quote!(
            fn value(&self) -> Vec<u8> {
                todo!()
            }
        );
        zyn::assert_tokens!(result, expected);
    }

    #[test]
    fn element_with_piped_ident_injection() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@greeting_inject_piped(name = zyn::format_ident!("HelloWorld")));
        let expected = zyn::quote::quote!(
            fn hello_world() {}
        );
        zyn::assert_tokens!(result, expected);
    }

    #[test]
    fn element_with_chained_pipe_injection() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        // "HelloWorld" | snake → "hello_world" | upper → "HELLO_WORLD"
        let result = zyn::zyn!(@greeting_inject_chained(name = zyn::format_ident!("HelloWorld")));
        let expected = zyn::quote::quote!(
            fn HELLO_WORLD() {}
        );
        zyn::assert_tokens!(result, expected);
    }

    #[test]
    fn element_with_multi_injection() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let ty: zyn::syn::Type = zyn::syn::parse_str("String").unwrap();
        let result =
            zyn::zyn!(@greeting_inject_multi(name = zyn::format_ident!("get_value"), ty = ty));
        let expected = quote!(
            pub fn get_value(&self) -> String {
                todo!()
            }
        );
        zyn::assert_tokens!(result, expected);
    }
}

mod full {
    use zyn::quote::quote;

    #[zyn::element(debug(full))]
    fn greeting_full(name: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!(fn {{ name }}() {})
    }

    #[test]
    fn element_with_full() {
        let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
        let result = zyn::zyn!(@greeting_full(name = zyn::format_ident!("hello")));
        let expected = quote!(
            fn hello() {}
        );
        zyn::assert_tokens!(result, expected);
    }

    #[cfg(feature = "pretty")]
    mod pretty {
        use zyn::quote::quote;

        #[zyn::element(debug(pretty, full))]
        fn greeting_full_pretty(name: zyn::syn::Ident) -> zyn::TokenStream {
            zyn::zyn!(fn {{ name }}() {})
        }

        #[test]
        fn element_with_full_pretty() {
            let input: zyn::Input = zyn::syn::parse_str("struct Foo;").unwrap();
            let result = zyn::zyn!(@greeting_full_pretty(name = zyn::format_ident!("hello")));
            let expected = quote!(
                fn hello() {}
            );
            zyn::assert_tokens_pretty!(result, expected);
        }
    }
}
