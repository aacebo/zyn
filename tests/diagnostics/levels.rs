use zyn::Level;

#[test]
fn level_empty_is_none() {
    assert_eq!(zyn::mark::new().build().level(), Level::None);
}

#[test]
fn level_error_only() {
    let d = zyn::mark::error("err").build();
    assert_eq!(d.level(), Level::Error);
}

#[test]
fn level_warning_only() {
    let d = zyn::mark::new().add(zyn::mark::warning("w")).build();
    assert_eq!(d.level(), Level::Warning);
}

#[test]
fn level_note_only() {
    let d = zyn::mark::new().add(zyn::mark::note("n")).build();
    assert_eq!(d.level(), Level::Note);
}

#[test]
fn level_help_only() {
    let d = zyn::mark::new().add(zyn::mark::help("h")).build();
    assert_eq!(d.level(), Level::Help);
}

#[test]
fn level_error_beats_warning() {
    let d = zyn::mark::new()
        .add(zyn::mark::warning("w"))
        .add(zyn::mark::error("e"))
        .build();
    assert_eq!(d.level(), Level::Error);
}

#[test]
fn level_warning_beats_note() {
    let d = zyn::mark::new()
        .add(zyn::mark::note("n"))
        .add(zyn::mark::warning("w"))
        .build();
    assert_eq!(d.level(), Level::Warning);
}

#[test]
fn level_warning_beats_help() {
    let d = zyn::mark::new()
        .add(zyn::mark::help("h"))
        .add(zyn::mark::warning("w"))
        .build();
    assert_eq!(d.level(), Level::Warning);
}

#[test]
fn level_all_four_returns_error() {
    let d = zyn::mark::new()
        .add(zyn::mark::note("n"))
        .add(zyn::mark::help("h"))
        .add(zyn::mark::warning("w"))
        .add(zyn::mark::error("e"))
        .build();
    assert_eq!(d.level(), Level::Error);
}

#[test]
fn is_error_with_error() {
    let d = zyn::mark::error("e").build();
    assert!(d.is_error());
}

#[test]
fn is_error_false_when_empty() {
    assert!(!zyn::mark::new().build().is_error());
}

#[test]
fn is_error_false_with_only_warnings() {
    let d = zyn::mark::new().add(zyn::mark::warning("w")).build();
    assert!(!d.is_error());
}

#[test]
fn is_error_false_with_note_and_help() {
    let d = zyn::mark::new()
        .add(zyn::mark::note("n"))
        .add(zyn::mark::help("h"))
        .build();
    assert!(!d.is_error());
}

#[test]
fn is_error_true_with_mixed_levels() {
    let d = zyn::mark::new()
        .add(zyn::mark::warning("w"))
        .add(zyn::mark::note("n"))
        .add(zyn::mark::error("e"))
        .build();
    assert!(d.is_error());
}
