use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
pub fn fallback_bail(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        bail!("not allowed");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn fallback_bail_emits_compile_error() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@fallback_bail(name = zyn::format_ident!("bad")));
    zyn::assert_tokens_contain!(output, "compile_error");
    zyn::assert_tokens_contain!(output, "not allowed");
}

#[zyn::element]
pub fn fallback_warn(name: syn::Ident) -> zyn::TokenStream {
    warn!("deprecated");
    bail!();
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn fallback_warn_does_not_bail() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@fallback_warn(name = zyn::format_ident!("my_fn")));
    zyn::assert_tokens_contain!(output, "my_fn");
}
