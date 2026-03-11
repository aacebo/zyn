use zyn::quote::quote;

#[derive(zyn::Attribute)]
#[zyn("my_attr")]
struct TestAttr {
    rename: Option<String>,
}

#[zyn::element]
fn attr_element(
    #[zyn(input)] attr: zyn::Attr<TestAttr>,
    label: zyn::syn::Ident,
) -> zyn::TokenStream {
    let name_str = attr.rename.as_deref().unwrap_or("default");
    let combined = zyn::format_ident!("{}_{}", label, name_str);
    zyn::zyn!(fn {{ combined }}() {})
}

#[test]
fn attr_with_matching_attribute() {
    let input: zyn::Input = zyn::parse!("#[my_attr(rename = \"custom\")] struct Foo;").unwrap();
    let result = zyn::Render::render(
        &AttrElement {
            label: zyn::format_ident!("hello"),
        },
        &input,
    );
    let expected = quote!(
        fn hello_custom() {}
    );
    zyn::assert_tokens!(result, expected);
}

#[test]
fn attr_absent_uses_default() {
    let result = zyn::Render::render(
        &AttrElement {
            label: zyn::format_ident!("hello"),
        },
        &Default::default(),
    );
    let expected = quote!(
        fn hello_default() {}
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn extract_ident_element(#[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>) -> zyn::TokenStream {
    zyn::zyn!(
        const NAME: &str = { { ident | str } };
    )
}

#[test]
fn extract_ident() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let result = zyn::Render::render(&ExtractIdentElement {}, &input);
    let expected = quote!(
        const NAME: &str = "Foo";
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn fields_element(#[zyn(input)] fields: zyn::Fields) -> zyn::TokenStream {
    let count = fields.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn fields() {
    let input: zyn::Input = zyn::parse!("struct Foo { x: u32, y: u32 }").unwrap();
    let result = zyn::Render::render(&FieldsElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn named_fields_element(
    #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
) -> zyn::TokenStream {
    let count = fields.named.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn fields_named() {
    let input: zyn::Input = zyn::parse!("struct Foo { x: u32 }").unwrap();
    let result = zyn::Render::render(&NamedFieldsElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 1usize;
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn variants_element(#[zyn(input)] variants: zyn::Variants) -> zyn::TokenStream {
    let count = variants.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn variants() {
    let input: zyn::Input = zyn::parse!("enum Color { Red, Green, Blue }").unwrap();
    let result = zyn::Render::render(&VariantsElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 3usize;
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn data_struct_element(#[zyn(input)] data: zyn::Data<zyn::syn::DataStruct>) -> zyn::TokenStream {
    let count = data.fields.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn data_struct() {
    let input: zyn::Input = zyn::parse!("struct Foo { x: u32, y: u32 }").unwrap();
    let result = zyn::Render::render(&DataStructElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn derive_input_element(#[zyn(input)] d: zyn::syn::DeriveInput) -> zyn::TokenStream {
    let name = &d.ident;
    zyn::zyn!(
        const NAME: &str = { { name | str } };
    )
}

#[test]
fn derive_input() {
    let input: zyn::Input = zyn::parse!("struct Point { x: f32 }").unwrap();
    let result = zyn::Render::render(&DeriveInputElement {}, &input);
    let expected = quote!(
        const NAME: &str = "Point";
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn data_enum_element(#[zyn(input)] e: zyn::syn::DataEnum) -> zyn::TokenStream {
    let count = e.variants.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn data_enum() {
    let input: zyn::Input = zyn::parse!("enum Dir { North, South }").unwrap();
    let result = zyn::Render::render(&DataEnumElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn data_union_element(#[zyn(input)] u: zyn::syn::DataUnion) -> zyn::TokenStream {
    let count = u.fields.named.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn data_union() {
    let input: zyn::Input = zyn::parse!("union Bits { i: i32, f: f32 }").unwrap();
    let result = zyn::Render::render(&DataUnionElement {}, &input);
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn item_fn_element(#[zyn(input)] item: zyn::syn::ItemFn) -> zyn::TokenStream {
    let name = &item.sig.ident;
    zyn::zyn!(
        const NAME: &str = { { name | str } };
    )
}

#[test]
fn item_fn() {
    let input = zyn::Input::Item(zyn::parse!("fn hello_world() {}").unwrap());
    let result = zyn::Render::render(&ItemFnElement {}, &input);
    let expected = quote!(
        const NAME: &str = "hello_world";
    );
    zyn::assert_tokens!(result, expected);
}

#[zyn::element]
fn item_element(#[zyn(input)] item: zyn::syn::Item) -> zyn::TokenStream {
    let name = match &item {
        zyn::syn::Item::Fn(f) => &f.sig.ident,
        _ => panic!("expected fn"),
    };
    zyn::zyn!(
        const NAME: &str = { { name | str } };
    )
}

#[test]
fn item() {
    let input = zyn::Input::Item(zyn::parse!("fn hello() {}").unwrap());
    let result = zyn::Render::render(&ItemElement {}, &input);
    let expected = quote!(
        const NAME: &str = "hello";
    );
    zyn::assert_tokens!(result, expected);
}
