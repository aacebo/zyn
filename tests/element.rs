use proc_macro2::TokenStream;
use quote::quote;

#[zyn::element]
fn greeting(name: proc_macro2::Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn!(fn {{ name }}() {}))
}

#[test]
fn basic_element() -> syn::Result<()> {
    let result: TokenStream = zyn::zyn!(
        @greeting { name: quote::format_ident!("hello") }
    );
    let expected = quote!(
        fn hello() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
    Ok(())
}

#[zyn::element]
fn wrapper(
    name: proc_macro2::Ident,
    children: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote::quote!(struct #name { #children }))
}

#[test]
fn element_with_children() -> syn::Result<()> {
    let result: TokenStream = zyn::zyn!(
        @wrapper { name: quote::format_ident!("Foo") } {
            x: i32,
        }
    );
    let expected = quote!(
        struct Foo {
            x: i32,
        }
    );
    assert_eq!(result.to_string(), expected.to_string());
    Ok(())
}

#[zyn::element("say_hello")]
fn get_greeting(name: proc_macro2::Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn!(fn {{ name }}() {}))
}

#[test]
fn custom_name_override() -> syn::Result<()> {
    let result: TokenStream = zyn::zyn!(
        @say_hello { name: quote::format_ident!("world") }
    );
    let expected = quote!(
        fn world() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
    Ok(())
}

mod namespaced {
    use super::*;

    pub mod components {
        #[zyn::element]
        pub fn field_decl(
            name: proc_macro2::Ident,
            ty: proc_macro2::Ident,
        ) -> syn::Result<proc_macro2::TokenStream> {
            Ok(zyn::zyn!({{ name }}: {{ ty }},))
        }
    }

    #[test]
    fn namespaced_element() -> syn::Result<()> {
        let result: TokenStream = zyn::zyn!(
            @components::field_decl {
                name: quote::format_ident!("age"),
                ty: quote::format_ident!("u32"),
            }
        );
        let expected = quote!(age: u32,);
        assert_eq!(result.to_string(), expected.to_string());
        Ok(())
    }
}

#[test]
fn element_inside_for_loop() -> syn::Result<()> {
    let names = vec![quote::format_ident!("foo"), quote::format_ident!("bar")];
    let result: TokenStream = zyn::zyn!(
        @for (name of names) {
            @greeting { name: name.clone() }
        }
    );
    let expected = quote!(
        fn foo() {}
        fn bar() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
    Ok(())
}
