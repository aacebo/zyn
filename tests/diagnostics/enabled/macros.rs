use zyn::Render;
use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
fn enabled_bail(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        bail!("not allowed");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_emits_error() {
    let tokens = EnabledBail {
        name: zyn::format_ident!("bad"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("not allowed"), "got: {output}");
}

#[test]
fn bail_allows_valid() {
    let tokens = EnabledBail {
        name: zyn::format_ident!("good"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("good"), "got: {output}");
}

#[zyn::element]
fn enabled_multi(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        error!("name is bad");
        help!("use a different name");
    }
    bail!();
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn error_and_help_accumulate() {
    let tokens = EnabledMulti {
        name: zyn::format_ident!("bad"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("name is bad"), "got: {output}");
    assert!(output.contains("use a different name"), "got: {output}");
}
