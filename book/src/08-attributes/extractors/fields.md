# `Fields<T>`

Extracts `syn::Fields` from a struct input. Generic over `T: FromFields`, defaults to `syn::Fields`.

Implements `Deref` and `DerefMut` to `T`.

| `T` | Behaviour |
|---|---|
| `syn::Fields` (default) | Returns any field shape |
| `syn::FieldsNamed` | Errors if not named fields |
| `syn::FieldsUnnamed` | Errors if not unnamed fields |

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] fields: zyn::Fields,
) -> zyn::TokenStream {
    let count = fields.len();
    zyn::zyn!(const COUNT: usize = {{ count }};)
}
```

With a specific field kind:

```rust
#[zyn::element]
fn named_struct_element(
    #[zyn(input)] fields: zyn::Fields<syn::FieldsNamed>,
) -> zyn::TokenStream {
    let names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
    zyn::zyn! { /* ... */ }
}
```

Can also be used outside elements via `FromInput`:

```rust
let fields = zyn::Fields::from_input(&input)?;
for field in fields.iter() { /* ... */ }
```

## `FromFields` Trait

Implement this trait to create custom field-shape extractors:

```rust
pub trait FromFields: Sized {
    fn from_fields(fields: syn::Fields) -> syn::Result<Self>;
}
```

Built-in implementations exist for `syn::Fields`, `syn::FieldsNamed`, and `syn::FieldsUnnamed`.
