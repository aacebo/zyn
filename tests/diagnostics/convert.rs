use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::ToDiagnostics;
use zyn::proc_macro2::Span;

#[test]
fn syn_error_to_diagnostics_preserves_message() {
    let err = zyn::syn::Error::new(Span::call_site(), "conversion test");
    let d = err.to_diagnostics();
    assert_eq!(d.len(), 1);
    assert!(d.has_errors());

    let output = format!("{d}");
    assert!(output.contains("conversion test"));
}

#[test]
fn syn_error_combined_to_diagnostics_preserves_all() {
    let mut err = zyn::syn::Error::new(Span::call_site(), "alpha");
    err.combine(zyn::syn::Error::new(Span::call_site(), "beta"));

    let d = err.to_diagnostics();
    assert_eq!(d.len(), 2);

    let output = format!("{d}");
    assert!(output.contains("alpha"));
    assert!(output.contains("beta"));
}

#[test]
fn diagnostics_to_diagnostics_is_identity() {
    let original = Diagnostics::error(Span::call_site(), "identity test");
    let converted = original.to_diagnostics();
    assert_eq!(converted.len(), 1);

    let output = format!("{converted}");
    assert!(output.contains("identity test"));
}

#[test]
fn from_syn_error_creates_error_level() {
    let err = zyn::syn::Error::new(Span::call_site(), "level check");
    let d = Diagnostics::from(err);

    for diag in &d {
        assert_eq!(diag.level(), Level::Error);
    }
}

#[test]
fn from_diagnostic_preserves_warning_level() {
    let diag = Diagnostic::spanned(Span::call_site(), Level::Warning, "warn level");
    let d = Diagnostics::from(diag);
    assert_eq!(d.max_level(), Some(Level::Warning));
    assert!(!d.has_errors());
}

#[test]
fn from_diagnostic_preserves_note_level() {
    let diag = Diagnostic::spanned(Span::call_site(), Level::Note, "note level");
    let d = Diagnostics::from(diag);
    assert_eq!(d.max_level(), Some(Level::Note));
}

#[test]
fn from_diagnostic_preserves_help_level() {
    let diag = Diagnostic::spanned(Span::call_site(), Level::Help, "help level");
    let d = Diagnostics::from(diag);
    assert_eq!(d.max_level(), Some(Level::Help));
}
