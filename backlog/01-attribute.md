# Phase 1: `Attribute` Trait + `#[derive(Attribute)]`

## Goal

A single `Attribute` trait handles typed extraction from `Arg`/`Args` and from `&[syn::Attribute]`. `#[derive(Attribute)]` generates the extraction logic for structs (attribute mode or argument mode) and enums (discriminated union argument types). Supports positional args, recursive nesting, `about` descriptions, and any `syn::Parse` type as a field type.

## What This Replaces

```rust
// Before: verbose, error-prone
let args = input.attrs.find_args("my_derive")?.unwrap_or_default();
let skip = args.has("skip");
let rename = match args.get("rename") {
    Some(arg) => {
        if let Arg::Expr(_, expr) = arg {
            if let syn::Expr::Lit(lit) = expr {
                if let syn::Lit::Str(s) = &lit.lit {
                    Some(s.value())
                } else { None }
            } else { None }
        } else { None }
    }
    None => None,
};
```

```rust
// After: declare the shape, derive the extraction
#[derive(Attribute)]
#[zyn("my_derive", about = "Configure my derive")]
struct MyConfig {
    skip: bool,
    rename: Option<String>,
    #[zyn(default = 3)]
    count: i64,
}

let config = MyConfig::attribute(&input.attrs)?;
```

## The `Attribute` Trait

Defined in `zyn-core`:

```rust
pub trait Attribute: Sized {
    fn from_args(args: &Args) -> syn::Result<Self>;
    fn from_arg(arg: &Arg) -> syn::Result<Self>;
    fn attribute(attrs: &[syn::Attribute]) -> syn::Result<Self>;
}
```

- `from_args` — extract from `&Args` directly (core extraction logic)
- `from_arg` — extract from a single `&Arg` (for nested `List` args and scalar values)
- `attribute` — find matching attribute(s) by the name specified in `#[zyn("name")]`, parse args, call `from_args`

## Two Derive Modes

The presence of `#[zyn("name")]` at the struct level determines the mode:

### Attribute mode

Struct has `#[zyn("name", ...)]`. Generates all three methods (`from_args`, `from_arg`, `attribute`) plus `about()`. Struct-level annotations set the rules, field-level annotations can override per-field.

```rust
#[derive(Attribute)]
#[zyn("serde", unique, about = "Configure serialization")]
struct SerdeConfig {
    #[zyn(0, about = "the input path")]
    path: String,
    #[zyn("rename_all", about = "case transform for keys")]
    casing: Option<String>,
    #[zyn(about = "reject unknown fields")]
    deny_unknown_fields: bool,
    #[zyn(default = "json", about = "output format")]
    format: String,
}

let config = SerdeConfig::attribute(&input.attrs)?;
```

### Argument mode

Struct has no `#[zyn("name")]` (or is an enum). Generates only `from_args` and `from_arg`. Only field-level annotations apply. Used as nested types within attribute structs.

```rust
#[derive(Attribute)]
struct Inner {
    a: i64,
    b: String,
}

#[derive(Attribute)]
#[zyn("outer")]
struct Outer {
    inner: Inner,  // parsed from: outer(inner(a = 1, b = "x"))
}
```

## Arg Enhancements

Add typed accessor methods to the existing `Arg` enum following the existing panicking accessor pattern:

| Method | Returns | From variant |
|---|---|---|
| `as_flag(&self) -> &Ident` | The flag ident | `Flag` |
| `as_str(&self) -> String` | String value | `Expr` (string lit) or `Lit` (string) |
| `as_int<T: FromStr>(&self) -> T` | Parsed integer | `Expr` (int lit) or `Lit` (int) |
| `as_float<T: FromStr>(&self) -> T` | Parsed float | `Expr` (float lit) or `Lit` (float) |
| `as_char(&self) -> char` | Char value | `Expr` (char lit) or `Lit` (char) |
| `as_bool(&self) -> bool` | Bool value | `Expr` (bool lit) or `Lit` (bool) |

## Built-in `Attribute` Impls

### Scalars

| Type | From `Flag(ident)` | From `Expr(_, lit)` | From `List(_, args)` | From `Lit(lit)` | From absent |
|---|---|---|---|---|---|
| `bool` | `true` | error | error | error | `false` |
| `String` | error | string `.value()` | error | string `.value()` | error |
| `i8`..`i128`, `u8`..`u128` | error | int literal | error | int literal | error |
| `f32`, `f64` | error | float literal | error | float literal | error |
| `char` | error | char literal | error | char literal | error |
| `syn::Ident` | the ident | error | error | error | error |
| `syn::Path` | as path | error | error | error | error |
| `syn::Expr` | error | the expr | error | error | error |
| `syn::LitStr` | error | string lit | error | string lit | error |
| `syn::LitInt` | error | int lit | error | int lit | error |

### Containers

| Type | From `Flag` | From `Expr` | From `List(_, args)` | From absent |
|---|---|---|---|---|
| `Option<T>` | `Some(T from flag)` | `Some(T from arg)` | `Some(T from arg)` | `None` |
| `Vec<T>` | error | error | parse each inner arg | `vec![]` |
| `Args` | error | error | nested args directly | `Args::new()` |

### `syn::Parse` fallback

Any field type that implements `syn::Parse` can be used as an argument type without needing an `Attribute` derive. The generated code attempts `from_arg` if the type implements `Attribute`, otherwise falls back to parsing the expression/literal token stream via `syn::Parse`.

## Struct-level Annotations

| Annotation | Effect |
|---|---|
| `#[zyn("name")]` | The attribute name to match (e.g. `"serde"` matches `#[serde(...)]`). Presence activates attribute mode. |
| `#[zyn(about = "...")]` | Description of the attribute; used in `about()` header and error messages |
| `#[zyn(unique)]` | Only one occurrence of this attribute allowed on an item. Multiple → error. Without this, multiple occurrences are merged. |

Combinable: `#[zyn("serde", unique, about = "Configure serialization")]`

## Field Annotations

| Annotation | Effect |
|---|---|
| `#[zyn(0)]` | Positional: consume `args[0]` (anonymous `Arg::Lit`). The integer is the positional index. |
| `#[zyn("key")]` | Name override: look for `args.get("key")` instead of the field name |
| (bare field) | Uses the field's own name: `args.get("field_name")` |
| `#[zyn(default)]` | Use `Default::default()` when absent |
| `#[zyn(default = value)]` | Use literal as default when absent |
| `#[zyn(skip)]` | Don't extract; always `Default::default()` |
| `#[zyn(about = "...")]` | Description of the argument; used in error messages and generated `about()` |

Combinable: `#[zyn(0, default = ".", about = "working directory")]`

## Required vs Optional

- Non-`Option<T>` fields without `#[zyn(default)]` or `#[zyn(skip)]` are **required**
- `Option<T>` → always optional (absent → `None`)
- `bool` → always optional (absent → `false`)

## Duplicate Key Behavior

- Duplicate keys within a single attribute → always error
- Exception: `Vec<T>` fields collect multiple occurrences of the same key
- Multiple attribute occurrences on same item (e.g. two `#[serde(...)]`) → merged unless `#[zyn(unique)]`

## Generated Code for Attribute Mode Structs

```rust
#[derive(Attribute)]
#[zyn("serde", unique, about = "Configure serialization")]
struct SerdeConfig {
    #[zyn(0, about = "the input path")]
    path: String,
    #[zyn("rename_all", about = "case transform")]
    casing: Option<String>,
    deny_unknown_fields: bool,
    #[zyn(default = "json", about = "output format")]
    format: String,
}
```

Generates:

```rust
impl Attribute for SerdeConfig {
    fn from_args(args: &Args) -> syn::Result<Self> {
        Ok(Self {
            path: String::from_arg(&args[0])?,
            casing: match args.get("rename_all") {
                Some(arg) => Some(String::from_arg(arg)?),
                None => None,
            },
            deny_unknown_fields: args.has("deny_unknown_fields"),
            format: match args.get("format") {
                Some(arg) => String::from_arg(arg)?,
                None => String::from("json"),
            },
        })
    }

    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::List(_, args) => Self::from_args(args),
            _ => Err(/* expected list */),
        }
    }

    fn attribute(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let matches: Vec<_> = attrs.iter()
            .filter(|a| a.path().is_ident("serde"))
            .collect();

        if matches.len() > 1 {
            return Err(/* unique: only one #[serde(...)] allowed */);
        }

        match matches.first() {
            Some(attr) => {
                let args: Args = attr.parse_args()?;
                Self::from_args(&args)
            }
            None => Self::from_args(&Args::new()),
        }
    }
}

impl SerdeConfig {
    pub fn about() -> &'static str {
        "#[serde(...)]: Configure serialization\n\
         \n\
         Arguments:\n\
         [0] path: String (required) — the input path\n\
         rename_all: Option<String> — case transform\n\
         deny_unknown_fields: bool\n\
         format: String (default: \"json\") — output format"
    }
}
```

## Enum Derive (Discriminated Unions)

Enums always derive in argument mode — they generate only `from_arg`. Used as field types within attribute structs. Dispatches by snake_cased variant name:

- **Unit variants** → match as `Arg::Flag`: `fast` → `Mode::Fast`
- **Struct variants** → match as `Arg::List`: `custom(speed = 5)` → `Mode::Custom { speed: 5 }`
- **Tuple variants** → match as `Arg::List` with positional args: `hex("#ff0000")` → `Color::Hex("#ff0000".into())`
- **Single-field tuple variants** → match as `Arg::Expr`: `name = "blue"` → `Color::Named("blue".into())`

```rust
#[derive(Attribute)]
enum Mode {
    Fast,
    Slow,
    Custom { speed: i64 },
}

#[derive(Attribute)]
#[zyn("config")]
struct Config {
    mode: Mode,  // matches: fast | slow | custom(speed = 5)
}
```

Generates:

```rust
impl Attribute for Mode {
    fn from_args(_args: &Args) -> syn::Result<Self> {
        Err(/* enums are matched via from_arg, not from_args */)
    }

    fn from_arg(arg: &Arg) -> syn::Result<Self> {
        match arg {
            Arg::Flag(ident) => match ident.to_string().as_str() {
                "fast" => Ok(Self::Fast),
                "slow" => Ok(Self::Slow),
                other => Err(/* unknown variant */),
            },
            Arg::List(ident, args) => match ident.to_string().as_str() {
                "custom" => {
                    let speed = match args.get("speed") {
                        Some(arg) => i64::from_arg(arg)?,
                        None => return Err(/* missing required field */),
                    };
                    Ok(Self::Custom { speed })
                }
                other => Err(/* unknown variant */),
            },
            _ => Err(/* expected flag or list */),
        }
    }

    fn attribute(_attrs: &[syn::Attribute]) -> syn::Result<Self> {
        Err(/* enums are argument types, not attributes */)
    }
}
```

## Recursive Nesting

A field whose type also derives `Attribute` is parsed from a nested `List` arg via `T::from_arg(arg)`:

```rust
#[derive(Attribute)]
struct Inner {
    a: i64,
    b: String,
}

#[derive(Attribute)]
#[zyn("my_attr")]
struct Outer {
    inner: Inner,  // parsed from: my_attr(inner(a = 1, b = "x"))
}
```

The `inner` field matches `Arg::List("inner", args)` and calls `Inner::from_arg(arg)`, which delegates to `Inner::from_args(args)`.

## `about()` Generation

Generated on attribute mode structs only. Rules:

- Header: `#[name(...)]: about text` (or just `#[name(...)]` if no struct-level `about`)
- Blank line, then `Arguments:` label
- One line per field:
  - Positional: `[N] name: Type (required|optional|default: "val") — about text`
  - Named: `name: Type (required|optional|default: "val") — about text`
  - `skip` fields omitted
  - `— about text` suffix omitted if no `about` on field

## Files to Create / Modify

| File | Change |
|---|---|
| `crates/core/src/meta/arg.rs` | Add typed accessors (`as_flag`, `as_str`, `as_int`, `as_float`, `as_char`, `as_bool`) |
| `crates/core/src/meta/attribute.rs` | **New** — `Attribute` trait + built-in impls for scalars, containers, syn types |
| `crates/core/src/meta/mod.rs` | Add `mod attribute; pub use attribute::*;` |
| `crates/derive/src/attribute.rs` | **New** — `#[derive(Attribute)]` expansion for structs (both modes) and enums |
| `crates/derive/src/lib.rs` | Register `Attribute` derive macro |

## Tests

### Attribute mode struct
- Full attribute extraction with multiple typed fields
- Positional args (`#[zyn(0)]`)
- Name override (`#[zyn("key")]`)
- Missing optional → `None`
- Missing required → error
- Unknown field → error
- `default`, `skip` annotations
- `unique` — error on multiple attribute occurrences
- Non-unique — multiple attributes merged
- Absent attribute entirely → defaults apply

### Argument mode struct
- `from_args` extraction
- `from_arg` from `List` arg
- Nested within an attribute mode struct

### Enum derive
- Unit variant from flag
- Struct variant from list
- Tuple variant from list with positional
- Single-field tuple variant from expr
- Unknown variant → error

### Recursive nesting
- Nested struct field parsed from `List` arg
- Multiple levels of nesting

### Type extraction
- `bool` from flag / from absent
- `String` from string literal
- Integer types from int literal
- `Option<String>` present vs absent
- `Vec<String>` from nested list / duplicate keys
- `syn::Ident` from flag ident
- `syn::Parse` type as field (fallback)

### Duplicate keys
- Duplicate named key → error
- Duplicate key with `Vec<T>` field → collected
- Duplicate `Arg::Lit` at same position → error

### Arg accessors
- `as_flag` on `Flag` variant
- `as_str` on `Expr` with string lit / `Lit` with string
- `as_int::<i64>` on `Lit` with int
- Panics on wrong variant

### `about()` generation
- Attribute mode struct with `about` on struct and fields
- No `about` annotations → minimal output
- `skip` fields omitted
- Positional fields show `[N]` prefix
- Default values shown in parenthetical
