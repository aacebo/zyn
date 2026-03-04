mod element;
mod pipe;

use zyn_core::proc_macro2;
use zyn_core::syn;

#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).into()
}

#[proc_macro_attribute]
pub fn element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    element::expand(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn pipe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    pipe::expand(args.into(), input.into()).into()
}

fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match syn::parse2::<zyn_core::ast::Element>(input) {
        Ok(element) => element.to_token_stream(),
        Err(e) => e.to_compile_error(),
    }
}
