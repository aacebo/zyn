use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::proc_macro2::Span;

#[test]
fn push_preserves_insertion_order() {
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
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "third"));

    let levels: Vec<Level> = d.iter().map(|diag| diag.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning, Level::Note]);
}

#[test]
fn push_all_four_levels() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Error, "err"));
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Warning,
        "warn",
    ));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "note"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Help, "help"));
    assert_eq!(d.len(), 4);

    let output = format!("{d}");
    assert!(output.contains("err"));
    assert!(output.contains("warn"));
    assert!(output.contains("note"));
    assert!(output.contains("help"));
}

#[test]
fn extend_merges_in_order() {
    let mut a = Diagnostics::new();
    a.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "from_a",
    ));

    let mut b = Diagnostics::new();
    b.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Warning,
        "from_b",
    ));

    a.extend(b);
    assert_eq!(a.len(), 2);

    let levels: Vec<Level> = a.iter().map(|diag| diag.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning]);
}

#[test]
fn extend_empty_into_nonempty_is_noop() {
    let mut a = Diagnostics::error(Span::call_site(), "only one");
    a.extend(Diagnostics::new());
    assert_eq!(a.len(), 1);
}

#[test]
fn extend_nonempty_into_empty() {
    let mut a = Diagnostics::new();
    a.extend(Diagnostics::error(Span::call_site(), "added"));
    assert_eq!(a.len(), 1);
    assert!(a.has_errors());
}

#[test]
fn accumulate_multiple_error_sources() {
    let mut diags = Diagnostics::new();

    let err1 = Diagnostics::error(Span::call_site(), "missing field `x`");
    diags.extend(err1);

    let err2 = Diagnostics::error(Span::call_site(), "missing field `y`");
    diags.extend(err2);

    diags.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "unknown argument `z`",
    ));

    assert_eq!(diags.len(), 3);
    assert!(diags.has_errors());

    let output = format!("{diags}");
    assert!(output.contains("missing field `x`"));
    assert!(output.contains("missing field `y`"));
    assert!(output.contains("unknown argument `z`"));
}
