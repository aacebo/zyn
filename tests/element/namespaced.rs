use zyn::quote::quote;

pub mod components {
    #[zyn::element]
    pub fn field_decl(name: zyn::syn::Ident, ty: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!({{ name }}: {{ ty }},)
    }
}

fn derive_field_decl(tokens: &str) -> zyn::TokenStream {
    let input: zyn::Input = zyn::syn::parse_str(tokens).unwrap();
    zyn::zyn!(
        @components::field_decl(
            name = zyn::format_ident!("age"),
            ty = zyn::format_ident!("u32"),
        )
    )
}

#[test]
fn namespaced_element() {
    let result = derive_field_decl("struct Foo;");
    let expected = quote!(age: u32,);
    zyn::assert_tokens!(result, expected);
}
