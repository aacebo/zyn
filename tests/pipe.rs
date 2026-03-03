use proc_macro2::TokenStream;
use quote::quote;
use zyn::{Camel, Lower, Pascal, Screaming, Snake, Upper};

mod builtin {
    use super::*;

    #[test]
    fn upper() {
        let name = quote::format_ident!("hello");
        let result: TokenStream = zyn::zyn!({ { name | Upper } });
        let expected = quote!(HELLO);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn lower() {
        let name = quote::format_ident!("HELLO");
        let result: TokenStream = zyn::zyn!({ { name | Lower } });
        let expected = quote!(hello);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn snake() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | Snake } });
        let expected = quote!(hello_world);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn camel() {
        let name = quote::format_ident!("hello_world");
        let result: TokenStream = zyn::zyn!({ { name | Camel } });
        let expected = quote!(helloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn pascal() {
        let name = quote::format_ident!("hello_world");
        let result: TokenStream = zyn::zyn!({ { name | Pascal } });
        let expected = quote!(HelloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn screaming() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | Screaming } });
        let expected = quote!(HELLO_WORLD);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn chained() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | Snake | Upper } });
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
        let result: TokenStream = zyn::zyn!({ { name | Shout } });
        let expected = quote!(HELLO_BANG);
        assert_eq!(result.to_string(), expected.to_string());
    }
}
