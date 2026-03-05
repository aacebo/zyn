# Attribute Extensions

This chapter covers two related but independent concerns:

- **`Arg` and `Args`** — metadata types for parsing attribute argument lists. Always available; no feature flag required.
- **`AttrExt` and `AttrsExt`** — extension traits on `syn::Attribute` and `[syn::Attribute]`. Require the `ext` feature.

```toml
[dependencies]
zyn = "0.0.0"                          # Arg, Args always available

zyn = { version = "0.0.0", features = ["ext"] }  # also enables AttrExt, AttrsExt
```

```rust
use zyn::{Arg, Args};           // no feature required

#[cfg(feature = "ext")]
use zyn::ext::{AttrExt, AttrsExt};
```
