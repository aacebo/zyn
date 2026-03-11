use zyn::Diagnostic;
use zyn::Level;

#[test]
fn iter_yields_in_push_order() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("first"))
        .add(zyn::mark::warning("second"))
        .add(zyn::mark::note("third"))
        .build();

    let levels: Vec<Level> = d.iter().map(|diag| diag.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning, Level::Note]);
}

#[test]
fn iter_empty_yields_nothing() {
    let d = zyn::mark::new().build();
    assert_eq!(d.iter().count(), 0);
}

#[test]
fn into_iter_consumes_all() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("a"))
        .add(zyn::mark::warning("b"))
        .add(zyn::mark::note("c"))
        .add(zyn::mark::help("d"))
        .build();

    let collected: Vec<Diagnostic> = d.into_iter().collect();
    assert_eq!(collected.len(), 4);
    assert_eq!(collected[0].level(), Level::Error);
    assert_eq!(collected[1].level(), Level::Warning);
    assert_eq!(collected[2].level(), Level::Note);
    assert_eq!(collected[3].level(), Level::Help);
}

#[test]
fn ref_into_iter_borrows() {
    let d = zyn::mark::new().add(zyn::mark::error("borrow")).build();

    let count = (&d).into_iter().count();
    assert_eq!(count, 1);
    assert_eq!(d.len(), 1);
}

#[test]
fn iter_with_multiple_children() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("a1"))
        .add(zyn::mark::warning("b1"))
        .add(zyn::mark::note("b2"))
        .build();

    let levels: Vec<Level> = d.iter().map(|d| d.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning, Level::Note]);
}
