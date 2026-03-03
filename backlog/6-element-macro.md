# Phase 6: #[element] Attribute Macro

## Scope

Provide a `#[zyn::element]` attribute macro that transforms a function into a element struct + `Render` impl, similar to how Dioxus `#[element]` transforms a function into a element with auto-generated props.

## Files to Create

- `crates/derive/src/element.rs`

## Files to Modify

- `crates/derive/src/lib.rs` — add `#[proc_macro_attribute] pub fn element`
- `src/lib.rs` — re-export `zyn_derive::element` (root crate uses `pub use zyn_core::*` so only derive re-export needed)

## Design

### Input

```rust
#[zyn::element]
fn FieldDecl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    })
}
```

### Generated output

```rust
struct FieldDecl {
    vis: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,
}

impl ::zyn::Render for FieldDecl {
    fn render(&self) -> ::syn::Result<::proc_macro2::TokenStream> {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        Ok(zyn::zyn! {
            {{ vis }} {{ name }}: {{ ty }},
        })
    }
}
```

### Transformation rules

1. Function name → struct name (must start with uppercase or contain underscore)
2. Function parameters → struct fields (preserve types exactly)
3. Function body → `Render::render()` body, with `let` bindings that destructure `&self` for each field
4. Function return type must be `syn::Result<proc_macro2::TokenStream>` (or shorthand — validated at parse time)
5. Visibility of the function is applied to both the struct and the impl

### Children support

If the function has a parameter named `children` with type `proc_macro2::TokenStream`, it becomes the children field:

```rust
#[zyn::element]
fn Wrapper(vis: syn::Visibility, children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} struct Foo {
            {{ children }}
        }
    })
}
```

Usage:

```rust
zyn::zyn! {
    @Wrapper { vis: quote::quote!(pub) } {
        name: String,
        age: u32,
    }
}
```

### Parse strategy

Use `syn::ItemFn` to parse the attributed function. Extract:
- `fn.sig.ident` → struct name
- `fn.sig.inputs` → struct fields (each `FnArg::Typed` becomes a field)
- `fn.sig.output` → validate return type
- `fn.vis` → struct + impl visibility
- `fn.block` → render body

### Error cases

- Function with no parameters → error: "element must have at least one parameter"
- Non-typed parameter (e.g., `self`) → error: "element parameters must be typed"
- Missing return type → error: "element must return syn::Result<proc_macro2::TokenStream>"

## Acceptance Criteria

- `cargo build --workspace` compiles
- `cargo test --workspace` passes
- `cargo clippy --workspace --all-features -- -D warnings` passes
- `#[element]` generates correct struct + Render impl
- Children parameter is handled correctly
- Error messages point at the correct span for invalid input
