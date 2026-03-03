use proc_macro2::TokenStream;
use quote::quote;

mod passthrough {
    use super::*;

    #[test]
    fn plain_tokens() {
        let result: TokenStream = zyn::zyn!(
            struct Foo;
        );
        let expected = quote!(
            struct Foo;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn multiple_tokens() {
        let result: TokenStream = zyn::zyn!(let x: i32 = 42;);
        let expected = quote!(let x: i32 = 42;);
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod interpolation {
    use super::*;

    #[test]
    fn simple_variable() {
        let name = quote::format_ident!("foo");
        let result: TokenStream = zyn::zyn!(fn {{ name }}() {});
        let expected = quote!(
            fn foo() {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod pipes {
    use super::*;
    use zyn::{Camel, Lower, Pascal, Screaming, Snake, Upper};

    #[test]
    fn upper_pipe() {
        let name = quote::format_ident!("hello");
        let result: TokenStream = zyn::zyn!({ { name | Upper } });
        let expected = quote!(HELLO);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn lower_pipe() {
        let name = quote::format_ident!("HELLO");
        let result: TokenStream = zyn::zyn!({ { name | Lower } });
        let expected = quote!(hello);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn snake_pipe() {
        let name = quote::format_ident!("HelloWorld");
        let result: TokenStream = zyn::zyn!({ { name | Snake } });
        let expected = quote!(hello_world);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn camel_pipe() {
        let name = quote::format_ident!("hello_world");
        let result: TokenStream = zyn::zyn!({ { name | Camel } });
        let expected = quote!(helloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn pascal_pipe() {
        let name = quote::format_ident!("hello_world");
        let result: TokenStream = zyn::zyn!({ { name | Pascal } });
        let expected = quote!(HelloWorld);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn screaming_pipe() {
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

mod control_flow {
    use super::*;

    #[test]
    fn if_true() {
        let flag = true;
        let result: TokenStream = zyn::zyn!(@if (flag) { struct Foo; });
        let expected = quote!(
            struct Foo;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn if_false() {
        let flag = false;
        let result: TokenStream = zyn::zyn!(@if (flag) { struct Foo; });
        assert!(result.is_empty());
    }

    #[test]
    fn if_else() {
        let flag = false;
        let result: TokenStream = zyn::zyn!(
            @if (flag) { struct Foo; }
            @else { struct Bar; }
        );
        let expected = quote!(
            struct Bar;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn for_loop() {
        let names = vec![quote::format_ident!("a"), quote::format_ident!("b")];
        let result: TokenStream = zyn::zyn!(@for (name of names) { {{ name }}, });
        let expected = quote!(a, b,);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn match_directive() {
        let kind = 1;
        let result: TokenStream = zyn::zyn!(
            @match (kind) {
                1 => { struct Foo; }
                _ => { struct Bar; }
            }
        );
        let expected = quote!(
            struct Foo;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod groups {
    use super::*;

    #[test]
    fn parenthesized() {
        let ty = quote::format_ident!("i32");
        let result: TokenStream = zyn::zyn!(fn foo(x: {{ ty }}));
        let expected = quote!(fn foo(x: i32));
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn bracketed() {
        let ty = quote::format_ident!("u8");
        let result: TokenStream = zyn::zyn!(type Foo = [{{ ty }}; 4];);
        let expected = quote!(
            type Foo = [u8; 4];
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}
