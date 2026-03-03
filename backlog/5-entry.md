# Phase 5: Entry Point + Tests

## Scope

Wire everything together: define the `#[proc_macro] pub fn zyn` entry point, finalize `template/mod.rs`, and write integration tests.

## Files to Modify

- `crates/derive/src/lib.rs` — add `#[proc_macro] pub fn zyn(input: TokenStream) -> TokenStream`
- `crates/derive/src/template/mod.rs` — expose `pub fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream`

## Files to Create

- `crates/derive/tests/zyn.rs` — integration tests (per CLAUDE.md: proc macro tests go in `tests/` directory)

## Entry Point

```rust
#[proc_macro]
pub fn zyn(input: TokenStream) -> TokenStream {
    template::expand(input.into()).into()
}
```

`template::expand` parses the input, expands the AST, and wraps in the top-level block:

```rust
pub fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match parse(input) {
        Ok(template) => {
            let expanded = expand_template(&template, &mut 0);
            quote! {
                {
                    let mut __zyn_ts = ::proc_macro2::TokenStream::new();
                    #expanded
                    __zyn_ts
                }
            }
        }
        Err(e) => e.to_compile_error(),
    }
}
```

## Tests

Integration tests that invoke `zyn!` and verify the produced `TokenStream`:

- Passthrough: plain tokens emit unchanged
- Interpolation: `{{ name }}` interpolates a variable
- Interpolation with field access: `{{ item.name }}` works
- Interpolation with pipes: `{{ name | upper }}`
- `@if`/`@else if`/`@else`: conditional code generation
- `@for`: loop over iterator, emit per-item tokens
- `@match`: pattern matching with multiple arms and wildcard
- `@throw`: produces `compile_error!`
- `@Element { prop: value }`: constructs struct, calls `Render::render()`, splices result
- `@Element { props } { children }`: element with children template
- `@path::Element { props }`: element with module path
- Nested directives: `@if` inside `@for`
- Nested elements: `@Element` inside `@for`
- Groups: `{{ expr }}` inside parentheses, brackets, braces

## Re-export from main crate

Update `src/lib.rs` (root zyn crate) to re-export:

```rust
#[cfg(feature = "derive")]
pub use zyn_derive::zyn;
```

## Acceptance Criteria

- `cargo build --workspace` compiles
- `cargo test --workspace` passes all integration tests
- `cargo clippy --workspace --all-features -- -D warnings` passes
- `cargo fmt --check` passes
