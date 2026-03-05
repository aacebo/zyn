# Arg

`Arg` is always available — no feature flag required.

```rust
use zyn::Arg;
```

An individual parsed argument. Four variants map to the four syntactic forms an attribute argument can take:

| Variant | Syntax | Example |
|---------|--------|---------|
| `Flag` | standalone identifier | `skip` |
| `Expr` | `key = value` | `rename = "foo"` |
| `List` | nested parens | `serde(flatten)` |
| `Lit` | bare literal | `"hello"`, `42` |

## Accessors

```rust
let arg: &Arg = args.get("rename").unwrap();

arg.name()       // Some(&Ident) for Flag/Expr/List; None for Lit
arg.is_flag()    // true if Flag variant
arg.is_expr()    // true if Expr variant
arg.is_list()    // true if List variant
arg.is_lit()     // true if Lit variant

arg.as_expr()    // &syn::Expr — panics if not Expr
arg.as_args()    // &Args     — panics if not List
arg.as_lit()     // &syn::Lit — panics if not Lit
```

## Pattern Matching

You can also match directly:

```rust
match arg {
    Arg::Flag(ident) => {
        // e.g. #[my_attr(skip)]
        println!("flag: {}", ident);
    }
    Arg::Expr(ident, expr) => {
        // e.g. #[my_attr(rename = "new_name")]
        println!("{} = {:?}", ident, expr);
    }
    Arg::List(ident, nested_args) => {
        // e.g. #[my_attr(serde(flatten))]
        println!("{}({} args)", ident, nested_args.len());
    }
    Arg::Lit(lit) => {
        // e.g. #[my_attr("hello")]
        println!("literal: {:?}", lit);
    }
}
```

## Practical Example

Parsing a field-level attribute `#[my_derive(rename = "new_name", skip)]`:

```rust
for field in &input.fields {
    let Some(args) = field.attrs.find_args("my_derive")? else { continue };

    let name = if let Some(arg) = args.get("rename") {
        arg.as_lit().to_token_stream().to_string()
    } else {
        field.ident.as_ref().unwrap().to_string()
    };

    if args.has("skip") {
        continue;
    }

    // emit code using `name` ...
}
```
