use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::proc_macro2::Span;

#[test]
fn max_level_empty_is_none() {
    assert!(Diagnostics::new().max_level().is_none());
}

#[test]
fn max_level_error_only() {
    let d = Diagnostics::error(Span::call_site(), "err");
    assert_eq!(d.max_level(), Some(Level::Error));
}

#[test]
fn max_level_warning_only() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    assert_eq!(d.max_level(), Some(Level::Warning));
}

#[test]
fn max_level_note_only() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "n"));
    assert_eq!(d.max_level(), Some(Level::Note));
}

#[test]
fn max_level_help_only() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Help, "h"));
    assert_eq!(d.max_level(), Some(Level::Help));
}

#[test]
fn max_level_error_beats_warning() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Error, "e"));
    assert_eq!(d.max_level(), Some(Level::Error));
}

#[test]
fn max_level_warning_beats_note() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "n"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    assert_eq!(d.max_level(), Some(Level::Warning));
}

#[test]
fn max_level_warning_beats_help() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Help, "h"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    assert_eq!(d.max_level(), Some(Level::Warning));
}

#[test]
fn max_level_all_four_returns_error() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "n"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Help, "h"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Error, "e"));
    assert_eq!(d.max_level(), Some(Level::Error));
}

#[test]
fn has_errors_with_error() {
    let d = Diagnostics::error(Span::call_site(), "e");
    assert!(d.has_errors());
}

#[test]
fn has_errors_false_when_empty() {
    assert!(!Diagnostics::new().has_errors());
}

#[test]
fn has_errors_false_with_only_warnings() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    assert!(!d.has_errors());
}

#[test]
fn has_errors_false_with_note_and_help() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "n"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Help, "h"));
    assert!(!d.has_errors());
}

#[test]
fn has_errors_true_with_mixed_levels() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "w"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "n"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Error, "e"));
    assert!(d.has_errors());
}
