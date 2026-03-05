use zyn::Diagnostic;
use zyn::Diagnostics;
use zyn::Level;
use zyn::proc_macro2::Span;

#[test]
fn iter_yields_in_push_order() {
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
fn iter_empty_yields_nothing() {
    let d = Diagnostics::new();
    assert_eq!(d.iter().count(), 0);
}

#[test]
fn into_iter_consumes_all() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(Span::call_site(), Level::Error, "a"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "b"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Note, "c"));
    d.push(Diagnostic::spanned(Span::call_site(), Level::Help, "d"));

    let collected: Vec<Diagnostic> = d.into_iter().collect();
    assert_eq!(collected.len(), 4);
    assert_eq!(collected[0].level(), Level::Error);
    assert_eq!(collected[1].level(), Level::Warning);
    assert_eq!(collected[2].level(), Level::Note);
    assert_eq!(collected[3].level(), Level::Help);
}

#[test]
fn ref_into_iter_borrows() {
    let mut d = Diagnostics::new();
    d.push(Diagnostic::spanned(
        Span::call_site(),
        Level::Error,
        "borrow",
    ));

    let count = (&d).into_iter().count();
    assert_eq!(count, 1);
    assert_eq!(d.len(), 1);
}

#[test]
fn iter_after_extend_preserves_order() {
    let mut a = Diagnostics::new();
    a.push(Diagnostic::spanned(Span::call_site(), Level::Error, "a1"));

    let mut b = Diagnostics::new();
    b.push(Diagnostic::spanned(Span::call_site(), Level::Warning, "b1"));
    b.push(Diagnostic::spanned(Span::call_site(), Level::Note, "b2"));

    a.extend(b);

    let levels: Vec<Level> = a.iter().map(|d| d.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning, Level::Note]);
}
