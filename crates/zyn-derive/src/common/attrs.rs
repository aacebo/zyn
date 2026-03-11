use zyn_core::syn;

pub fn exists(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path().is_ident("zyn")
            && a.parse_args::<syn::Ident>()
                .map(|i| i == "input")
                .unwrap_or(false)
    })
}
