# `Data<T>`

Extracts the `syn::Data` from a derive input. Generic over `T: FromData`, defaults to `syn::Data`.

Implements `Deref` and `DerefMut` to `T`. Only works with derive inputs — errors on item inputs.

| `T` | Behaviour |
|---|---|
| `syn::Data` (default) | Returns any data kind |
| `syn::DataStruct` | Errors if not a struct |
| `syn::DataEnum` | Errors if not an enum |
| `syn::DataUnion` | Errors if not a union |

```rust
#[zyn::element]
fn struct_data_element(
    #[zyn(input)] data: zyn::Data<syn::DataStruct>,
) -> zyn::TokenStream {
    let count = data.fields.len();
    zyn::zyn!(const FIELD_COUNT: usize = {{ count }};)
}
```

## `FromData` Trait

Implement this trait to create custom data extractors:

```rust
pub trait FromData: Sized {
    fn from_data(data: syn::Data) -> syn::Result<Self>;
}
```

Built-in implementations exist for `syn::Data`, `syn::DataStruct`, `syn::DataEnum`, and `syn::DataUnion`.
