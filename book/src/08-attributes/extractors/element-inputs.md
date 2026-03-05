# Element Inputs

`FromInput` is implemented for syn types directly — use them as `#[zyn(input)]` element parameters to inject the full typed input.

## Derive Input Types

For derive macros (`#[derive(...)]`):

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] d: zyn::types::DeriveInput,
) -> zyn::TokenStream {
    let name = &d.ident;
    zyn::zyn! { /* ... */ }
}
```

| Type | Matches |
|---|---|
| `zyn::types::DeriveInput` | Any derive input |
| `zyn::types::DataStruct` | Struct data only |
| `zyn::types::DataEnum` | Enum data only |
| `zyn::types::DataUnion` | Union data only |

## Item Input Types

For attribute macros and other item-level macros:

```rust
#[zyn::element]
fn fn_element(
    #[zyn(input)] item: zyn::types::ItemFn,
) -> zyn::TokenStream {
    let name = &item.sig.ident;
    let args = &item.sig.inputs;
    zyn::zyn! { /* ... */ }
}
```

| Type | Matches |
|---|---|
| `zyn::types::Item` | Any item |
| `zyn::types::ItemFn` | `fn` |
| `zyn::types::ItemStruct` | `struct` |
| `zyn::types::ItemEnum` | `enum` |
| `zyn::types::ItemUnion` | `union` |
| `zyn::types::ItemTrait` | `trait` |
| `zyn::types::ItemImpl` | `impl` |
| `zyn::types::ItemType` | `type` |
| `zyn::types::ItemMod` | `mod` |
| `zyn::types::ItemConst` | `const` |
| `zyn::types::ItemStatic` | `static` |
| `zyn::types::ItemUse` | `use` |
| `zyn::types::ItemExternCrate` | `extern crate` |
| `zyn::types::ItemForeignMod` | `extern "C"` |

## Cross-Input Extraction

`zyn::types::ItemStruct`, `zyn::types::ItemEnum`, and `zyn::types::ItemUnion` also work with derive inputs — zyn reconstructs the item type from the derive data:

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] s: zyn::types::ItemStruct,
) -> zyn::TokenStream {
    // Works whether input is Input::Item or Input::Derive
    let name = &s.ident;
    zyn::zyn! { /* ... */ }
}
```

All other item types require an `Input::Item` context and will error on derive inputs.
