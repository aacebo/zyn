#[test]
fn empty_emits_empty_tokens() {
    let d = zyn::mark::new().build();
    let tokens = d.emit();
    assert!(tokens.is_empty());
}

#[test]
fn single_error_emits_compile_error() {
    let d = zyn::mark::error("broken input").build();
    let tokens = d.emit();
    let output = tokens.to_string();
    assert!(output.contains("compile_error"));
    assert!(output.contains("broken input"));
}

#[test]
fn multiple_errors_emit_multiple_compile_errors() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("error one"))
        .add(zyn::mark::error("error two"))
        .build();

    let tokens = d.emit();
    let output = tokens.to_string();
    assert!(output.contains("error one"));
    assert!(output.contains("error two"));
}

#[test]
fn mixed_levels_emit_errors() {
    let d = zyn::mark::new()
        .add(zyn::mark::error("err_token"))
        .add(zyn::mark::warning("warn_token"))
        .build();

    let tokens = d.emit();
    let output = tokens.to_string();
    assert!(output.contains("err_token"));
}
