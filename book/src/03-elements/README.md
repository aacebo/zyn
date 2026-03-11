# Elements

Elements are reusable template components. Define them with `#[zyn::element]` and invoke them with `@` in templates.

## Defining

Annotate a function with `#[zyn::element]`. Parameters become struct fields (props); the function must return `zyn::TokenStream`:

```rust,zyn
#[zyn::element]
fn field_decl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    }
}
```

The macro generates a struct and a `Render` impl:

```rust
pub struct FieldDecl {
    pub vis: syn::Visibility,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

impl zyn::Render for FieldDecl {
    fn render(&self, input: &zyn::Input) -> zyn::Output {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
    }
}
```

The function name (snake_case) becomes the template directive name. The struct name is the PascalCase equivalent — `field_decl` → `FieldDecl`.

Elements are always infallible. For error handling, see [Diagnostics](./diagnostics.md).

## Invoking

Reference an element by its snake_case name prefixed with `@`. Props are passed as `name = value` pairs:

```rust,zyn
zyn! {
    @field_decl(
        vis = field.vis.clone(),
        name = field.ident.clone().unwrap(),
        ty = field.ty.clone(),
    )
}
// output: pub age: u32,
```

Prop values are raw Rust expressions — any expression that produces the right type works. Trailing commas are allowed. Elements can be invoked as many times as needed:

```rust,zyn
zyn! {
    @for (field in fields.iter()) {
        @field_decl(
            vis = field.vis.clone(),
            name = field.ident.clone().unwrap(),
            ty = field.ty.clone(),
        )
    }
}
```

## The `input` Parameter

Every element's `render` body has an `input: &zyn::Input` in scope. This is the proc macro input context — the item being annotated:

```rust,zyn
#[zyn::element]
fn my_element(name: syn::Ident) -> zyn::TokenStream {
    let ident = input.ident();
    zyn::zyn! { /* ... */ }
}
```

When using `#[zyn::derive]` or `#[zyn::attribute]`, `input` is provided automatically. For manual usage, define `let input: zyn::Input = ...;` before calling `zyn!`:

```rust
let input: zyn::Input = real_derive_input.into();
zyn::zyn! {
    @my_element(name = ident)
}
```

See [Proc Macro Entry Points](../06-macros/README.md) for the recommended approach.

## Extractor Params

Parameters marked with `#[zyn(input)]` are automatically resolved from the `input` context — they are not props and are not passed at the call site:

```rust,zyn
#[zyn::element]
fn my_element(
    #[zyn(input)] cfg: zyn::Attr<MyConfig>,
    #[zyn(input)] fields: zyn::Fields,
    label: syn::Ident,
) -> zyn::TokenStream {
    zyn::zyn! { /* cfg, fields, label all available */ }
}
```

Call site — only props are passed:

```
@my_element(label = some_ident)
```

See [Extractors](./extractors.md) for the full list of extractor types.
