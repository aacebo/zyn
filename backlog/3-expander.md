# Phase 3: Expander

## Scope

Transform a `Template` AST into generated Rust code (`TokenStream`) that builds a `proc_macro2::TokenStream` at macro-expansion time.

## Files to Create

- `crates/derive/src/template/expand.rs`

## Design

The expander generates Rust code that incrementally builds a `TokenStream` using `quote::ToTokens::to_tokens()`.

### Generated code pattern

Each template expansion operates on a mutable `TokenStream` variable. The top-level wrapper:

```rust
{
    let mut __zyn_ts = ::proc_macro2::TokenStream::new();
    // ... expanded nodes append to __zyn_ts ...
    __zyn_ts
}
```

Use unique variable names (`__zyn_ts_0`, `__zyn_ts_1`, etc.) via a counter to avoid collisions in nested scopes.

### Node expansion rules

**`Node::Tokens(ts)`**
```rust
::quote::quote!(#ts).to_tokens(&mut __zyn_ts);
```

**`Node::Interpolation { expr, pipes: [] }`**
```rust
::quote::ToTokens::to_tokens(&(#expr), &mut __zyn_ts);
```

**`Node::Interpolation { expr, pipes }`** (with pipes)
Generate pipe application code — see Phase 4.

**`Node::If { branches, else_body }`**
```rust
if #cond1 {
    // expand body1 into __zyn_ts
} else if #cond2 {
    // expand body2 into __zyn_ts
} else {
    // expand else_body into __zyn_ts
}
```

**`Node::For { binding, iter, body }`**
```rust
for #binding in #iter {
    // expand body into __zyn_ts
}
```

**`Node::Match { expr, arms }`**
```rust
match #expr {
    #pat1 => {
        // expand body1 into __zyn_ts
    },
    #pat2 => {
        // expand body2 into __zyn_ts
    },
}
```

**`Node::Group { delimiter, body }`**
```rust
{
    let mut __zyn_ts_N = ::proc_macro2::TokenStream::new();
    // expand body into __zyn_ts_N
    ::proc_macro2::Group::new(#delimiter, __zyn_ts_N).to_tokens(&mut __zyn_ts);
}
```

Where `#delimiter` maps to `::proc_macro2::Delimiter::Brace` / `Paren` / `Bracket` / `None`.

**`Node::Throw { message }`**
```rust
::core::compile_error!(#message);
```

**`Node::Element { name, props, children: None }`**
```rust
{
    let __zyn_rendered = ::zyn::Render::render(&#name {
        #(#prop_name: #prop_value,)*
    })?;
    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut __zyn_ts);
}
```

**`Node::Element { name, props, children: Some(body) }`**
```rust
{
    let mut __zyn_ts_N = ::proc_macro2::TokenStream::new();
    // expand children body into __zyn_ts_N
    let __zyn_rendered = ::zyn::Render::render(&#name {
        #(#prop_name: #prop_value,)*
        children: __zyn_ts_N,
    })?;
    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut __zyn_ts);
}
```

The children template is rendered first into a `TokenStream`, then passed as the `children` field on the element struct. The element's `render()` implementation can splice it wherever needed.

The `?` operator means the generated code must be inside a function returning `syn::Result`. This is the standard shape for proc macro implementations.

### Fully qualified paths

Per CLAUDE.md, all generated code uses fully qualified paths: `::proc_macro2::`, `::quote::`, `::core::`, `::zyn::`.

## Acceptance Criteria

- `cargo build -p zyn-derive` compiles
- `cargo clippy -p zyn-derive -- -D warnings` passes
- Expander produces valid Rust code for all node types
- Nested groups produce correct delimiter wrapping
- Control flow directives generate proper Rust if/for/match
- Element expansion constructs the struct with props and calls `Render::render()`
- Element with children renders body first and passes as `children` field
