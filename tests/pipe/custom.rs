use zyn::quote::quote;

#[zyn::pipe]
fn shout(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(
        &format!("{}_BANG", input.to_uppercase()),
        zyn::Span::call_site(),
    )
}

#[test]
fn custom_pipe() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | shout } });
    let expected = quote!(HELLO_BANG);
    zyn::assert_tokens!(result, expected);
}

#[zyn::pipe("yell")]
fn make_loud(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(
        &format!("{}__LOUD", input.to_uppercase()),
        zyn::Span::call_site(),
    )
}

#[test]
fn custom_name_override() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | yell } });
    let expected = quote!(HELLO__LOUD);
    zyn::assert_tokens!(result, expected);
}
