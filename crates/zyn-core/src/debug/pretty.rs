use proc_macro2::TokenStream;

pub fn pretty(tokens: &TokenStream) -> String {
    let raw = tokens.to_string();

    match syn::parse_str::<syn::File>(&raw) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => raw,
    }
}
