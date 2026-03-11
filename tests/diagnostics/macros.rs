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
    zyn::assert_tokens_contain!(output, "reserved identifier");
}

#[test]
fn bail_allows_valid_input() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@bail_on_forbidden(name = zyn::format_ident!("allowed")));
    zyn::assert_tokens_contain!(output, "allowed");
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
    zyn::assert_tokens_contain!(output, "name is bad");
    zyn::assert_tokens_contain!(output, "use a different name");
}

#[test]
fn no_errors_passes_through() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@multi_diag(name = zyn::format_ident!("good")));
    zyn::assert_tokens_contain!(output, "good");
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
    zyn::assert_tokens_contain!(output, "my_fn");
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
    zyn::assert_tokens_contain!(output, "invalid");
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
    zyn::assert_tokens_contain!(output, "my_fn");
}
