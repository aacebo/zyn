use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::proc_macro2::Span;

#[test]
fn empty_displays_as_empty_string() {
    let d = Diagnostics::new();
    assert_eq!(format!("{d}"), "");
}

#[test]
fn single_error_contains_message() {
    let d = Diagnostics::error(Span::call_site(), "field `name` is required");
    let output = format!("{d}");
    assert!(output.contains("field `name` is required"));
}

#[test]
fn multiple_errors_contain_all_messages() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "missing `a`",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "missing `b`",
    ));

    let output = format!("{d}");
    assert!(output.contains("missing `a`"));
    assert!(output.contains("missing `b`"));
}

#[test]
fn multiple_diagnostics_separated_by_newlines() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "first",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Warning,
        "second",
    ));

    let output = format!("{d}");
    assert!(output.contains('\n'));
}

#[test]
fn mixed_levels_all_appear_in_output() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "err_msg",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Warning,
        "warn_msg",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Note,
        "note_msg",
    ));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Help,
        "help_msg",
    ));

    let output = format!("{d}");
    assert!(output.contains("err_msg"));
    assert!(output.contains("warn_msg"));
    assert!(output.contains("note_msg"));
    assert!(output.contains("help_msg"));
}

#[test]
fn from_syn_error_message_in_display() {
    let err = zyn::syn::Error::new(Span::call_site(), "syn error message");
    let d = Diagnostics::from(err);

    let output = format!("{d}");
    assert!(output.contains("syn error message"));
}

#[test]
fn debug_output_is_nonempty_for_errors() {
    let d = Diagnostics::error(Span::call_site(), "debug test");
    let output = format!("{d:?}");
    assert!(!output.is_empty());
}
