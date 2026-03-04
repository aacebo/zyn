# Introduction

<p align="center">
    <img
        src="https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/banner.svg"
        style="border-radius:4px"
        alt="zyn"
    />
</p>

Zyn is a template engine for Rust procedural macros. It lets you write `quote!`-like code with control flow, interpolation pipes, and composable elements.

Rust's `quote!` macro is great for generating code, but it lacks conditionals, loops, and pattern matching. Zyn fills that gap with a template syntax that feels natural inside proc macros:

```rust,zyn
use zyn::prelude::*;

let name = &input.ident;
let is_pub = true;

let output: proc_macro2::TokenStream = zyn! {
    @if (is_pub) { pub }
    fn {{ name | snake }}() {
        println!("hello!");
    }
};

// output: pub fn hello_world() { println!("hello!"); }
```

## Features

- **Interpolation** — `{{ expr }}` inserts any `ToTokens` value
- **Pipes** — `{{ name | snake }}` transforms values inline
- **Control flow** — `@if`, `@for`, `@match`, `@throw`
- **Elements** — reusable template components via `#[zyn::element]`
- **Custom pipes** — define transforms with `#[zyn::pipe]`
- **Case conversion** — built-in `snake`, `camel`, `pascal`, `screaming`, `kebab` pipes and utilities
