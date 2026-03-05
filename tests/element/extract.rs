use zyn::quote::quote;
use zyn::syn;

#[derive(zyn::Attribute)]
#[zyn("my_attr")]
struct TestAttr {
    rename: Option<String>,
}

#[zyn::element]
fn attr_element(
    #[zyn(input)] attr: zyn::Attr<TestAttr>,
    label: zyn::proc_macro2::Ident,
) -> zyn::proc_macro2::TokenStream {
    let name_str = attr.rename.as_deref().unwrap_or("default");
    let combined = zyn::quote::format_ident!("{}_{}", label, name_str);
    zyn::zyn!(fn {{ combined }}() {})
}

#[test]
fn attr_with_matching_attribute() {
    let input: zyn::Input = syn::parse_str("#[my_attr(rename = \"custom\")] struct Foo;").unwrap();
    let result = zyn::Render::render(
        &AttrElement {
            label: zyn::quote::format_ident!("hello"),
        },
        &input,
    );
    let expected = quote!(
        fn hello_custom() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn attr_absent_uses_default() {
    let result = zyn::Render::render(
        &AttrElement {
            label: zyn::quote::format_ident!("hello"),
        },
        &Default::default(),
    );
    let expected = quote!(
        fn hello_default() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn extract_ident_element(
    #[zyn(input)] ident: zyn::Extract<zyn::proc_macro2::Ident>,
) -> zyn::proc_macro2::TokenStream {
    zyn::zyn!(
        const NAME: &str = { { ident | str } };
    )
}

#[test]
fn extract_ident() {
    let input: zyn::Input = syn::parse_str("struct Foo;").unwrap();
    let result = zyn::Render::render(&ExtractIdentElement {}, &input);
    let expected = quote!(
        const NAME: &str = "Foo";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn fields_element(#[zyn(input)] fields: zyn::Fields) -> zyn::proc_macro2::TokenStream {
    let count = fields.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn fields() {
    let input: zyn::Input = syn::parse_str("struct Foo { x: u32, y: u32 }").unwrap();
    let result = zyn::Render::render(&FieldsElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn named_fields_element(
    #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
) -> zyn::proc_macro2::TokenStream {
    let count = fields.named.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn fields_named() {
    let input: zyn::Input = syn::parse_str("struct Foo { x: u32 }").unwrap();
    let result = zyn::Render::render(&NamedFieldsElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 1usize;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn variants_element(#[zyn(input)] variants: zyn::Variants) -> zyn::proc_macro2::TokenStream {
    let count = variants.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn variants() {
    let input: zyn::Input = syn::parse_str("enum Color { Red, Green, Blue }").unwrap();
    let result = zyn::Render::render(&VariantsElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 3usize;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn data_struct_element(
    #[zyn(input)] data: zyn::Data<zyn::syn::DataStruct>,
) -> zyn::proc_macro2::TokenStream {
    let count = data.fields.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn data_struct() {
    let input: zyn::Input = syn::parse_str("struct Foo { x: u32, y: u32 }").unwrap();
    let result = zyn::Render::render(&DataStructElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn derive_input_element(#[zyn(input)] d: zyn::syn::DeriveInput) -> zyn::proc_macro2::TokenStream {
    let name = &d.ident;
    zyn::zyn!(
        const NAME: &str = { { name | str } };
    )
}

#[test]
fn derive_input() {
    let input: zyn::Input = syn::parse_str("struct Point { x: f32 }").unwrap();
    let result = zyn::Render::render(&DeriveInputElement {}, &input);
    let expected = quote!(
        const NAME: &str = "Point";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn data_enum_element(#[zyn(input)] e: zyn::syn::DataEnum) -> zyn::proc_macro2::TokenStream {
    let count = e.variants.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn data_enum() {
    let input: zyn::Input = syn::parse_str("enum Dir { North, South }").unwrap();
    let result = zyn::Render::render(&DataEnumElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn data_union_element(#[zyn(input)] u: zyn::syn::DataUnion) -> zyn::proc_macro2::TokenStream {
    let count = u.fields.named.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn data_union() {
    let input: zyn::Input = syn::parse_str("union Bits { i: i32, f: f32 }").unwrap();
    let result = zyn::Render::render(&DataUnionElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn item_fn_element(#[zyn(input)] item: zyn::syn::ItemFn) -> zyn::proc_macro2::TokenStream {
    let name = &item.sig.ident;
    zyn::zyn!(
        const NAME: &str = { { name | str } };
    )
}

#[test]
fn item_fn() {
    let input = zyn::Input::Item(syn::parse_str("fn hello_world() {}").unwrap());
    let result = zyn::Render::render(&ItemFnElement {}, &input);
    let expected = quote!(
        const NAME: &str = "hello_world";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn item_element(#[zyn(input)] item: zyn::syn::Item) -> zyn::proc_macro2::TokenStream {
    let name = match &item {
        syn::Item::Fn(f) => &f.sig.ident,
        _ => panic!("expected fn"),
    };
    zyn::zyn!(
        const NAME: &str = { { name | str } };
    )
}

#[test]
fn item() {
    let input = zyn::Input::Item(syn::parse_str("fn hello() {}").unwrap());
    let result = zyn::Render::render(&ItemElement {}, &input);
    let expected = quote!(
        const NAME: &str = "hello";
    );
    assert_eq!(result.to_string(), expected.to_string());
}
