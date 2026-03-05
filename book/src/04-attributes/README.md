# derive(Attribute)

`#[derive(zyn::Attribute)]` generates typed extraction for proc macro attributes like `#[my_attr(skip, rename = "foo")]`. Declare the shape as a Rust struct and zyn handles parsing.

## Overview

```rust
use zyn::FromInput;

#[derive(zyn::Attribute)]
#[zyn("builder", about = "Configure the builder derive")]
struct BuilderConfig {
    #[zyn(default = "build".to_string())]
    method: String,
    skip: bool,
    rename: Option<String>,
}

let input: zyn::Input = derive_input.into();
let cfg = BuilderConfig::from_input(&input)?;
```

## Attribute Mode

Add `#[zyn("name")]` to activate attribute mode. The derive searches `input.attrs()` for a matching `#[name(...)]` attribute, parses its arguments, and constructs your struct:

```rust
#[derive(zyn::Attribute)]
#[zyn("serde", unique, about = "Configure serialization")]
struct SerdeConfig {
    #[zyn(0, about = "the input path")]
    path: String,
    #[zyn("rename_all", about = "case transform for keys")]
    casing: Option<String>,
    #[zyn(about = "reject unknown fields")]
    deny_unknown_fields: bool,
    #[zyn(default = "json".to_string(), about = "output format")]
    format: String,
}
```

Generated methods:
- `from_args(args: &Args) -> zyn::Result<Self>` — extract from a parsed `Args`; accumulates all errors
- `from_input(input: &Input) -> zyn::Result<Self>` — implements `FromInput`
- `about() -> &'static str` — human-readable description

### Error Accumulation

`from_args` collects all validation errors and returns them together as `Diagnostics`:

- Missing fields, type mismatches, and unknown keys are all reported at once
- Close misspellings suggest the correct field name via Levenshtein distance
- `about` text is included in error messages as context

## Argument Mode

Without `#[zyn("name")]`, the derive generates only `from_args` and `FromArg`. Used as nested field types inside attribute mode structs:

```rust
#[derive(zyn::Attribute)]
struct Inner {
    a: i64,
    b: String,
}

#[derive(zyn::Attribute)]
#[zyn("outer")]
struct Outer {
    inner: Inner,  // parsed from: outer(inner(a = 1, b = "x"))
}
```

## Struct-level Annotations

| Annotation | Effect |
|---|---|
| `#[zyn("name")]` | Attribute name to match — activates attribute mode |
| `#[zyn(about = "...")]` | Description for `about()` and error messages |
| `#[zyn(unique)]` | Only one occurrence allowed; multiple → error |

Combinable: `#[zyn("serde", unique, about = "Configure serialization")]`

## Field Annotations

| Annotation | Effect |
|---|---|
| `#[zyn(0)]` | Positional: consume `args[0]` |
| `#[zyn("key")]` | Name override: look up `args.get("key")` |
| *(bare field)* | Uses the field's own name |
| `#[zyn(default)]` | Use `Default::default()` when absent |
| `#[zyn(default = expr)]` | Use expression as default (wrapped in `Into::into`) |
| `#[zyn(skip)]` | Always `Default::default()`, never extracted |
| `#[zyn(about = "...")]` | Description for error messages and `about()` |

## Required vs Optional

- Fields without `#[zyn(default)]` or `#[zyn(skip)]` and not `Option<T>` → **required**
- `Option<T>` → always optional (absent → `None`)
- `bool` → always optional (absent → `false`)
- `#[zyn(default)]` → use `Default::default()` when absent
- `#[zyn(default = expr)]` → use expression when absent

## Enum Variants

Enums derive in argument mode — they generate `from_arg` and `FromArg`:

```rust
#[derive(zyn::Attribute)]
enum Mode {
    Fast,
    Slow,
    Custom { speed: i64 },
}

#[derive(zyn::Attribute)]
#[zyn("config")]
struct Config {
    mode: Mode,  // matches: fast | slow | custom(speed = 5)
}
```

Variant dispatch by snake_case name:
- **Unit variants** → `Arg::Flag`: `fast` → `Mode::Fast`
- **Struct variants** → `Arg::List`: `custom(speed = 5)` → `Mode::Custom { speed: 5 }`
- **Single-field tuple** → `Arg::Expr`: `name = "blue"` → `Color::Named("blue".into())`
- **Multi-field tuple** → `Arg::List` with positional args

## Using with Elements

Attribute mode structs implement `FromInput` and can be used as [extractors](../03-elements/extractors.md):

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] cfg: zyn::Attr<MyConfig>,
    name: syn::Ident,
) -> zyn::TokenStream {
    zyn::zyn! { /* cfg.format, cfg.enabled, name all available */ }
}
```
