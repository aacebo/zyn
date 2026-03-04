use quote::quote;

mod builtin {
    use super::*;

    #[test]
    fn upper() {
        let name = quote::format_ident!("hello");
        let result = zyn::zyn!({ { name | upper } });
        let expected = quote!(HELLO);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn lower() {
        let name = quote::format_ident!("HELLO");
        let result = zyn::zyn!({ { name | lower } });
        let expected = quote!(hello);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn snake() {
        let name = quote::format_ident!("HelloWorld");
        let result = zyn::zyn!({ { name | snake } });
        let expected = quote!(hello_world);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn camel() {
        let name = quote::format_ident!("hello_world");
        let result = zyn::zyn!({ { name | camel } });
        let expected = quote!(helloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn pascal() {
        let name = quote::format_ident!("hello_world");
        let result = zyn::zyn!({ { name | pascal } });
        let expected = quote!(HelloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn screaming() {
        let name = quote::format_ident!("HelloWorld");
        let result = zyn::zyn!({ { name | screaming } });
        let expected = quote!(HELLO_WORLD);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn chained() {
        let name = quote::format_ident!("HelloWorld");
        let result = zyn::zyn!({ { name | snake | upper } });
        let expected = quote!(HELLO_WORLD);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn kebab() {
        let name = quote::format_ident!("HelloWorld");
        let result = zyn::zyn!({ { name | kebab } });
        assert_eq!(result.to_string(), "\"hello-world\"");
    }
}

mod custom {
    use super::*;

    #[zyn::pipe]
    fn shout(input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(
            &format!("{}_BANG", input.to_uppercase()),
            proc_macro2::Span::call_site(),
        )
    }

    #[test]
    fn custom_pipe() {
        let name = quote::format_ident!("hello");
        let result = zyn::zyn!({ { name | shout } });
        let expected = quote!(HELLO_BANG);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[zyn::pipe("yell")]
    fn make_loud(input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(
            &format!("{}__LOUD", input.to_uppercase()),
            proc_macro2::Span::call_site(),
        )
    }

    #[test]
    fn custom_name_override() {
        let name = quote::format_ident!("hello");
        let result = zyn::zyn!({ { name | yell } });
        let expected = quote!(HELLO__LOUD);
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod ident_pipe {
    use super::*;

    #[test]
    fn prefix_pattern() {
        let name = quote::format_ident!("hello");
        let result = zyn::zyn!({{ name | ident:"get_{}" }});
        let expected = quote!(get_hello);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn suffix_pattern() {
        let name = quote::format_ident!("hello");
        let result = zyn::zyn!({{ name | ident:"{}_impl" }});
        let expected = quote!(hello_impl);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn chained_with_case() {
        let name = quote::format_ident!("HelloWorld");
        let result = zyn::zyn!({{ name | snake | ident:"get_{}" }});
        let expected = quote!(get_hello_world);
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod fmt_pipe {
    #[test]
    fn string_format() {
        let name = quote::format_ident!("hello");
        let result = zyn::zyn!({{ name | fmt:"get_{}" }});
        assert_eq!(result.to_string(), "\"get_hello\"");
    }

    #[test]
    fn chained_with_case() {
        let name = quote::format_ident!("HelloWorld");
        let result = zyn::zyn!({{ name | snake | fmt:"{}-component" }});
        assert_eq!(result.to_string(), "\"hello_world-component\"");
    }
}
