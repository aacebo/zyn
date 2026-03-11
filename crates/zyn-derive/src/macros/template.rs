use zyn_core::proc_macro2::TokenStream;

pub fn expand(input: TokenStream) -> TokenStream {
    match zyn_core::parse!(input => zyn_core::Template) {
        Ok(element) => element.to_token_stream(),
        Err(e) => e.to_compile_error(),
    }
}
