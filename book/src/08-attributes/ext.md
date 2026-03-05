# AttrExt and AttrsExt

> **Note:** `AttrExt` and `AttrsExt` are superseded by `#[derive(Attribute)]` and `FromInput`. They remain available for backwards compatibility and for cases where you need low-level `syn::Attribute` access.

These extension traits require the `ext` feature:

```toml
[dependencies]
zyn = { version = "0.0.0", features = ["ext"] }
```

```rust
use zyn::ext::{AttrExt, AttrsExt};
```

## `AttrExt`

Extension methods on a single `syn::Attribute`:

```rust
attr.is("serde")     // true if last path segment == "serde"
attr.args()?         // parses the attribute's argument list as Args
```

## `AttrsExt`

Extension methods on `&[syn::Attribute]`:

```rust
attrs.has_attr("serde")              // bool — any attribute matches name
attrs.find_attr("serde")             // Option<&syn::Attribute>
attrs.find_args("serde")?            // Option<Args>
attrs.merge_args("serde")?           // Args — merges all matching occurrences
```

## Migration

Replace manual `AttrsExt` usage with `#[derive(Attribute)]`:

```rust
// Before:
let args = input.attrs.find_args("my_derive")?.unwrap_or_default();
let skip = args.has("skip");
let rename = args.get("rename").map(|a| a.as_str());

// After:
#[derive(zyn::Attribute)]
#[zyn("my_derive")]
struct MyConfig {
    skip: bool,
    rename: Option<String>,
}

let cfg = MyConfig::from_input(&input)?;
// cfg.skip, cfg.rename
```
