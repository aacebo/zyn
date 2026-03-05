# Expansion Model

Every `zyn!(...)` invocation expands to a single Rust **block expression** that evaluates to `zyn::TokenStream`:

```rust
{
    let mut __zyn_ts_0 = ::zyn::TokenStream::new();
    // ... expanded nodes writing to __zyn_ts_0 ...
    __zyn_ts_0
}
```

## Variable Names

An incrementing counter allocates unique identifiers across the expansion:

| Identifier | Purpose |
|---|---|
| `__zyn_ts_0` | Root output accumulator |
| `__zyn_ts_1`, `__zyn_ts_2`, … | Inner accumulators for groups and element children |
| `__zyn_val` | Pipe chain intermediate (scoped inside `{}`) |
| `__zyn_rendered` | Result of `Render::render(...)` inside an element call |

Nested structures each get a fresh `__zyn_ts_N`, allocated sequentially from the same counter.

## Node Expansions

### TokensNode

```rust
output.extend(quote!( <stream> ));
```

### InterpNode (no pipes)

```rust
ToTokens::to_tokens(&( expr ), &mut output);
```

### InterpNode (with pipes)

```rust
{
    let __zyn_val = ( expr ).to_string();
    let __zyn_val = ::zyn::Pipe::pipe(&( Pipe1 ), __zyn_val);
    let __zyn_val = __zyn_val.to_string();   // re-stringify between pipes
    let __zyn_val = ::zyn::Pipe::pipe(&( Pipe2 ), __zyn_val);
    ToTokens::to_tokens(&__zyn_val, &mut output);
}
```

The re-stringify step happens between every pipe, so each pipe in a chain always receives a `String`.

### @if / @for / @match

Expand to native Rust `if` / `for` / `match` that write directly to the enclosing accumulator. No intermediate collection; loop body nodes append to `output` on each iteration.

### @throw / @warn / @note / @help

Delegate to `proc-macro2-diagnostics` via `Diagnostic::spanned(...).emit_as_item_tokens()`. On nightly, proper `proc_macro::Diagnostic` emission is used. On stable, errors fall back to `compile_error!`.

### @element (without children)

```rust
{
    let __zyn_rendered = ::zyn::Render::render(&Name {
        prop: value,
    });
    ToTokens::to_tokens(&__zyn_rendered, &mut output);
}
```

### @element (with children)

```rust
{
    let mut __zyn_ts_N = ::zyn::TokenStream::new();
    // children Element expanded into __zyn_ts_N
    let __zyn_rendered = ::zyn::Render::render(&Name {
        prop: value,
        children: __zyn_ts_N,
    });
    ToTokens::to_tokens(&__zyn_rendered, &mut output);
}
```

## Control Flow

`@if`, `@for`, and `@match` are lowered to native Rust control flow inside the accumulator block. For example:

```rust
// @if (cond) { ... }
if cond {
    // expanded body writes to output
}

// @for (x in iter) { ... }
for x in iter {
    // expanded body writes to output
}
```
