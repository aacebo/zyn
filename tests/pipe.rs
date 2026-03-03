use proc_macro2::TokenStream;
use quote::quote;
use zyn::{Camel, Lower, Pascal, Screaming, Snake, Upper};

mod builtin {
    use super::*;

    #[test]
    fn upper() {
        let name = quote::format_ident!("hello");
        let result: TokenStream = zyn::zyn!({ { name | upper } });
        let expected = quote!(HELLO);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn lower() {
        let name = quote::format_ident!("HELLO");
        let result: TokenStream = zyn::zyn!({ { name | lower } });
        let expected = quote!(hello);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn snake() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | snake } });
        let expected = quote!(hello_world);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn camel() {
        let name = quote::format_ident!("hello_world");
        let result: TokenStream = zyn::zyn!({ { name | camel } });
        let expected = quote!(helloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn pascal() {
        let name = quote::format_ident!("hello_world");
        let result: TokenStream = zyn::zyn!({ { name | pascal } });
        let expected = quote!(HelloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn screaming() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | screaming } });
        let expected = quote!(HELLO_WORLD);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn chained() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | snake | upper } });
        let expected = quote!(HELLO_WORLD);
        assert_eq!(result.to_string(), expected.to_string());
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
        let result: TokenStream = zyn::zyn!({ { name | shout } });
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
        let result: TokenStream = zyn::zyn!({ { name | yell } });
        let expected = quote!(HELLO__LOUD);
        assert_eq!(result.to_string(), expected.to_string());
    }
}
