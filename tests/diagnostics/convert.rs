use zyn::Diagnostic;
use zyn::mark::Level;
use zyn::proc_macro2::Span;

#[test]
fn syn_error_to_diagnostic_preserves_message() {
    let err = zyn::syn::Error::new(Span::call_site(), "conversion test");
    let d = Diagnostic::from(err);
    assert_eq!(d.len(), 1);
    assert!(d.is_error());

    let output = format!("{d}");
    assert!(output.contains("conversion test"));
}

#[test]
fn syn_error_combined_to_diagnostic_preserves_all() {
    let mut err = zyn::syn::Error::new(Span::call_site(), "alpha");
    err.combine(zyn::syn::Error::new(Span::call_site(), "beta"));

    let d = Diagnostic::from(err);
    assert_eq!(d.len(), 2);

    let output = format!("{d}");
    assert!(output.contains("alpha"));
    assert!(output.contains("beta"));
}

#[test]
fn from_syn_error_creates_error_level() {
    let err = zyn::syn::Error::new(Span::call_site(), "level check");
    let d = Diagnostic::from(err);

    for diag in &d {
        assert_eq!(diag.level(), Level::Error);
    }
}
