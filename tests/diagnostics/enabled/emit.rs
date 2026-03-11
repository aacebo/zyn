use zyn::Render;
use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
fn emit_error() -> zyn::TokenStream {
    bail!("broken input");
    zyn::TokenStream::new()
}

#[test]
fn error_emits_tokens() {
    let tokens = (EmitError {}).render(&dummy_input());
    let output = tokens.to_string();
    assert!(!output.is_empty(), "expected non-empty token output");
    assert!(output.contains("broken input"), "got: {output}");
}

#[zyn::element]
fn emit_warning() -> zyn::TokenStream {
    warn!("deprecated usage");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn warning_emits_tokens() {
    let tokens = (EmitWarning {}).render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("deprecated usage"), "got: {output}");
}

#[zyn::element]
fn emit_multiple() -> zyn::TokenStream {
    error!("first");
    error!("second");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn multiple_errors_all_emit() {
    let tokens = (EmitMultiple {}).render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("first"), "got: {output}");
    assert!(output.contains("second"), "got: {output}");
}

#[zyn::element]
fn emit_nothing(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn no_diagnostics_passes_through() {
    let tokens = EmitNothing {
        name: zyn::format_ident!("my_fn"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("my_fn"), "got: {output}");
}
