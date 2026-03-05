# Input

## `Input` Enum

`Input` is the unified proc macro input context — it wraps either a `syn::DeriveInput` or `syn::Item` and provides common accessors:

```rust
pub enum Input {
    Derive(syn::DeriveInput),
    Item(syn::Item),
}

impl Input {
    pub fn attrs(&self) -> &[syn::Attribute];
    pub fn ident(&self) -> &syn::Ident;
    pub fn generics(&self) -> &syn::Generics;
    pub fn vis(&self) -> &syn::Visibility;
}
```

Convert from standard syn types:

```rust
// From a derive macro input:
let input: zyn::Input = zyn::parse_input!(ts as syn::DeriveInput).into();

// From an attribute macro input:
let input: zyn::Input = zyn::parse_input!(ts as syn::Item).into();

// Parse directly:
let input: zyn::Input = zyn::parse!(token_stream)?;
```

`Input` implements `Default`, `Parse`, `ToTokens`, and `syn::spanned::Spanned`.

## `FromInput` Trait

```rust
pub trait FromInput: Sized {
    fn from_input(input: &Input) -> zyn::Result<Self>;
}
```

Returns `zyn::Result<Self>` — an alias for `Result<Self, Diagnostics>`. Errors are accumulated as `Diagnostics` instead of short-circuiting.

Implemented by:

| Type | Extracts |
|---|---|
| `#[derive(Attribute)]` structs | Named attribute from `input.attrs()` |
| `syn::Ident` | `input.ident()` |
| `syn::Generics` | `input.generics()` |
| `syn::Visibility` | `input.vis()` |
| `syn::DeriveInput` | Full derive input |
| `syn::DataStruct` / `DataEnum` / `DataUnion` | Specific derive data variant |
| `syn::Item` | Full item |
| `syn::ItemFn` / `ItemStruct` / etc. | Specific item variant |
| `Fields<T>` | Struct fields |
| `Variants` | Enum variants |
| `Data<T>` | Derive data |
| `Extract<T: FromInput>` | Delegates to `T` |
| `Attr<T: FromInput>` | Delegates to `T` |

## Threading Input Through `zyn!`

Inside `zyn!`, an `input` variable of type `&zyn::Input` is always in scope (defaults to a sentinel value). Shadow it before calling `zyn!` to pass real proc macro context:

```rust
#[proc_macro_derive(MyDerive)]
pub fn my_derive(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: zyn::Input = zyn::parse_input!(ts as syn::DeriveInput).into();

    zyn::zyn! {
        @my_element(name = some_ident)
    }.into()
}
```

Every element's `render(&self, input: &Input)` body also has `input` available directly — no need to pass it as a prop.
