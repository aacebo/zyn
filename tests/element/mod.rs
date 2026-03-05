mod extract;
mod namespaced;

use zyn::quote::quote;

#[zyn::element]
fn greeting(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn basic_element() {
    let result = zyn::zyn!(
        @greeting(name = zyn::format_ident!("hello"))
    );
    let expected = quote!(
        fn hello() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn wrapper(name: zyn::syn::Ident, children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::quote::quote!(struct #name { #children })
}

#[test]
fn element_with_children() {
    let result = zyn::zyn!(
        @wrapper(name = zyn::format_ident!("Foo")) {
            x: i32,
        }
    );
    let expected = quote!(
        struct Foo {
            x: i32,
        }
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element("say_hello")]
fn get_greeting(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn custom_name_override() {
    let result = zyn::zyn!(
        @say_hello(name = zyn::format_ident!("world"))
    );
    let expected = quote!(
        fn world() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn divider() -> zyn::TokenStream {
    zyn::zyn!(
        const DIVIDER: &str = "---";
    )
}

#[test]
fn zero_param_no_parens() {
    let result = zyn::zyn!(@divider);
    let expected = quote!(
        const DIVIDER: &str = "---";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn zero_param_with_parens() {
    let result = zyn::zyn!(@divider());
    let expected = quote!(
        const DIVIDER: &str = "---";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn container(children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::quote::quote!(mod container { #children })
}

#[test]
fn children_without_parens() {
    let result = zyn::zyn!(
        @container {
            struct Inner;
        }
    );
    let expected = quote!(
        mod container {
            struct Inner;
        }
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn element_inside_for_loop() {
    let names = vec![zyn::format_ident!("foo"), zyn::format_ident!("bar")];
    let result = zyn::zyn!(
        @for (name in names) {
            @greeting(name = name.clone())
        }
    );
    let expected = quote!(
        fn foo() {}
        fn bar() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}
