# Templates

The `zyn!` macro is the core of zyn. It accepts a template and returns a `zyn::TokenStream`:

```rust
let tokens: zyn::TokenStream = zyn! {
    pub struct {{ name }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
};
```

Everything outside `{{ }}` and `@` directives passes through as literal tokens, just like `quote!`.

## Syntax Overview

| Syntax | Purpose |
|---|---|
| `{{ expr }}` | [Interpolation](./interpolation.md) — insert any `ToTokens` value |
| `{{ expr \| pipe }}` | [Pipes](./pipes.md) — transform values (case conversion, formatting) |
| `@if` / `@for` / `@match` | [Control flow](./control-flow.md) — conditionals, loops, pattern matching |
| `@element_name(props)` | [Element invocation](../03-elements/README.md) — reusable template components |
