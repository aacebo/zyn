use quote::quote;

mod passthrough {
    use super::*;

    #[test]
    fn plain_tokens() {
        let result = zyn::zyn!(
            struct Foo;
        );
        let expected = quote!(
            struct Foo;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn multiple_tokens() {
        let result = zyn::zyn!(let x: i32 = 42;);
        let expected = quote!(let x: i32 = 42;);
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod interpolation {
    use super::*;

    #[test]
    fn simple_variable() {
        let name = quote::format_ident!("foo");
        let result = zyn::zyn!(fn {{ name }}() {});
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
        let result = zyn::zyn!(@if (flag) { struct Foo; });
        let expected = quote!(
            struct Foo;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn if_false() {
        let flag = false;
        let result = zyn::zyn!(@if (flag) { struct Foo; });
        assert!(result.is_empty());
    }

    #[test]
    fn if_else() {
        let flag = false;
        let result = zyn::zyn!(
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
        let result = zyn::zyn!(@for (name of names) { {{ name }}, });
        let expected = quote!(a, b,);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn match_directive() {
        let kind = 1;
        let result = zyn::zyn!(
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
        let result = zyn::zyn!(
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
        let result = zyn::zyn!(@for (item of items) { {{ item }} });
        assert!(result.is_empty());
    }

    #[test]
    fn for_inline_iterator() {
        let result = zyn::zyn!(
            @for (name of ["x", "y", "z"].map(|s| quote::format_ident!("{}", s))) {
                pub {{ name }}: f64,
            }
        );
        let expected = quote!(pub x: f64, pub y: f64, pub z: f64,);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn match_multiple_arms() {
        let kind = "enum";
        let result = zyn::zyn!(
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
    fn match_wildcard_only() {
        let kind = "anything";
        let result = zyn::zyn!(
            @match (kind) {
                _ => { struct Fallback; }
            }
        );
        let expected = quote!(
            struct Fallback;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn match_wildcard_catches() {
        let kind = 99;
        let result = zyn::zyn!(
            @match (kind) {
                1 => { struct One; }
                2 => { struct Two; }
                _ => { struct Other; }
            }
        );
        let expected = quote!(
            struct Other;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn nested_if_inside_for() {
        let items = vec![
            (quote::format_ident!("a"), true),
            (quote::format_ident!("b"), false),
        ];
        let result = zyn::zyn!(
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

    #[test]
    fn nested_field_in_condition() {
        struct Opts {
            is_pub: bool,
        }
        let opts = Opts { is_pub: true };
        let result = zyn::zyn!(
            @if (opts.is_pub) { pub }
            fn foo() {}
        );
        let expected = quote!(
            pub fn foo() {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn method_call_in_match() {
        let value = "hello".to_string();
        let result = zyn::zyn!(
            @match (value.len()) {
                5 => { struct Five; }
                _ => { struct Other; }
            }
        );
        let expected = quote!(
            struct Five;
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

    struct Item {
        field: Field,
    }

    #[test]
    fn field_access() {
        let field = Field {
            name: quote::format_ident!("age"),
            ty: quote::format_ident!("u32"),
        };
        let result = zyn::zyn!({{ field.name }}: {{ field.ty }});
        let expected = quote!(age: u32);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn nested_field_access() {
        let item = Item {
            field: Field {
                name: quote::format_ident!("age"),
                ty: quote::format_ident!("u32"),
            },
        };
        let result = zyn::zyn!({{ item.field.name }}: {{ item.field.ty }});
        let expected = quote!(age: u32);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn method_call() {
        let names = vec![quote::format_ident!("foo"), quote::format_ident!("bar")];
        let result = zyn::zyn!({ { names.len() } });
        let expected = quote!(2usize);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn chained_method_call() {
        let name = "hello_world".to_string();
        let result = zyn::zyn!({
            { proc_macro2::Ident::new(&name.to_uppercase(), proc_macro2::Span::call_site()) }
        });
        let expected = quote!(HELLO_WORLD);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn method_call_in_for() {
        let items = vec![
            vec![quote::format_ident!("a")],
            vec![quote::format_ident!("b"), quote::format_ident!("c")],
        ];
        let result = zyn::zyn!(
            @for (item of items) {
                {{ item.len() }},
            }
        );
        let expected = quote!(1usize, 2usize,);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn method_call_in_condition() {
        let items: Vec<i32> = vec![1, 2, 3];
        let result = zyn::zyn!(
            @if (items.is_empty()) { struct Empty; }
            @else { struct NonEmpty; }
        );
        let expected = quote!(
            struct NonEmpty;
        );
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn nested_field_with_pipe() {
        let item = Item {
            field: Field {
                name: quote::format_ident!("hello"),
                ty: quote::format_ident!("u32"),
            },
        };
        let result = zyn::zyn!({ { item.field.name | upper } });
        let expected = quote!(HELLO);
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod groups {
    use super::*;

    #[test]
    fn parenthesized() {
        let ty = quote::format_ident!("i32");
        let result = zyn::zyn!(fn foo(x: {{ ty }}));
        let expected = quote!(fn foo(x: i32));
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn bracketed() {
        let ty = quote::format_ident!("u8");
        let result = zyn::zyn!(type Foo = [{{ ty }}; 4];);
        let expected = quote!(
            type Foo = [u8; 4];
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod combined {
    use super::*;

    #[test]
    fn if_with_pipe_and_braces() {
        let name = quote::format_ident!("hello_world");
        let is_pub = true;
        let result = zyn::zyn!(
            @if (is_pub) { pub }
            fn {{ name | snake }}() {}
        );
        let expected = quote!(
            pub fn hello_world() {}
        );
        assert_eq!(result.to_string(), expected.to_string());
    }
}
