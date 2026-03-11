use zyn::ext::{FieldKey, FieldsExt};
use zyn::syn;

#[test]
fn is_named_for_braced_struct() {
    let input: syn::DeriveInput =
        zyn::parse!("struct Foo { x: i32, y: String }" => syn::DeriveInput).unwrap();
    let data = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("expected struct"),
    };
    assert!(data.is_named());
    assert!(!data.is_unnamed());
    assert!(!data.is_unit());
}

#[test]
fn is_unnamed_for_tuple_struct() {
    let input: syn::DeriveInput =
        zyn::parse!("struct Foo(i32, String);" => syn::DeriveInput).unwrap();
    let data = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("expected struct"),
    };
    assert!(data.is_unnamed());
    assert!(!data.is_named());
}

#[test]
fn get_field_by_name() {
    let input: syn::DeriveInput =
        zyn::parse!("struct Foo { name: String, age: u32 }" => syn::DeriveInput).unwrap();
    let data = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("expected struct"),
    };
    let key = FieldKey::from("name");
    assert!(data.get(&key).is_some());
}

#[test]
fn get_field_by_index() {
    let input: syn::DeriveInput =
        zyn::parse!("struct Foo(i32, String);" => syn::DeriveInput).unwrap();
    let data = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("expected struct"),
    };
    assert!(data.get(&FieldKey::from(0usize)).is_some());
    assert!(data.get(&FieldKey::from(1usize)).is_some());
    assert!(data.get(&FieldKey::from(2usize)).is_none());
}

#[test]
fn keyed_iterates_named_fields() {
    let input: syn::DeriveInput =
        zyn::parse!("struct Foo { x: i32, y: String }" => syn::DeriveInput).unwrap();
    let data = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("expected struct"),
    };
    let keys: Vec<String> = data.keyed().map(|(k, _)| k.to_string()).collect();
    assert_eq!(keys, vec!["x", "y"]);
}

#[zyn::element]
pub fn option_getters(fields: syn::Fields) -> zyn::TokenStream {
    use zyn::ext::{FieldsExt, TypeExt};

    let mut tokens = zyn::TokenStream::new();
    for (key, field) in fields.keyed() {
        let ty = &field.ty;
        if field.is_option() {
            let inner = field.inner_type().unwrap();
            tokens.extend(zyn::zyn!(
                fn {{ key | ident:"get_{}" }}(&self) -> Option<&{{ inner }}> {
                    self.{{ key }}.as_ref()
                }
            ));
        } else {
            tokens.extend(zyn::zyn!(
                fn {{ key | ident:"get_{}" }}(&self) -> &{{ ty }} {
                    &self.{{ key }}
                }
            ));
        }
    }
    tokens
}

#[test]
fn element_renders_option_aware_getters() {
    let input: zyn::Input =
        zyn::parse!("struct User { name: String, email: Option<String> }" => syn::DeriveInput)
            .unwrap()
            .into();
    let fields = match input.as_derive().unwrap().data.clone() {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("expected struct"),
    };

    let output = zyn::zyn!(@option_getters(fields = fields));
    zyn::assert_tokens_contain!(output, "get_name");
    zyn::assert_tokens_contain!(output, "get_email");
    zyn::assert_tokens_contain!(output, "as_ref");
}
