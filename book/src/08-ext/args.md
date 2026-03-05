# Args

`Args` is always available — no feature flag required.

```rust
use zyn::Args;
```

A parsed, ordered collection of `Arg` values from an attribute's argument list.

```rust
// Given: #[my_attr(skip, rename = "foo", tags("a", "b"))]
let args: Args = attr.args()?;

args.has("skip")       // true — "skip" flag is present
args.get("rename")     // Some(&Arg) — the rename argument
args.len()             // 3
args.is_empty()        // false
args[0]                // first Arg by index

for arg in &args {
    println!("{:?}", arg.name());
}
```

## Querying by Name

`get` returns the first argument with a matching name:

```rust
if let Some(arg) = args.get("rename") {
    let lit = arg.as_lit();    // &syn::Lit
    // or
    let expr = arg.as_expr();  // &syn::Expr
}
```

## Presence Check

`has` tests whether a named argument (flag or keyed) is present, without extracting its value:

```rust
let is_skipped = field.attrs.find_args("my_attr")?.map_or(false, |a| a.has("skip"));
```

## Merging Two `Args`

`merge` combines two collections. Keys from the second override the first:

```rust
// base: skip, mode = "fast"
// override: mode = "slow", verbose
let merged = base_args.merge(&override_args);
// result: skip, mode = "slow", verbose
```

## Iterating

`Args` implements `IntoIterator` and `Index<usize>`:

```rust
for arg in &args {
    match arg {
        Arg::Flag(ident) => println!("flag: {}", ident),
        Arg::Expr(ident, expr) => println!("expr: {} = ...", ident),
        Arg::List(ident, nested) => println!("list: {}(...)", ident),
        Arg::Lit(lit) => println!("lit: {:?}", lit),
    }
}
```
