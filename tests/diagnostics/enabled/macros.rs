use zyn::Render;
use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
fn enabled_bail(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        bail!("not allowed");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_emits_error() {
    let tokens = EnabledBail {
        name: zyn::format_ident!("bad"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("not allowed"), "got: {output}");
}

#[test]
fn bail_allows_valid() {
    let tokens = EnabledBail {
        name: zyn::format_ident!("good"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("good"), "got: {output}");
}

#[zyn::element]
fn enabled_multi(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        error!("name is bad");
        help!("use a different name");
    }
    bail!();
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn error_and_help_accumulate() {
    let tokens = EnabledMulti {
        name: zyn::format_ident!("bad"),
    }
    .render(&dummy_input());
    let output = tokens.to_string();
    assert!(output.contains("name is bad"), "got: {output}");
    assert!(output.contains("use a different name"), "got: {output}");
}

#[zyn::element]
fn warn_with_template(name: syn::Ident) -> zyn::TokenStream {
    warn!("deprecated");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn warn_does_not_block_body() {
    let output = WarnWithTemplate {
        name: zyn::format_ident!("my_fn"),
    }
    .render(&dummy_input())
    .to_string();
    assert!(output.contains("my_fn"), "expected body, got: {output}");
}

#[zyn::element]
fn note_and_help_with_template(name: syn::Ident) -> zyn::TokenStream {
    note!("processing `{}`", name);
    help!("consider adding #[derive(Debug)]");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn note_and_help_do_not_block_body() {
    let output = NoteAndHelpWithTemplate {
        name: zyn::format_ident!("my_fn"),
    }
    .render(&dummy_input())
    .to_string();
    assert!(output.contains("my_fn"), "expected body, got: {output}");
}

#[zyn::element]
fn mixed_non_errors_with_template(name: syn::Ident) -> zyn::TokenStream {
    warn!("field will be removed");
    note!("see migration guide");
    help!("use `new_field` instead");
    zyn::zyn!(
        impl {{ name }} {
            fn validate(&self) -> bool { true }
        }
    )
}

#[test]
fn mixed_non_errors_do_not_block_body() {
    let output = MixedNonErrorsWithTemplate {
        name: zyn::format_ident!("MyStruct"),
    }
    .render(&dummy_input())
    .to_string();
    assert!(output.contains("MyStruct"), "expected body, got: {output}");
    assert!(
        output.contains("validate"),
        "expected body method, got: {output}"
    );
}
