# Extractors

Extractors are types implementing `FromInput` that pull structured data from a proc macro `Input`. Mark element params with `#[zyn(input)]` to auto-resolve them from the input context — they won't be passed at the call site.

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] item: syn::DeriveInput,          // auto-resolved from input
    #[zyn(input)] fields: zyn::Fields,               // auto-resolved from input
    label: syn::Ident,                  // prop — passed at @call site
) -> zyn::TokenStream {
    // item, fields available via Deref; label from self
    zyn::zyn! { /* ... */ }
}
```

All extractors implement `Deref` and `DerefMut` to their inner type, plus an `inner(self)` method to take ownership.

## Categories

- [Input Attribute](./input-attribute.md) — the `#[zyn(input)]` mechanism
- [Attr and Extract](./attr-extract.md) — generic `FromInput` wrappers
- [Fields](./fields.md) — struct field extraction
- [Variants](./variants.md) — enum variant extraction
- [Data](./data.md) — derive data extraction (`syn::DataStruct`, etc.)
- [Element Inputs](./element-inputs.md) — `syn::ItemFn`, `syn::DeriveInput`, `syn::Item`, and all input types
