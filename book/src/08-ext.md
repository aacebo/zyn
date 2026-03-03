# Attribute Extensions

The `ext` module provides helpers for parsing `syn` attributes and their arguments. Enable it with the `ext` feature:

```toml
[dependencies]
zyn = { version = "0.0.0", features = ["ext"] }
```

```rust
use zyn::ext::*;
```

## AttrExt

Extension methods on `syn::Attribute`:

```rust
let attr: &syn::Attribute = /* ... */;

attr.is("serde")          // true if the attribute is named "serde"
let args = attr.args()?;  // parse the attribute's arguments
```

## AttrsExt

Extension methods on `[syn::Attribute]`:

```rust
let attrs: &[syn::Attribute] = /* ... */;

attrs.has_attr("skip")                // true if any attr is named "skip"
attrs.find_attr("rename")             // Option<&Attribute>
let args = attrs.find_args("serde")?; // Option<Args>
let args = attrs.merge_args("zyn")?;  // merge args from all #[zyn(...)] attrs
```

## Args

A container of parsed arguments. Supports querying, indexing, and iteration:

```rust
let args: Args = attr.args()?;

args.has("skip")       // true if "skip" is present
args.get("rename")     // Option<&Arg>
args.len()             // number of arguments
args.is_empty()        // true if no arguments
args[0]                // index access

for arg in &args {
    // iterate over Arg values
}
```

Merge two `Args` collections (the second overrides matching keys):

```rust
let merged = base_args.merge(&override_args);
```

## Arg

Individual argument variants:

| Variant | Syntax | Example |
|---------|--------|---------|
| `Flag` | standalone identifier | `skip` |
| `Expr` | key = value | `rename = "foo"` |
| `List` | nested group | `serde(flatten)` |
| `Lit` | literal value | `"hello"`, `42` |

```rust
let arg: &Arg = args.get("rename").unwrap();

arg.name()       // Some(&Ident) — None for Lit variant
arg.is_flag()    // type predicates
arg.is_expr()
arg.as_expr()    // &Expr — panics if not Expr
arg.as_args()    // &Args — panics if not List
arg.as_lit()     // &Lit — panics if not Lit
```
