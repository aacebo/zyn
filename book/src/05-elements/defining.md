# Defining

Annotate a function with `#[zyn::element]`. Parameters become struct fields (props); the function must return `zyn::TokenStream`:

```rust,zyn
#[zyn::element]
fn field_decl(vis: zyn::types::Visibility, name: zyn::types::Ident, ty: zyn::types::Type) -> zyn::TokenStream {
    zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    }
}
```

The macro generates a struct and a `Render` impl:

```rust
// generated:
pub struct FieldDecl {
    pub vis: zyn::types::Visibility,
    pub name: zyn::types::Ident,
    pub ty: zyn::types::Type,
}

impl zyn::Render for FieldDecl {
    fn render(&self, input: &zyn::Input) -> zyn::TokenStream {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
    }
}
```

The function name (snake_case) becomes the template directive name. The struct name is the PascalCase equivalent — `field_decl` → `FieldDecl`.

Elements are always infallible. Diagnostics (errors, warnings, notes, helps) are expressed using [`@throw`, `@warn`, `@note`, `@help`](./diagnostics.md) inside the template body.

## The `input` Parameter

Every element's `render` body has an `input: &zyn::Input` in scope. This is the proc macro input context — the item being annotated. It provides access to attributes, the item's identifier, generics, and visibility:

```rust,zyn
#[zyn::element]
fn my_element(name: zyn::types::Ident) -> zyn::TokenStream {
    // `input` is always in scope — use it directly
    let ident = input.ident();
    let fields = zyn::Fields::from_input(input).unwrap_or_default();
    zyn::zyn! { /* ... */ }
}
```

When calling `zyn!` from a proc macro, shadow the default `input` with your real input:

```rust
// In your #[proc_macro_derive] or #[proc_macro_attribute]:
let input: zyn::Input = real_derive_input.into();
zyn::zyn! {
    @my_element(name = ident)
}
```

## Extractor Params

Parameters typed as built-in extractor types are **not** struct fields — they are automatically resolved from `input` at render time and are not passed at the call site:

| Type | Extracts |
|---|---|
| `zyn::Attr<T>` | `T::from_input(input)` — for `#[derive(Attribute)]` structs |
| `zyn::Extract<T>` | `T::from_input(input)` — general `FromInput` wrapper |
| `zyn::Fields<T>` | Struct fields from the input item |
| `zyn::Variants` | Enum variants from the input item |
| `zyn::Data<T>` | Re-parses the full input as `T: Parse` |

```rust,zyn
#[derive(zyn::Attribute)]
#[zyn("my_attr")]
struct MyConfig {
    skip: bool,
    rename: Option<String>,
}

#[zyn::element]
fn my_element(
    #[zyn(input)] cfg: zyn::Attr<MyConfig>,        // extractor — resolved from input, not a prop
    #[zyn(input)] fields: zyn::Fields,             // extractor — resolved from input, not a prop
    label: zyn::types::Ident,  // prop — passed at @my_element(label = ...)
) -> zyn::TokenStream {
    // cfg.skip, cfg.rename, fields, label all available via Deref
    zyn::zyn! { /* ... */ }
}
```

Call site — only props are passed:

```
@my_element(label = some_ident)
```

## Using `zyn!` for Element Bodies

Element bodies use `zyn!` for template rendering:

```rust,zyn
#[zyn::element]
fn wrapper(name: zyn::types::Ident, children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::zyn! {
        pub mod {{ name }} {
            {{ children }}
        }
    }
}
```
