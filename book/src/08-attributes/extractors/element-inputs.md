# Element Inputs

`FromInput` is implemented for syn types directly — use them as `#[zyn(input)]` element parameters to inject the full typed input.

## Derive Input Types

For derive macros (`#[derive(...)]`):

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] d: zyn::syn::DeriveInput,
) -> zyn::proc_macro2::TokenStream {
    let name = &d.ident;
    zyn::zyn! { /* ... */ }
}
```

| Type | Matches |
|---|---|
| `syn::DeriveInput` | Any derive input |
| `syn::DataStruct` | Struct data only |
| `syn::DataEnum` | Enum data only |
| `syn::DataUnion` | Union data only |

## Item Input Types

For attribute macros and other item-level macros:

```rust
#[zyn::element]
fn fn_element(
    #[zyn(input)] item: zyn::syn::ItemFn,
) -> zyn::proc_macro2::TokenStream {
    let name = &item.sig.ident;
    let args = &item.sig.inputs;
    zyn::zyn! { /* ... */ }
}
```

| Type | Matches |
|---|---|
| `syn::Item` | Any item |
| `syn::ItemFn` | `fn` |
| `syn::ItemStruct` | `struct` |
| `syn::ItemEnum` | `enum` |
| `syn::ItemUnion` | `union` |
| `syn::ItemTrait` | `trait` |
| `syn::ItemImpl` | `impl` |
| `syn::ItemType` | `type` |
| `syn::ItemMod` | `mod` |
| `syn::ItemConst` | `const` |
| `syn::ItemStatic` | `static` |
| `syn::ItemUse` | `use` |
| `syn::ItemExternCrate` | `extern crate` |
| `syn::ItemForeignMod` | `extern "C"` |

## Cross-Input Extraction

`syn::ItemStruct`, `syn::ItemEnum`, and `syn::ItemUnion` also work with derive inputs — zyn reconstructs the item type from the derive data:

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] s: zyn::syn::ItemStruct,
) -> zyn::proc_macro2::TokenStream {
    // Works whether input is Input::Item or Input::Derive
    let name = &s.ident;
    zyn::zyn! { /* ... */ }
}
```

All other item types require an `Input::Item` context and will error on derive inputs.
