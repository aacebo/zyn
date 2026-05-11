use zyn::quote::quote;

// ── Reference parameter tests ─────────────────────────────────────────────────

#[zyn::element]
fn ref_ident_element(#[zyn(input)] ident: &zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(
        const NAME: &str = { { ident | str } };
    )
}

#[test]
fn ref_ident() {
    let input: zyn::Input = zyn::parse!("struct MyStruct;").unwrap();
    let output = zyn::zyn!(@ref_ident_element());
    let expected = quote!(
        const NAME: &str = "MyStruct";
    );
    zyn::assert_tokens!(output, expected);
}

#[zyn::element]
fn ref_generics_element(#[zyn(input)] generics: &zyn::syn::Generics) -> zyn::TokenStream {
    let count = generics.params.len();
    zyn::zyn!(
        const COUNT: usize = { { count } };
    )
}

#[test]
fn ref_generics() {
    let input: zyn::Input = zyn::parse!("struct Foo<A, B, C>;").unwrap();
    let output = zyn::zyn!(@ref_generics_element());
    let expected = quote!(
        const COUNT: usize = 3usize;
    );
    zyn::assert_tokens!(output, expected);
}

#[zyn::element]
fn ref_ident_and_generics_element(
    #[zyn(input)] ident: &zyn::syn::Ident,
    #[zyn(input)] generics: &zyn::syn::Generics,
) -> zyn::TokenStream {
    let name_str = ident.to_string();
    let param_count = generics.params.len();
    zyn::zyn!(
        const NAME: &str = { { name_str | str } };
        const PARAMS: usize = { { param_count } };
    )
}

#[test]
fn ref_ident_and_generics() {
    let input: zyn::Input = zyn::parse!("struct Point<X, Y>;").unwrap();
    let output = zyn::zyn!(@ref_ident_and_generics_element());
    let expected = quote!(
        const NAME: &str = "Point";
        const PARAMS: usize = 2usize;
    );
    zyn::assert_tokens!(output, expected);
}

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
    let output = zyn::zyn!(
        @attr_element(label = zyn::format_ident!("hello"))
    );
    let expected = quote!(
        fn hello_custom() {}
    );
    zyn::assert_tokens!(output, expected);
}

#[test]
fn attr_absent_uses_default() {
    let input: zyn::Input = Default::default();
    let output = zyn::zyn!(
        @attr_element(label = zyn::format_ident!("hello"))
    );
    let expected = quote!(
        fn hello_default() {}
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@extract_ident_element());
    let expected = quote!(
        const NAME: &str = "Foo";
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@fields_element());
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@named_fields_element());
    let expected = quote!(
        const COUNT: usize = 1usize;
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@variants_element());
    let expected = quote!(
        const COUNT: usize = 3usize;
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@data_struct_element());
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@derive_input_element());
    let expected = quote!(
        const NAME: &str = "Point";
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@data_enum_element());
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@data_union_element());
    let expected = quote!(
        const COUNT: usize = 2usize;
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@item_fn_element());
    let expected = quote!(
        const NAME: &str = "hello_world";
    );
    zyn::assert_tokens!(output, expected);
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
    let output = zyn::zyn!(@item_element());
    let expected = quote!(
        const NAME: &str = "hello";
    );
    zyn::assert_tokens!(output, expected);
}
