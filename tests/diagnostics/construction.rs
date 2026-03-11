use zyn::Diagnostic;
use zyn::mark::Level;
use zyn::proc_macro2::Span;

#[test]
fn new_is_empty() {
    let d = zyn::mark::new().build();
    assert!(d.is_empty());
    assert_eq!(d.len(), 0);
    assert!(!d.is_error());
    assert_eq!(d.level(), Level::None);
}

#[test]
fn default_is_empty() {
    let d = Diagnostic::default();
    assert!(d.is_empty());
}

#[test]
fn error_creates_single_with_correct_level() {
    let d = zyn::mark::error("something broke").build();
    assert!(d.is_error());
    assert_eq!(d.level(), Level::Error);

    let output = format!("{d}");
    assert!(output.contains("something broke"));
}

#[test]
fn from_syn_error_preserves_message() {
    let err = zyn::syn::Error::new(Span::call_site(), "field is missing");
    let d = Diagnostic::from(err);
    assert_eq!(d.len(), 1);
    assert!(d.is_error());

    let output = format!("{d}");
    assert!(output.contains("field is missing"));
}

#[test]
fn from_syn_error_combined_preserves_all_messages() {
    let mut err = zyn::syn::Error::new(Span::call_site(), "first problem");
    err.combine(zyn::syn::Error::new(Span::call_site(), "second problem"));
    err.combine(zyn::syn::Error::new(Span::call_site(), "third problem"));

    let d = Diagnostic::from(err);
    assert_eq!(d.len(), 3);

    let output = format!("{d}");
    assert!(output.contains("first problem"));
    assert!(output.contains("second problem"));
    assert!(output.contains("third problem"));
}
