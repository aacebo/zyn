use zyn::quote::quote;

pub mod components {
    #[zyn::element]
    pub fn field_decl(name: zyn::syn::Ident, ty: zyn::syn::Ident) -> zyn::TokenStream {
        zyn::zyn!({{ name }}: {{ ty }},)
    }
}

#[test]
fn namespaced_element() {
    let result = zyn::zyn!(
        @components::field_decl(
            name = zyn::format_ident!("age"),
            ty = zyn::format_ident!("u32"),
        )
    );
    let expected = quote!(age: u32,);
    assert_eq!(result.to_string(), expected.to_string());
}
