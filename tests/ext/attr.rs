use zyn::ext::AttrExt;
use zyn::syn;

#[test]
fn is_matches_name() {
    let input: syn::DeriveInput =
        zyn::parse!("#[serde(rename_all = \"camelCase\")] struct Foo;" => syn::DeriveInput)
            .unwrap();
    let attr = &input.attrs[0];
    assert!(attr.is("serde"));
    assert!(!attr.is("derive"));
}

#[test]
fn get_navigates_nested_meta() {
    let input: syn::DeriveInput =
        zyn::parse!("#[serde(rename_all = \"camelCase\")] struct Foo;" => syn::DeriveInput)
            .unwrap();
    let attr = &input.attrs[0];
    assert!(attr.get("rename_all").is_some());
}

#[test]
fn exists_checks_presence() {
    let input: syn::DeriveInput = zyn::parse!("#[serde(rename_all = \"camelCase\", default)] struct Foo;" => syn::DeriveInput).unwrap();
    let attr = &input.attrs[0];
    assert!(attr.exists("rename_all"));
    assert!(attr.exists("default"));
    assert!(!attr.exists("skip"));
}

#[test]
fn args_parses_attribute_arguments() {
    let input: syn::DeriveInput =
        zyn::parse!("#[builder(skip, method = \"create\")] struct Foo;" => syn::DeriveInput)
            .unwrap();
    let attr = &input.attrs[0];
    let args = attr.args().unwrap();
    assert!(args.has("skip"));
    assert!(args.get("method").is_some());
}

#[zyn::element]
pub fn render_if_serde(attrs: Vec<syn::Attribute>, name: syn::Ident) -> zyn::TokenStream {
    use zyn::ext::AttrExt;

    let has_serde = attrs.iter().any(|a| a.is("serde"));
    zyn::zyn!(
        @if (has_serde) {
            impl {{ name }} {
                fn is_serde(&self) -> bool { true }
            }
        }
    )
}

#[test]
fn element_uses_attr_ext_to_conditionally_render() {
    let input: zyn::Input = zyn::parse!("#[serde] struct Foo;" => syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @render_if_serde(
            attrs = input.attrs().to_vec(),
            name = input.ident().clone(),
        )
    );
    zyn::assert_tokens_contain!(output, "is_serde");
}

#[test]
fn element_skips_when_no_serde() {
    let input: zyn::Input = zyn::parse!("struct Bar;" => syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(
        @render_if_serde(
            attrs = input.attrs().to_vec(),
            name = input.ident().clone(),
        )
    );
    zyn::assert_tokens_empty!(output);
}
