# Defining

Annotate a function with `#[zyn::element]`. Parameters become struct fields; the return type controls whether the element can fail:

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
    fn render(&self) -> zyn::Result {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        Ok(zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, })
    }
}
```

The function name (snake_case) becomes the template directive name. The struct name is the PascalCase equivalent — `field_decl` → `FieldDecl`.

## Return Types

Three return types are supported:

| Return type | Behavior |
|---|---|
| `proc_macro2::TokenStream` | Infallible — most common. No `Ok()` wrapper needed. |
| `zyn::Result` | Fallible — can return `Err(Diagnostic)` with notes and help. |
| `syn::Result<proc_macro2::TokenStream>` | Legacy bridge — still accepted. |

```rust,zyn
// Infallible
#[zyn::element]
fn greeting(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

// Fallible
#[zyn::element]
fn validated(name: proc_macro2::Ident) -> zyn::Result {
    if name == "forbidden" {
        return Err(zyn::Diagnostic::error(name.span(), "reserved name"));
    }
    Ok(zyn::zyn!(fn {{ name }}() {}))
}
```

## Using `quote!` Directly

Elements can use `quote!` alongside or instead of `zyn!`:

```rust,zyn
#[zyn::element]
fn wrapper(name: proc_macro2::Ident, children: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote! {
        pub mod #name {
            #children
        }
    }
}
```
