# The `#[zyn(input)]` Attribute

Element parameters marked with `#[zyn(input)]` are automatically resolved from the `Input` context via `FromInput::from_input`. Parameters without this attribute are treated as props and must be passed at the call site.

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] item: zyn::types::DeriveInput,   // extractor — resolved from input
    name: zyn::types::Ident,            // prop — passed at @my_element(name = ...)
) -> zyn::TokenStream {
    zyn::zyn! { /* ... */ }
}
```

Any type that implements `FromInput` can be used as an extractor parameter. This includes:

- All wrapper extractors: `Attr<T>`, `Extract<T>`, `Fields<T>`, `Variants`, `Data<T>`
- All syn input types: `zyn::types::ItemFn`, `zyn::types::ItemStruct`, `zyn::types::DeriveInput`, `zyn::types::DataEnum`, etc.
- Built-in impls: `zyn::types::Ident`, `zyn::types::Generics`, `zyn::types::Visibility`

Multiple extractors can be used in the same element:

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] attr: zyn::Attr<MyConfig>,
    #[zyn(input)] fields: zyn::Fields<zyn::types::FieldsNamed>,
    label: zyn::types::Ident,
) -> zyn::TokenStream {
    zyn::zyn! { /* attr.my_field, fields.named, label all available */ }
}
```
