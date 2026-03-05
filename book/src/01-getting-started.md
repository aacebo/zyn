# Getting Started

## Installation

Add zyn to your proc-macro crate:

```toml
[dependencies]
zyn = "0.0.0"
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `derive` | yes | Enables `#[zyn::element]` and `#[zyn::pipe]` attribute macros |
| `ext` | no | Enables `AttrExt` and `AttrsExt` traits for `syn::Attribute` parsing (`Arg`/`Args` are always available) |

## Basic Usage

Import the prelude and use the `zyn!` macro:

```rust
use zyn::prelude::*;

let name = &input.ident;
let output: zyn::TokenStream = zyn! {
    pub struct {{ name }} {
        id: u64,
    }
};
```

The `zyn!` macro returns a `zyn::TokenStream`. Everything outside `{{ }}` and `@` directives passes through as literal tokens, just like `quote!`.
