use zyn::Render;
use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
fn bail_on_forbidden(name: syn::Ident) -> zyn::TokenStream {
    if name == "forbidden" {
        bail!("reserved identifier");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_emits_compile_error() {
    let tokens = BailOnForbidden {
        name: zyn::format_ident!("forbidden"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("compile_error") || output.contains("reserved identifier"),
        "expected error diagnostic, got: {output}",
    );
}

#[test]
fn bail_allows_valid_input() {
    let tokens = BailOnForbidden {
        name: zyn::format_ident!("allowed"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("allowed"),
        "expected function output, got: {output}"
    );
}

#[zyn::element]
fn multi_diag(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        error!("name is bad");
        help!("use a different name");
    }
    bail!();
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn error_and_help_accumulate() {
    let tokens = MultiDiag {
        name: zyn::format_ident!("bad"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("name is bad"), "got: {output}");
    assert!(output.contains("use a different name"), "got: {output}");
}

#[test]
fn no_errors_passes_through() {
    let tokens = MultiDiag {
        name: zyn::format_ident!("good"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("good"),
        "expected function output, got: {output}"
    );
}

#[zyn::element]
fn warn_element(name: syn::Ident) -> zyn::TokenStream {
    warn!("this element is deprecated");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn warn_does_not_suppress_output() {
    let tokens = WarnElement {
        name: zyn::format_ident!("my_fn"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("my_fn"),
        "expected function output, got: {output}"
    );
}

#[zyn::element]
fn format_error(name: syn::Ident) -> zyn::TokenStream {
    if name == "foo" {
        bail!("field `{}` is invalid", name);
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_with_format_args() {
    let tokens = FormatError {
        name: zyn::format_ident!("foo"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("foo") && output.contains("invalid"),
        "expected formatted error, got: {output}",
    );
}

#[zyn::element]
fn note_element(name: syn::Ident) -> zyn::TokenStream {
    note!("processing field");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn note_does_not_suppress_output() {
    let tokens = NoteElement {
        name: zyn::format_ident!("my_fn"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(
        output.contains("my_fn"),
        "expected function output, got: {output}"
    );
}
