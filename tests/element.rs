use proc_macro2::TokenStream;
use quote::quote;

#[zyn::element]
fn greeting(name: proc_macro2::Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn!(fn {{ name }}() {}))
}

#[test]
fn basic_element() -> syn::Result<()> {
    let result: TokenStream = zyn::zyn!(
        @Greeting { name: quote::format_ident!("hello") }
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
        @Wrapper { name: quote::format_ident!("Foo") } {
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
