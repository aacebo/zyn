#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).into()
}

fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match syn::parse2::<zyn_core::ast::Element>(input) {
        Ok(element) => element.to_token_stream(),
        Err(e) => e.to_compile_error(),
    }
}
