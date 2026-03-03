# Phase 7: #[pipe] Attribute Macro

## Scope

Provide a `#[zyn::pipe]` attribute macro that transforms a function into a struct + `Pipe` impl, making it easy to define custom pipes.

## Files to Create

- `crates/derive/src/pipe_macro.rs`

## Files to Modify

- `crates/derive/src/lib.rs` — add `#[proc_macro_attribute] pub fn pipe`
- `src/lib.rs` — re-export `zyn_derive::pipe` (root crate uses `pub use zyn_core::*` so only derive re-export needed)

## Design

### Input

```rust
#[zyn::pipe]
fn prefix(input: String, pre: &str) -> proc_macro2::Ident {
    proc_macro2::Ident::new(
        &format!("{}_{}", pre, input),
        proc_macro2::Span::call_site(),
    )
}
```

### Generated output

```rust
struct Prefix;

impl ::zyn::Pipe for Prefix {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        fn __zyn_pipe_impl(input: String, pre: &str) -> proc_macro2::Ident {
            proc_macro2::Ident::new(
                &format!("{}_{}", pre, input),
                proc_macro2::Span::call_site(),
            )
        }
        __zyn_pipe_impl(input)
    }
}
```

### Transformation rules

1. Function name → unit struct name (PascalCase, e.g. `Prefix`, `Snake`, `Upper`)
2. First parameter → `Pipe::Input` type, passed as the pipe input
3. Additional parameters → pipe args (from `:arg` syntax in template), passed at expansion time
4. Return type → `Pipe::Output` (must implement `ToTokens`)
5. Function body → `Pipe::transform()` body

### Pipe args

For pipes with arguments like `{{ name | Prefix:"my" }}`, the additional function parameters beyond the first are the pipe args. The expander passes them to the constructed pipe or directly to the generated code:

```rust
// {{ name | Prefix:"my" }}
{
    let __zyn_val = (#name).to_string();
    let __zyn_val = ::zyn::Pipe::pipe(&Prefix, __zyn_val, "my");
    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts);
}
```

Note: if the `Pipe` trait needs to support args, the trait signature may need adjustment. Alternative: the `#[pipe]` macro generates a function rather than a trait impl, and the expander calls it directly.

### Alternative: simple function approach

Instead of generating a `Pipe` trait impl, `#[pipe]` could simply validate the function signature and leave it as a function. The expander would call `pipe_name(input, args...)` directly:

```rust
// {{ name | Prefix:"my" }}
{
    let __zyn_val = (#name).to_string();
    let __zyn_val = Prefix(__zyn_val, "my");
    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts);
}
```

This is simpler and avoids complexity around trait generics with variable args. The `#[pipe]` macro just validates the signature and generates nothing extra — the function IS the pipe.

### Error cases

- Function with no parameters → error: "pipe must have at least one input parameter"
- Missing return type → error: "pipe must have an explicit return type"

## Acceptance Criteria

- `cargo build --workspace` compiles
- `cargo test --workspace` passes
- `cargo clippy --workspace --all-features -- -D warnings` passes
- Custom pipes defined with `#[pipe]` work in `{{ expr | PipeName }}` syntax
- Pipe args from `:arg` syntax are passed to additional parameters
- Error messages point at the correct span for invalid input
