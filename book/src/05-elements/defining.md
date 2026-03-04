# Defining

Annotate a function with `#[zyn::element]`. Parameters become struct fields; the function must return `proc_macro2::TokenStream`:

```rust,zyn
#[zyn::element]
fn field_decl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> proc_macro2::TokenStream {
    zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    }
}
```

The macro generates a struct and a `Render` impl:

```rust
// generated:
pub struct FieldDecl {
    pub vis: syn::Visibility,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

impl zyn::Render for FieldDecl {
    fn render(&self) -> proc_macro2::TokenStream {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
    }
}
```

The function name (snake_case) becomes the template directive name. The struct name is the PascalCase equivalent — `field_decl` → `FieldDecl`.

Elements are always infallible. Diagnostics (errors, warnings, notes, helps) are expressed using [`@throw`, `@warn`, `@note`, `@help`](./diagnostics.md) inside the template body.

## Using `zyn!` for Element Bodies

Element bodies use `zyn!` for template rendering:

```rust,zyn
#[zyn::element]
fn wrapper(name: proc_macro2::Ident, children: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    zyn::zyn! {
        pub mod {{ name }} {
            {{ children }}
        }
    }
}
```
