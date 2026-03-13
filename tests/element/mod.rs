mod extract;
mod namespaced;
#[cfg(feature = "pretty")]
mod pretty;

use zyn::{format_ident, quote::quote};

#[zyn::element]
fn greeting(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

fn derive_greeting(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        @greeting(name = zyn::format_ident!("hello"))
    )
}

#[test]
fn basic_element() {
    let result = derive_greeting("struct Foo;");
    let expected = quote!(
        fn hello() {}
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn wrapper(name: zyn::syn::Ident, children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::quote::quote!(struct #name { #children })
}

fn derive_wrapper(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        @wrapper(name = zyn::format_ident!("Foo")) {
            x: i32,
        }
    )
}

#[test]
fn element_with_children() {
    let result = derive_wrapper("struct Foo;");
    let expected = quote!(
        struct Foo {
            x: i32,
        }
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element("say_hello")]
fn get_greeting(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

fn derive_say_hello(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        @say_hello(name = zyn::format_ident!("world"))
    )
}

#[test]
fn custom_name_override() {
    let result = derive_say_hello("struct Foo;");
    let expected = quote!(
        fn world() {}
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn divider() -> zyn::TokenStream {
    zyn::zyn!(
        const DIVIDER: &str = "---";
    )
}

fn derive_divider(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(@divider)
}

fn derive_divider_parens(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(@divider())
}

#[test]
fn zero_param_no_parens() {
    let result = derive_divider("struct Foo;");
    let expected = quote!(
        const DIVIDER: &str = "---";
    );
    zyn::assert_tokens!(result, expected);
}

#[test]
fn zero_param_with_parens() {
    let result = derive_divider_parens("struct Foo;");
    let expected = quote!(
        const DIVIDER: &str = "---";
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn container(children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::quote::quote!(mod container { #children })
}

fn derive_container(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        @container {
            struct Inner;
        }
    )
}

#[test]
fn children_without_parens() {
    let result = derive_container("struct Foo;");
    let expected = quote!(
        mod container {
            struct Inner;
        }
    );
    zyn::assert_tokens!(result, expected);
}

fn derive_for_greeting(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    let names = vec![zyn::format_ident!("foo"), zyn::format_ident!("bar")];
    zyn::zyn!(
        @for (name in names) {
            @greeting(name = name.clone())
        }
    )
}

#[test]
fn element_inside_for_loop() {
    let result = derive_for_greeting("struct Foo;");
    let expected = quote!(
        fn foo() {}
        fn bar() {}
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn generics<T: zyn::ToTokens>(children: T) -> zyn::TokenStream {
    zyn::quote::quote!(#children)
}

fn derive_generics(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        fn gen() {
            @generics {
                {{ input }}
            }
        }
    )
}

#[test]
fn element_with_generics() {
    let result = derive_generics("struct Foo;");
    let expected = quote!(
        fn gen() {
            struct Foo;
        }
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn generics_with_where_clause<T>(children: T) -> zyn::TokenStream
where
    T: zyn::ToTokens,
{
    zyn::quote::quote!(#children)
}

fn derive_generics_with_where_clause(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        fn gen() {
            @generics_with_where_clause {
                {{ input }}
            }
        }
    )
}

#[test]
fn element_with_generics_with_where_clause() {
    let result = derive_generics_with_where_clause("struct Foo;");
    let expected = quote!(
        fn gen() {
            struct Foo;
        }
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn lifetimes<'a>(ident: &'a zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(struct {{ ident }};)
}

fn derive_lifetimes(tokens: &str) -> zyn::Output {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    let ident = format_ident!("Bar");
    let ident_ref = &ident;
    zyn::zyn!(
        @lifetimes(ident = ident_ref)
    )
}

#[test]
fn element_with_lifetimes() {
    let result = derive_lifetimes("struct Foo;");
    let expected = quote!(
        struct Bar;
    );
    zyn::assert_tokens!(result, expected);
}
