use zyn::Render;
use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
fn fallback_bail(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        bail!("not allowed");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn fallback_bail_emits_compile_error() {
    let tokens = FallbackBail {
        name: zyn::format_ident!("bad"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("compile_error"), "got: {output}");
    assert!(output.contains("not allowed"), "got: {output}");
}

#[zyn::element]
fn fallback_warn(name: syn::Ident) -> zyn::TokenStream {
    warn!("deprecated");
    bail!();
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn fallback_warn_does_not_bail() {
    let tokens = FallbackWarn {
        name: zyn::format_ident!("my_fn"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("my_fn"),
        "expected body output, got: {output}"
    );
}
