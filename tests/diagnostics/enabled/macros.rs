use zyn::syn;

fn dummy_input() -> zyn::Input {
    zyn::parse!("struct Test;" => syn::DeriveInput)
        .unwrap()
        .into()
}

#[zyn::element]
pub fn enabled_bail(name: syn::Ident) -> zyn::TokenStream {
    if name == "bad" {
        bail!("not allowed");
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn bail_emits_error() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@enabled_bail(name = zyn::format_ident!("bad")));
    zyn::assert_tokens_contain!(output, "not allowed");
}

#[test]
fn bail_allows_valid() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@enabled_bail(name = zyn::format_ident!("good")));
    zyn::assert_tokens_contain!(output, "good");
}

#[zyn::element]
pub fn enabled_multi(name: syn::Ident) -> zyn::TokenStream {
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
    let output = zyn::zyn!(@enabled_multi(name = zyn::format_ident!("bad")));
    zyn::assert_tokens_contain!(output, "name is bad");
    zyn::assert_tokens_contain!(output, "use a different name");
}

#[zyn::element]
pub fn warn_with_template(name: syn::Ident) -> zyn::TokenStream {
    warn!("deprecated");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn warn_does_not_block_body() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@warn_with_template(name = zyn::format_ident!("my_fn")));
    zyn::assert_tokens_contain!(output, "my_fn");
}

#[zyn::element]
pub fn note_and_help_with_template(name: syn::Ident) -> zyn::TokenStream {
    note!("processing `{}`", name);
    help!("consider adding #[derive(Debug)]");
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn note_and_help_do_not_block_body() {
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@note_and_help_with_template(name = zyn::format_ident!("my_fn")));
    zyn::assert_tokens_contain!(output, "my_fn");
}

#[zyn::element]
pub fn mixed_non_errors_with_template(name: syn::Ident) -> zyn::TokenStream {
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
    let input: zyn::Input = dummy_input();
    let output = zyn::zyn!(@mixed_non_errors_with_template(name = zyn::format_ident!("MyStruct")));
    zyn::assert_tokens_contain!(output, "MyStruct");
    zyn::assert_tokens_contain!(output, "validate");
}
