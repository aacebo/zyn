use zyn::quote::quote;

struct Field {
    name: zyn::syn::Ident,
    ty: zyn::syn::Ident,
}

struct Item {
    field: Field,
}

#[test]
fn field_access() {
    let field = Field {
        name: zyn::format_ident!("age"),
        ty: zyn::format_ident!("u32"),
    };
    let result = zyn::zyn!({{ field.name }}: {{ field.ty }});
    let expected = quote!(age: u32);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn nested_field_access() {
    let item = Item {
        field: Field {
            name: zyn::format_ident!("age"),
            ty: zyn::format_ident!("u32"),
        },
    };
    let result = zyn::zyn!({{ item.field.name }}: {{ item.field.ty }});
    let expected = quote!(age: u32);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn method_call() {
    let names = [zyn::format_ident!("foo"), zyn::format_ident!("bar")];
    let result = zyn::zyn!({ { names.len() } });
    let expected = quote!(2usize);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn chained_method_call() {
    let name = "hello_world".to_string();
    let result =
        zyn::zyn!({ { zyn::syn::Ident::new(&name.to_uppercase(), zyn::Span::call_site()) } });
    let expected = quote!(HELLO_WORLD);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn method_call_in_for() {
    let items = vec![
        vec![zyn::format_ident!("a")],
        vec![zyn::format_ident!("b"), zyn::format_ident!("c")],
    ];
    let result = zyn::zyn!(
        @for (item in items) {
            {{ item.len() }},
        }
    );
    let expected = quote!(1usize, 2usize,);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn method_call_in_condition() {
    let items: Vec<i32> = vec![1, 2, 3];
    let result = zyn::zyn!(
        @if (items.is_empty()) { struct Empty; }
        @else { struct NonEmpty; }
    );
    let expected = quote!(
        struct NonEmpty;
    );
    zyn::assert_tokens!(result, expected);
}

#[test]
fn nested_field_with_pipe() {
    let item = Item {
        field: Field {
            name: zyn::format_ident!("hello"),
            ty: zyn::format_ident!("u32"),
        },
    };
    let result = zyn::zyn!({ { item.field.name | upper } });
    let expected = quote!(HELLO);
    zyn::assert_tokens!(result, expected);
}
