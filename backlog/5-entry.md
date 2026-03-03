# Phase 5: Entry Point + Tests ✅

## Status: Complete

## Three-crate architecture

- **`zyn-core`** (`crates/core`) — AST, parsing, expansion, `Expand`/`Render`/`Pipe` traits, built-in pipe structs, `ident` module
- **`zyn-derive`** (`crates/derive`) — `#[proc_macro] pub fn zyn` entry point only, depends on `zyn-core`
- **`zyn`** (root) — re-export facade (`pub use zyn_core::*` + `pub use zyn_derive::zyn`)

`zyn-derive` does NOT depend on `zyn`. Generated code uses `::zyn::` paths resolved at the user's call site.

## Entry Point (`crates/derive/src/lib.rs`)

```rust
#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).into()
}

fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match syn::parse2::<zyn_core::ast::Element>(input) {
        Ok(element) => element.to_token_stream(),
        Err(e) => e.to_compile_error(),
    }
}
```

## Re-export (`src/lib.rs`)

```rust
pub use zyn_core::*;

#[cfg(feature = "derive")]
pub use zyn_derive::zyn;
```

## Tests

Integration tests in `tests/zyn.rs` (17 tests) covering:

- Passthrough (plain tokens, multiple tokens)
- Interpolation (`{{ name }}`)
- Pipes (`{{ name | Upper }}`, `{{ name | Snake }}`, chained)
- `@if`/`@else` conditionals
- `@for` loops
- `@match` pattern matching
- Groups (parenthesized, bracketed)

AST-level tests in `crates/core/tests/`:

- `expand.rs` (2 tests) — token passthrough, interpolation expansion
- `pipes.rs` (8 tests) — per-pipe expansion output, chaining, custom dispatch
- `case_conversion.rs` (18 tests) — all pipe case conversions via `Pipe::pipe()`
