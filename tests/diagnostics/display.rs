use zyn::Diagnostic;
use zyn::proc_macro2::Span;

#[test]
fn empty_displays_as_empty_string() {
    let d = zyn::mark::new().build();
    assert_eq!(format!("{d}"), "");
}

#[test]
fn single_error_contains_message() {
    let d = zyn::mark::error("field `name` is required").build();
    let output = format!("{d}");
    assert!(output.contains("field `name` is required"));
}

#[test]
fn multiple_errors_contain_all_messages() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("missing `a`"))
        .add(zyn::mark::error("missing `b`"))
        .build();

    let output = format!("{d}");
    assert!(output.contains("missing `a`"));
    assert!(output.contains("missing `b`"));
}

#[test]
fn multiple_diagnostics_separated_by_newlines() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("first"))
        .add(zyn::mark::warning("second"))
        .build();

    let output = format!("{d}");
    assert!(output.contains('\n'));
}

#[test]
fn mixed_levels_all_appear_in_output() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("err_msg"))
        .add(zyn::mark::warning("warn_msg"))
        .add(zyn::mark::note("note_msg"))
        .add(zyn::mark::help("help_msg"))
        .build();

    let output = format!("{d}");
    assert!(output.contains("err_msg"));
    assert!(output.contains("warn_msg"));
    assert!(output.contains("note_msg"));
    assert!(output.contains("help_msg"));
}

#[test]
fn from_syn_error_message_in_display() {
    let err = zyn::syn::Error::new(Span::call_site(), "syn error message");
    let d = Diagnostic::from(err);

    let output = format!("{d}");
    assert!(output.contains("syn error message"));
}

#[test]
fn debug_output_is_nonempty_for_errors() {
    let d = zyn::mark::error("debug test").build();
    let output = format!("{d:?}");
    assert!(!output.is_empty());
}
