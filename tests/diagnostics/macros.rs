use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
pub fn bail_on_forbidden(name: syn::Ident) -> zyn::TokenStream {
    if name == "forbidden" {
        bail!("reserved identifier");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_emits_compile_error() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@bail_on_forbidden(name = zyn::format_ident!("forbidden")));
    let output = output.to_string();
    assert!(
        output.contains("compile_error") || output.contains("reserved identifier"),
        "expected error diagnostic, got: {output}",
    );
}

#[test]
fn bail_allows_valid_input() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@bail_on_forbidden(name = zyn::format_ident!("allowed")));
    assert!(
        output.to_string().contains("allowed"),
        "expected function output, got: {}",
        output
    );
}

#[zyn::element]
pub fn multi_diag(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        error!("name is bad");
        help!("use a different name");
    }
    bail!();
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn error_and_help_accumulate() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@multi_diag(name = zyn::format_ident!("bad")));
    let output = output.to_string();
    assert!(output.contains("name is bad"), "got: {output}");
    assert!(output.contains("use a different name"), "got: {output}");
}

#[test]
fn no_errors_passes_through() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@multi_diag(name = zyn::format_ident!("good")));
    assert!(
        output.to_string().contains("good"),
        "expected function output, got: {}",
        output
    );
}

#[zyn::element]
pub fn warn_element(name: syn::Ident) -> zyn::TokenStream {
    warn!("this element is deprecated");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn warn_does_not_suppress_output() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@warn_element(name = zyn::format_ident!("my_fn")));
    assert!(
        output.to_string().contains("my_fn"),
        "expected function output, got: {}",
        output
    );
}

#[zyn::element]
pub fn format_error(name: syn::Ident) -> zyn::TokenStream {
    if name == "foo" {
        bail!("field `{}` is invalid", name);
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_with_format_args() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@format_error(name = zyn::format_ident!("foo")));
    let output = output.to_string();
    assert!(
        output.contains("foo") && output.contains("invalid"),
        "expected formatted error, got: {output}",
    );
}

#[zyn::element]
pub fn note_element(name: syn::Ident) -> zyn::TokenStream {
    note!("processing field");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn note_does_not_suppress_output() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@note_element(name = zyn::format_ident!("my_fn")));
    assert!(
        output.to_string().contains("my_fn"),
        "expected function output, got: {}",
        output
    );
}
