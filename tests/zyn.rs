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

    #[test]
    fn else_if_chain() {
        let val = 2;
        let result: TokenStream = zyn::zyn!(
            @if (val == 1) { struct One; }
            @else if (val == 2) { struct Two; }
            @else { struct Other; }
        );
        let expected = quote!(
            struct Two;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn for_empty_iterable() {
        let items: Vec<proc_macro2::Ident> = vec![];
        let result: TokenStream = zyn::zyn!(@for (item of items) { {{ item }} });
        assert!(result.is_empty());
    }

    #[test]
    fn match_multiple_arms() {
        let kind = "enum";
        let result: TokenStream = zyn::zyn!(
            @match (kind) {
                "struct" => { struct Foo; }
                "enum" => { enum Bar {} }
                "union" => { union Baz {} }
                _ => { type Other = (); }
            }
        );
        let expected = quote!(
            enum Bar {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn nested_if_inside_for() {
        let items = vec![
            (quote::format_ident!("a"), true),
            (quote::format_ident!("b"), false),
        ];
        let result: TokenStream = zyn::zyn!(
            @for (item of items) {
                @if (item.1) {
                    fn {{ item.0 }}() {}
                }
            }
        );
        let expected = quote!(
            fn a() {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod interpolation_advanced {
    use super::*;

    struct Field {
        name: proc_macro2::Ident,
        ty: proc_macro2::Ident,
    }

    #[test]
    fn field_access() {
        let field = Field {
            name: quote::format_ident!("age"),
            ty: quote::format_ident!("u32"),
        };
        let result: TokenStream = zyn::zyn!({{ field.name }}: {{ field.ty }});
        let expected = quote!(age: u32);
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
