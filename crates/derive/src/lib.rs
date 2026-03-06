mod attribute;
mod common;
mod macros;

#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::template::expand(input.into()).into()
}

#[proc_macro]
pub fn debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::debug::expand(input.into()).into()
}

#[proc_macro_attribute]
pub fn element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::element::expand(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn pipe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::pipe::expand(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn derive(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::derive::expand(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn attribute(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    macros::attribute::expand(args.into(), input.into()).into()
}

#[proc_macro_derive(Attribute, attributes(zyn))]
pub fn derive_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute::expand(input.into()).into()
}
