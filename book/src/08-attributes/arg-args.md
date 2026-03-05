# Arg and Args

`Arg` and `Args` are the low-level attribute argument primitives. They are always available — no feature flag required. They are used internally by `#[derive(Attribute)]` and are available for manual attribute parsing when needed.

> For most use cases, `#[derive(Attribute)]` is the recommended API. Use `Arg`/`Args` when you need fine-grained control over argument parsing.

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

A parsed, ordered collection of `Arg` values. Parse from an attribute's argument list:

```rust
use zyn::Args;

// Given: #[my_attr(skip, rename = "foo")]
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

Implemented for all scalar types. Used by `#[derive(Attribute)]` generated code. Can also be used directly:

```rust
let s: String = String::from_arg(args.get("name").unwrap())?;
let n: i64 = i64::from_arg(args.get("count").unwrap())?;
```
