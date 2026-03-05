# `Data<T>`

Extracts the `zyn::types::Data` from a derive input. Generic over `T: FromData`, defaults to `zyn::types::Data`.

Implements `Deref` and `DerefMut` to `T`. Only works with derive inputs — errors on item inputs.

| `T` | Behaviour |
|---|---|
| `zyn::types::Data` (default) | Returns any data kind |
| `zyn::types::DataStruct` | Errors if not a struct |
| `zyn::types::DataEnum` | Errors if not an enum |
| `zyn::types::DataUnion` | Errors if not a union |

```rust
#[zyn::element]
fn struct_data_element(
    #[zyn(input)] data: zyn::Data<zyn::types::DataStruct>,
) -> zyn::TokenStream {
    let count = data.fields.len();
    zyn::zyn!(const FIELD_COUNT: usize = {{ count }};)
}
```

## `FromData` Trait

Implement this trait to create custom data extractors:

```rust
pub trait FromData: Sized {
    fn from_data(data: zyn::types::Data) -> syn::Result<Self>;
}
```

Built-in implementations exist for `zyn::types::Data`, `zyn::types::DataStruct`, `zyn::types::DataEnum`, and `zyn::types::DataUnion`.
