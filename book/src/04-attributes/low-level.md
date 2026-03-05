# Low-Level API

## `Arg`

An individual parsed argument. Four variants map to the four syntactic forms:

| Variant | Syntax | Example |
|---------|--------|---------|
| `Flag` | standalone identifier | `skip` |
| `Expr` | `key = value` | `rename = "foo"` |
| `List` | nested parens | `serde(flatten)` |
| `Lit` | bare literal | `"hello"`, `42` |

```rust
use zyn::Arg;

let arg: &Arg = args.get("rename").unwrap();

arg.name()       // Some(&Ident) for Flag/Expr/List; None for Lit
arg.is_flag()    // true if Flag variant
arg.is_expr()    // true if Expr variant
arg.is_list()    // true if List variant
arg.is_lit()     // true if Lit variant

arg.as_flag()    // &Ident     — panics if not Flag
arg.as_expr()    // &syn::Expr — panics if not Expr
arg.as_args()    // &Args      — panics if not List
arg.as_lit()     // &syn::Lit  — panics if not Lit
arg.as_str()     // String     — panics if not a string literal
arg.as_int::<i64>() // T      — panics if not an integer literal
```

Match directly:

```rust
match arg {
    Arg::Flag(ident) => { /* #[my_attr(skip)] */ }
    Arg::Expr(ident, expr) => { /* #[my_attr(rename = "foo")] */ }
    Arg::List(ident, nested_args) => { /* #[my_attr(serde(flatten))] */ }
    Arg::Lit(lit) => { /* #[my_attr("hello")] */ }
}
```

## `Args`

A parsed, ordered collection of `Arg` values:

```rust
use zyn::Args;

let args: Args = zyn::parse!("skip, rename = \"foo\"")?;

args.has("skip")    // true
args.get("rename")  // Some(&Arg)
args.len()          // 2
args[0]             // first Arg
```

`merge` combines two `Args` — keys from the second override the first:

```rust
let merged = base_args.merge(&override_args);
```

## `FromArg` Trait

```rust
pub trait FromArg: Sized {
    fn from_arg(arg: &Arg) -> zyn::Result<Self>;
}
```

Implemented for: `bool`, `String`, `i8`–`i128`, `u8`–`u128`, `f32`, `f64`, `char`, `syn::Ident`, `syn::Path`, `syn::Expr`, `syn::LitStr`, `syn::LitInt`, `Option<T>`, `Vec<T>`, `Args`.

> For most use cases, [`#[derive(Attribute)]`](./README.md) is the recommended API. Use `Arg`/`Args` when you need fine-grained control.

## `AttrExt` and `AttrsExt`

> These extension traits are superseded by `#[derive(Attribute)]` and `FromInput`. They remain available for backwards compatibility and for cases where you need low-level `syn::Attribute` access.

Requires the `ext` feature:

```toml
[dependencies]
zyn = { version = "0.1.0", features = ["ext"] }
```

```rust
use zyn::ext::{AttrExt, AttrsExt};
```

### `AttrExt`

Extension methods on a single `syn::Attribute`:

```rust
attr.is("serde")     // true if last path segment == "serde"
attr.args()?         // parses the attribute's argument list as Args
```

### `AttrsExt`

Extension methods on `&[syn::Attribute]`:

```rust
attrs.has_attr("serde")              // bool
attrs.find_attr("serde")             // Option<&syn::Attribute>
attrs.find_args("serde")?            // Option<Args>
attrs.merge_args("serde")?           // Args — merges all matching occurrences
```

### Migration

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
```
