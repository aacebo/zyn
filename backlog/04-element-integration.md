# Phase 4: `#[element]` + `FromInput` Extractors

## Goal

`#[element]` parameters whose types implement `FromInput` are automatically populated from the proc macro's `Input` context — no annotations required. Plain-typed parameters remain as props. Depends on Phase 2.

## The Pattern

Inspired by Actix Web's `FromRequest` extractors. Each parameter type either:
- Implements `FromInput` → `#[element]` calls `T::from_input(&ctx)` in `render()`, not a struct field
- Does not implement `FromInput` → plain prop, becomes a struct field filled at the call site

## Usage

```rust
#[derive(Attribute)]
#[zyn("my_attr")]
struct MyAttr {
    rename: Option<String>,
    skip: bool,
}

#[zyn::element]
fn example(
    my_attr: MyAttr,              // FromInput → extracted automatically
    label: String,                // no FromInput → plain prop: @example(label = val)
) -> zyn::proc_macro2::TokenStream {
    zyn::zyn! { ... }
}
```

## What `#[element]` generates

**Struct fields:** only plain prop params (e.g. `label: String`). A hidden `__input: ::zyn::Input` field is also added to carry the extraction context.

**`render()` body:** extractor params are resolved before binding:

```rust
impl ::zyn::Render for Example {
    fn render(&self) -> ::zyn::proc_macro2::TokenStream {
        let my_attr = <MyAttr as ::zyn::FromInput>::from_input(&self.__input)
            .unwrap_or_else(|e| return e.into().to_compile_error());
        let label = &self.label;
        // element body follows
    }
}
```

## How `__input` is populated

The element call site (`@example(label = val)`) exists inside a proc macro expansion. The generated struct gains a hidden `__input: ::zyn::Input` field populated with the outer macro's input context. `element_node.rs` threads the input through when constructing element structs.

## Files to Create / Modify

| File | Change |
|---|---|
| `crates/core/src/input/mod.rs` | Add `Input` enum (wraps `DeriveInput` \| `ItemInput`) with `.attrs()`, `.ident()`, `.generics()` |
| `crates/core/src/extract.rs` | **New** — `FromInput` trait |
| `crates/core/src/lib.rs` | `pub mod extract; pub use extract::*;` |
| `crates/derive/src/element.rs` | Detect `FromInput`-implementing params vs props; add hidden `__input` field; generate extractor calls in `render()` |
| `crates/core/src/ast/at/element_node.rs` | Thread `Input` context into `__input` field when constructing element structs |

## Tests

- `#[derive(Attribute)]` attribute mode struct as element param → extracted via `FromInput`, not a prop
- Plain-typed param → remains a prop, unchanged behavior
- Extraction failure at expand time → emits `compile_error!`
- Element with only props (no `FromInput` params) → unchanged behavior
