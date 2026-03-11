use zyn::Render;
use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
fn three_levels() -> zyn::TokenStream {
    error!("first");
    warn!("second");
    note!("third");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn preserves_insertion_order() {
    let output = ThreeLevels.render(&dummy_input()).to_string();
    assert!(output.contains("first"), "got: {output}");
    assert!(output.contains("second"), "got: {output}");
    assert!(output.contains("third"), "got: {output}");
}

#[zyn::element]
fn all_four_levels() -> zyn::TokenStream {
    error!("err");
    warn!("warn");
    note!("note");
    help!("help");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn all_four_levels_accumulate() {
    let output = AllFourLevels.render(&dummy_input()).to_string();
    assert!(output.contains("err"), "got: {output}");
    assert!(output.contains("warn"), "got: {output}");
    assert!(output.contains("note"), "got: {output}");
    assert!(output.contains("help"), "got: {output}");
}

#[zyn::element]
fn error_and_warning() -> zyn::TokenStream {
    error!("from_a");
    warn!("from_b");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn merges_in_order() {
    let output = ErrorAndWarning.render(&dummy_input()).to_string();
    assert!(output.contains("from_a"), "got: {output}");
    assert!(output.contains("from_b"), "got: {output}");
}

#[zyn::element]
fn multiple_errors() -> zyn::TokenStream {
    error!("missing field `x`");
    error!("missing field `y`");
    error!("unknown argument `z`");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn accumulate_multiple_error_sources() {
    let output = MultipleErrors.render(&dummy_input()).to_string();
    assert!(output.contains("missing field `x`"), "got: {output}");
    assert!(output.contains("missing field `y`"), "got: {output}");
    assert!(output.contains("unknown argument `z`"), "got: {output}");
}

#[zyn::element]
fn warn_only() -> zyn::TokenStream {
    warn!("just a warning");
    bail!();
    zyn::zyn!(
        struct Foo {}
    )
}

#[test]
fn bail_without_errors_does_not_stop() {
    let output = WarnOnly.render(&dummy_input()).to_string();
    assert!(
        output.contains("Foo"),
        "expected body output, got: {output}"
    );
}

#[zyn::element]
fn error_then_bail() -> zyn::TokenStream {
    error!("fatal");
    bail!();
    zyn::zyn!(
        struct Foo {}
    )
}

#[test]
fn bail_with_errors_stops() {
    let output = ErrorThenBail.render(&dummy_input()).to_string();
    assert!(output.contains("fatal"), "got: {output}");
    assert!(
        !output.contains("Foo"),
        "body should not appear, got: {output}"
    );
}
