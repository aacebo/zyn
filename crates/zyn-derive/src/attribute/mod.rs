mod emit;
mod enums;
mod structs;

use zyn_core::proc_macro2::TokenStream;
use zyn_core::syn;
use zyn_core::syn::Data;
use zyn_core::syn::DeriveInput;

pub fn expand(input: TokenStream) -> TokenStream {
    match zyn_core::parse!(input => DeriveInput) {
        Ok(input) => match &input.data {
            Data::Struct(_) => structs::expand(input),
            Data::Enum(_) => enums::expand(input),
            Data::Union(_) => syn::Error::new(
                syn::spanned::Spanned::span(&input.ident),
                "unions are not supported by #[derive(Attribute)]",
            )
            .to_compile_error(),
        },
        Err(e) => e.to_compile_error(),
    }
}
