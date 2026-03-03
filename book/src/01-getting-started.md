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
| `ext` | no | Enables `zyn::ext` module for syn attribute parsing helpers |

## Basic Usage

Import the prelude and use the `zyn!` macro:

```rust
use zyn::prelude::*;

let name = quote::format_ident!("my_struct");
let output: proc_macro2::TokenStream = zyn! {
    pub struct {{ name }} {
        id: u64,
    }
};
```

The `zyn!` macro returns a `proc_macro2::TokenStream`. Everything outside `{{ }}` and `@` directives passes through as literal tokens, just like `quote!`.
