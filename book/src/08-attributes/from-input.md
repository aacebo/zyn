# FromInput and Input

## `Input`

`Input` is the unified proc macro input context — it wraps either a `zyn::types::DeriveInput` or `zyn::types::Item` and provides common accessors:

```rust
pub enum Input {
    Derive(zyn::types::DeriveInput),
    Item(zyn::types::Item),
}

impl Input {
    pub fn attrs(&self) -> &[zyn::types::Attribute];
    pub fn ident(&self) -> &zyn::types::Ident;
    pub fn generics(&self) -> &zyn::types::Generics;
    pub fn vis(&self) -> &zyn::types::Visibility;
}
```

Convert from standard syn types:

```rust
// From a derive macro input:
let input: zyn::Input = zyn::parse_input!(ts as zyn::types::DeriveInput).into();

// From an attribute macro input:
let input: zyn::Input = zyn::parse_input!(ts as zyn::types::Item).into();

// Parse directly:
let input: zyn::Input = zyn::parse!(token_stream)?;
```

`Input` implements `Default` (returns an empty sentinel struct), `Parse`, and `ToTokens`.

## `FromInput` Trait

```rust
pub trait FromInput: Sized {
    type Error: Into<syn::Error>;
    fn from_input(input: &Input) -> Result<Self, Self::Error>;
}
```

Implemented by:

| Type | Extracts |
|---|---|
| `#[derive(Attribute)]` structs | Named attribute from `input.attrs()` |
| `zyn::types::Ident` | `input.ident()` |
| `zyn::types::Generics` | `input.generics()` |
| `zyn::types::Visibility` | `input.vis()` |
| `zyn::types::DeriveInput` | Full derive input |
| `zyn::types::DataStruct` / `DataEnum` / `DataUnion` | Specific derive data variant |
| `zyn::types::Item` | Full item |
| `zyn::types::ItemFn` / `ItemStruct` / etc. | Specific item variant |
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
    let input: zyn::Input = zyn::parse_input!(ts as zyn::types::DeriveInput).into();

    // `input` is now in scope for all elements called inside zyn!
    zyn::zyn! {
        @my_element(name = some_ident)
    }.into()
}
```

Every element's `render(&self, input: &Input)` body also has `input` available directly — no need to pass it as a prop.
