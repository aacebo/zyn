use zyn::quote::quote;

#[test]
fn for_classic_literal() {
    let result = zyn::zyn!(@for (3) { x, });
    let expected = quote!(x, x, x,);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn for_classic_variable() {
    let count = 2;
    let result = zyn::zyn!(@for (count) { z, });
    let expected = quote!(z, z,);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn for_classic_method_call() {
    let items = [1, 2, 3, 4];
    let result = zyn::zyn!(@for (items.len()) { w, });
    let expected = quote!(w, w, w, w,);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
#[allow(clippy::reversed_empty_ranges)]
fn for_classic_zero() {
    let result = zyn::zyn!(@for (0) { x, });
    assert!(result.is_empty());
}

#[test]
fn for_range_with_wildcard() {
    let items = [11, 22, 33];
    let result = zyn::zyn!(@for (i in 0..items.len()) { {{ i }}, });
    let expected = quote!(0usize, 1usize, 2usize,);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn for_range_with_binding() {
    let result = zyn::zyn!(@for (i in 0..3usize) { {{ i }}, });
    let expected = quote!(0usize, 1usize, 2usize,);
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
#[allow(clippy::reversed_empty_ranges)]
fn for_range_empty() {
    let result = zyn::zyn!(@for (_ in 0..0) { x, });
    assert!(result.is_empty());
}

#[test]
fn for_range_with_interpolation() {
    let names = ["a", "b", "c"];
    let result = zyn::zyn!(
        @for (i in 0..names.len()) {
            {{ zyn::format_ident!("{}", names[i]) }},
        }
    );
    let expected = quote!(a, b, c,);
    assert_eq!(result.to_string(), expected.to_string());
}

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
    let names = vec![zyn::format_ident!("a"), zyn::format_ident!("b")];
    let result = zyn::zyn!(@for (name in names) { {{ name }}, });
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
    let items: Vec<zyn::syn::Ident> = vec![];
    let result = zyn::zyn!(@for (item in items) { {{ item }} });
    assert!(result.is_empty());
}

#[test]
fn for_inline_iterator() {
    let result = zyn::zyn!(
        @for (name in ["x", "y", "z"].map(|s| zyn::format_ident!("{}", s))) {
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
        (zyn::format_ident!("a"), true),
        (zyn::format_ident!("b"), false),
    ];
    let result = zyn::zyn!(
        @for (item in items) {
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
