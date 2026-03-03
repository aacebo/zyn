# Phase 4: Pipes + Pipe Trait

## Scope

Implement the `Pipe` trait, built-in pipes, and the expansion logic for `{{ expr | pipe }}` interpolation. Pipes transform values at macro-expansion time before emitting them as tokens. Users can define custom pipes by implementing the `Pipe` trait.

## Files to Create

- `crates/derive/src/template/pipe.rs` — built-in pipe expansion logic

## Files to Modify

- `src/lib.rs` — add `Pipe` trait

## Pipe Trait (in `src/lib.rs`)

```rust
pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn transform(&self, input: Self::Input) -> Self::Output;
}
```

Pipes are types implementing this trait. The `Input` is what the pipe receives (typically a `String` from `.to_string()`), and `Output` is what gets emitted to the token stream.

Example custom pipe:

```rust
struct Prefix(String);

impl zyn::Pipe for Prefix {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn transform(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(
            &format!("{}_{}", self.0, input),
            proc_macro2::Span::call_site(),
        )
    }
}
```

## Built-in Pipes

Built-in pipes are recognized by name at expansion time and generate inline code (no trait dispatch needed):

| Pipe | Description | Generated code |
|------|-------------|---------------|
| `upper` | Uppercase string | `.to_string().to_uppercase()` |
| `lower` | Lowercase string | `.to_string().to_lowercase()` |
| `snake` | Convert to snake_case | custom conversion logic |
| `camel` | Convert to camelCase | custom conversion logic |
| `pascal` | Convert to PascalCase | custom conversion logic |
| `screaming` | Convert to SCREAMING_SNAKE_CASE | custom conversion logic |

## Design

### Built-in pipe expansion

Built-in pipes are expanded inline without trait dispatch:

```rust
// {{ name | snake }}
{
    let __zyn_val = (#name).to_string();
    let __zyn_val = /* snake_case conversion */;
    let __zyn_ident = ::proc_macro2::Ident::new(&__zyn_val, ::proc_macro2::Span::call_site());
    ::quote::ToTokens::to_tokens(&__zyn_ident, &mut __zyn_ts);
}
```

### Custom pipe expansion

Unknown pipe names are treated as expressions implementing `zyn::Pipe`. The expander generates trait-based dispatch:

```rust
// {{ name | my_pipe }}
{
    let __zyn_val = (#name).to_string();
    let __zyn_val = ::zyn::Pipe::transform(&(my_pipe), __zyn_val);
    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts);
}
```

### Pipe with args

`{{ val | slice:0:5 }}` — pipe args are parsed as token streams. For custom pipes, args are passed to the pipe expression (the pipe "name" could be a constructor call). For built-in pipes, args are handled inline.

### Chained pipes

```rust
// {{ name | snake | upper }}
{
    let __zyn_val = (#name).to_string();
    let __zyn_val = /* snake_case conversion */;
    let __zyn_val = __zyn_val.to_uppercase();
    let __zyn_ident = ::proc_macro2::Ident::new(&__zyn_val, ::proc_macro2::Span::call_site());
    ::quote::ToTokens::to_tokens(&__zyn_ident, &mut __zyn_ts);
}
```

### Case conversion functions

- `snake`: split on case boundaries, join with `_`, lowercase
- `camel`: split on `_` or case boundaries, lowercase first word, capitalize subsequent
- `pascal`: split on `_` or case boundaries, capitalize each word
- `screaming`: snake_case then uppercase

### Expansion entry point

```rust
fn expand_pipes(expr: &TokenStream, pipes: &[Pipe], target: &Ident) -> TokenStream
```

Called by the expander when a `Node::Interpolation` has pipes. Returns generated code that applies pipes and appends result to `target`.

## Acceptance Criteria

- `cargo build --workspace` compiles
- `cargo clippy --workspace --all-features -- -D warnings` passes
- `upper`/`lower` correctly transform strings
- `snake`/`camel`/`pascal`/`screaming` handle mixed-case identifiers
- Custom pipes via `Pipe` trait work with trait dispatch
- Chained pipes apply in sequence
