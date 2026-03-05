use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::proc_macro2::Span;

#[test]
fn new_is_empty() {
    let d = Diagnostics::new();
    assert!(d.is_empty());
    assert_eq!(d.len(), 0);
    assert!(!d.has_errors());
    assert!(d.max_level().is_none());
}

#[test]
fn default_is_empty() {
    let d = Diagnostics::default();
    assert!(d.is_empty());
}

#[test]
fn error_creates_single_with_correct_level() {
    let d = Diagnostics::error(Span::call_site(), "something broke");
    assert_eq!(d.len(), 1);
    assert!(d.has_errors());
    assert_eq!(d.max_level(), Some(Level::Error));

    let output = format!("{d}");
    assert!(output.contains("something broke"));
}

#[test]
fn from_single_diagnostic_preserves_level() {
    let diag = Diagnostic::spanned(Span::call_site(), Level::Warning, "a warning");
    let d = Diagnostics::from(diag);
    assert_eq!(d.len(), 1);
    assert!(!d.has_errors());
    assert_eq!(d.max_level(), Some(Level::Warning));

    let output = format!("{d}");
    assert!(output.contains("a warning"));
}

#[test]
fn from_syn_error_preserves_message() {
    let err = zyn::syn::Error::new(Span::call_site(), "field is missing");
    let d = Diagnostics::from(err);
    assert_eq!(d.len(), 1);
    assert!(d.has_errors());

    let output = format!("{d}");
    assert!(output.contains("field is missing"));
}

#[test]
fn from_syn_error_combined_preserves_all_messages() {
    let mut err = zyn::syn::Error::new(Span::call_site(), "first problem");
    err.combine(zyn::syn::Error::new(Span::call_site(), "second problem"));
    err.combine(zyn::syn::Error::new(Span::call_site(), "third problem"));

    let d = Diagnostics::from(err);
    assert_eq!(d.len(), 3);

    let output = format!("{d}");
    assert!(output.contains("first problem"));
    assert!(output.contains("second problem"));
    assert!(output.contains("third problem"));
}
