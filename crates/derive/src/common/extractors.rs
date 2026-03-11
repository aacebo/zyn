use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn;

pub fn bindings(
    names: &[syn::Ident],
    types: &[syn::Type],
    input_expr: &TokenStream,
) -> Vec<TokenStream> {
    names
        .iter()
        .zip(types.iter())
        .map(|(name, ty)| {
            quote! {
                let #name = match <#ty as ::zyn::FromInput>::from_input(#input_expr) {
                    ::std::result::Result::Ok(v) => v,
                    ::std::result::Result::Err(e) => {
                        diagnostics = diagnostics.add(e);
                        return diagnostics.build().emit();
                    }
                };
            }
        })
        .collect()
}
