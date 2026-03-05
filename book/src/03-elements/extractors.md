# Extractors

Extractors are types implementing `FromInput` that pull structured data from a proc macro `Input`. Mark element params with `#[zyn(input)]` to auto-resolve them — they won't be passed at the call site.

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] item: syn::DeriveInput,
    #[zyn(input)] fields: zyn::Fields,
    label: syn::Ident,                       // prop — passed at @call site
) -> zyn::TokenStream {
    zyn::zyn! { /* ... */ }
}
```

All wrapper extractors implement `Deref`/`DerefMut` to their inner type and provide an `inner(self) -> T` method.

## Extractor Types

| Type | Extracts | Use Case |
|---|---|---|
| `Attr<T>` | `T::from_input(input)` | `#[derive(Attribute)]` structs |
| `Extract<T>` | `T::from_input(input)` | General `FromInput` wrapper |
| `Fields<T>` | Struct fields | Defaults to `syn::Fields` |
| `Variants` | Enum variants | `Vec<syn::Variant>` |
| `Data<T>` | Derive data | Defaults to `syn::Data` |

## `Attr<T>`

For attribute structs created with `#[derive(Attribute)]`:

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] cfg: zyn::Attr<MyConfig>,
    name: syn::Ident,
) -> zyn::TokenStream {
    if cfg.skip { return zyn::TokenStream::new(); }
    zyn::zyn! { /* ... */ }
}
```

## `Extract<T>`

General-purpose wrapper for any `FromInput` type:

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] generics: zyn::Extract<syn::Generics>,
) -> zyn::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    zyn::zyn! { /* ... */ }
}
```

## `Fields<T>`

Extracts `syn::Fields` from a struct input. Generic over `T: FromFields`:

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
    zyn::zyn!(const COUNT: usize = {{ fields.len() }};)
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

Can also be used outside elements:

```rust
let fields = zyn::Fields::from_input(&input)?;
```

## `Variants`

Extracts `Vec<syn::Variant>` from an enum input. Errors if not an enum:

```rust
#[zyn::element]
fn variant_names(
    #[zyn(input)] variants: zyn::Variants,
) -> zyn::TokenStream {
    zyn::zyn! {
        @for (v in variants.iter()) {
            const {{ v.ident | screaming }}: &str = {{ v.ident | str }};
        }
    }
}
```

## `Data<T>`

Extracts `syn::Data` from a derive input. Generic over `T: FromData`. Only works with derive inputs:

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
    zyn::zyn!(const FIELD_COUNT: usize = {{ data.fields.len() }};)
}
```

## Syn Types as Extractors

`FromInput` is implemented for syn types directly — use them as `#[zyn(input)]` params:

### Derive Input Types

| Type | Matches |
|---|---|
| `syn::DeriveInput` | Any derive input |
| `syn::DataStruct` | Struct data only |
| `syn::DataEnum` | Enum data only |
| `syn::DataUnion` | Union data only |

### Item Input Types

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

### Cross-Input Extraction

`syn::ItemStruct`, `syn::ItemEnum`, and `syn::ItemUnion` also work with derive inputs — zyn reconstructs the item type from the derive data:

```rust
#[zyn::element]
fn struct_element(
    #[zyn(input)] s: syn::ItemStruct,
) -> zyn::TokenStream {
    // Works whether input is Input::Item or Input::Derive
    zyn::zyn! { /* ... */ }
}
```

All other item types require an `Input::Item` context.
