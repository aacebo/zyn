use zyn::Level;

#[test]
fn push_preserves_insertion_order() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("first"))
        .add(zyn::mark::warning("second"))
        .add(zyn::mark::note("third"))
        .build();

    let levels: Vec<Level> = d.iter().map(|diag| diag.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning, Level::Note]);
}

#[test]
fn push_all_four_levels() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("err"))
        .add(zyn::mark::warning("warn"))
        .add(zyn::mark::note("note"))
        .add(zyn::mark::help("help"))
        .build();
    assert_eq!(d.len(), 4);

    let output = format!("{d}");
    assert!(output.contains("err"));
    assert!(output.contains("warn"));
    assert!(output.contains("note"));
    assert!(output.contains("help"));
}

#[test]
fn add_merges_in_order() {
    let a = zyn::mark::new()
        .add(zyn::mark::error("from_a"))
        .add(zyn::mark::warning("from_b"))
        .build();

    assert_eq!(a.len(), 2);

    let levels: Vec<Level> = a.iter().map(|diag| diag.level()).collect();
    assert_eq!(levels, vec![Level::Error, Level::Warning]);
}

#[test]
fn add_diagnostic_via_into() {
    let existing = zyn::mark::error("only one").build();
    let d = zyn::mark::new().add(existing).build();
    assert!(d.is_error());
}

#[test]
fn add_builder_into_builder() {
    let d = zyn::mark::new().add(zyn::mark::error("added")).build();
    assert!(d.is_error());
}

#[test]
fn accumulate_multiple_error_sources() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("missing field `x`"))
        .add(zyn::mark::error("missing field `y`"))
        .add(zyn::mark::error("unknown argument `z`"))
        .build();

    assert!(d.is_error());

    let output = format!("{d}");
    assert!(output.contains("missing field `x`"));
    assert!(output.contains("missing field `y`"));
    assert!(output.contains("unknown argument `z`"));
}
