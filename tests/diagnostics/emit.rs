use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::proc_macro2::Span;

#[test]
fn empty_emits_empty_tokens() {
    let d = Diagnostics::new();
    let tokens = d.emit();
    assert!(tokens.is_empty());
}

#[test]
fn single_error_emits_compile_error() {
    let d = Diagnostics::error(Span::call_site(), "broken input");
    let tokens = d.emit();
    let output = tokens.to_string();
    assert!(output.contains("compile_error"));
    assert!(output.contains("broken input"));
}

#[test]
fn multiple_errors_emit_multiple_compile_errors() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "error one",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "error two",
    ));

    let tokens = d.emit();
    let output = tokens.to_string();
    assert!(output.contains("error one"));
    assert!(output.contains("error two"));
}

#[test]
fn warning_emits_nonempty_tokens() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Warning,
        "deprecation",
    ));

    let tokens = d.emit();
    assert!(!tokens.is_empty());
}

#[test]
fn mixed_levels_all_emit() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "err_token",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Warning,
        "warn_token",
    ));

    let tokens = d.emit();
    let output = tokens.to_string();
    assert!(output.contains("err_token"));
}
