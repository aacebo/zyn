use quote::quote;

#[zyn::element]
fn greeting(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn basic_element() {
    let result = zyn::zyn!(
        @greeting(name = quote::format_ident!("hello"))
    );
    let expected = quote!(
        fn hello() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn wrapper(
    name: proc_macro2::Ident,
    children: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote!(struct #name { #children })
}

#[test]
fn element_with_children() {
    let result = zyn::zyn!(
        @wrapper(name = quote::format_ident!("Foo")) {
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
fn get_greeting(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn custom_name_override() {
    let result = zyn::zyn!(
        @say_hello(name = quote::format_ident!("world"))
    );
    let expected = quote!(
        fn world() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

mod namespaced {
    use super::*;

    pub mod components {
        #[zyn::element]
        pub fn field_decl(
            name: proc_macro2::Ident,
            ty: proc_macro2::Ident,
        ) -> proc_macro2::TokenStream {
            zyn::zyn!({{ name }}: {{ ty }},)
        }
    }

    #[test]
    fn namespaced_element() {
        let result = zyn::zyn!(
            @components::field_decl(
                name = quote::format_ident!("age"),
                ty = quote::format_ident!("u32"),
            )
        );
        let expected = quote!(age: u32,);
        assert_eq!(result.to_string(), expected.to_string());
    }
}

#[zyn::element]
fn divider() -> proc_macro2::TokenStream {
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
fn container(children: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote!(mod container { #children })
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
    let names = vec![quote::format_ident!("foo"), quote::format_ident!("bar")];
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
