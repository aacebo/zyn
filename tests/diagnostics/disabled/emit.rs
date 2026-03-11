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
fn fallback_emits_compile_error() {
    let tokens = EmitError.render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("compile_error"), "got: {output}");
    assert!(output.contains("broken input"), "got: {output}");
}

#[zyn::element]
fn emit_warning() -> zyn::TokenStream {
    warn!("deprecated usage");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn fallback_warning_prefixes_message() {
    let tokens = EmitWarning.render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("warning:"), "got: {output}");
    assert!(output.contains("deprecated usage"), "got: {output}");
}

#[zyn::element]
fn emit_note() -> zyn::TokenStream {
    note!("see documentation");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn fallback_note_prefixes_message() {
    let tokens = EmitNote.render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("note:"), "got: {output}");
    assert!(output.contains("see documentation"), "got: {output}");
}

#[zyn::element]
fn emit_help() -> zyn::TokenStream {
    help!("try this instead");
    bail!("stop");
    zyn::TokenStream::new()
}

#[test]
fn fallback_help_prefixes_message() {
    let tokens = EmitHelp.render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("help:"), "got: {output}");
    assert!(output.contains("try this instead"), "got: {output}");
}

#[zyn::element]
fn emit_multiple() -> zyn::TokenStream {
    error!("first");
    error!("second");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn fallback_multiple_errors() {
    let tokens = EmitMultiple.render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("first"), "got: {output}");
    assert!(output.contains("second"), "got: {output}");
}
