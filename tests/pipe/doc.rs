use zyn::quote::quote;

/// A documented pipe.
#[zyn::pipe]
fn documented_pipe(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(&input.to_uppercase(), zyn::Span::call_site())
}

/// Multi-line doc comment.
///
/// Second paragraph.
#[zyn::pipe]
fn documented_pipe_multi(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(&input.to_uppercase(), zyn::Span::call_site())
}

#[test]
fn doc_comment_pipe_compiles() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | documented_pipe } });
    let expected = quote!(HELLO);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn multi_line_doc_comment_pipe_compiles() {
    let name = zyn::format_ident!("world");
    let result = zyn::zyn!({ { name | documented_pipe_multi } });
    let expected = quote!(WORLD);
    zyn::assert_tokens!(result, expected);
}
