# Phase 2: `#[derive(Attribute)]` — Structs

## Goal

Generate `FromInput` implementations for structs. Supports attribute mode (struct has `#[zyn("name")]`) and argument mode (no struct-level `#[zyn(...)]`). Depends on Phase 1.

## Two Modes

The presence of `#[zyn("name")]` at the struct level determines the mode:

### Attribute mode

Struct has `#[zyn("name", ...)]`. Generates a `FromInput` impl that finds the named attribute in `input.attrs()`, parses its args, and constructs the struct from them. Also generates `about()`.

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

// used directly as an element extractor param:
// fn my_element(cfg: SerdeConfig, ...) -> TokenStream
```

### Argument mode

Struct has no `#[zyn("name")]`. Generates a helper `from_args(args: &Args) -> syn::Result<Self>` associated function (not a trait impl). Used as nested types within attribute mode structs.

```rust
#[derive(Attribute)]
struct Inner {
    a: i64,
    b: String,
}
```

## Struct-level Annotations

| Annotation | Effect |
|---|---|
| `#[zyn("name")]` | The attribute name to match (e.g. `"serde"` matches `#[serde(...)]`). Activates attribute mode. |
| `#[zyn(about = "...")]` | Description used in `about()` and error messages |
| `#[zyn(unique)]` | Only one occurrence allowed on an item. Multiple → error. Without this, multiple occurrences are merged. |

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
| `#[zyn(about = "...")]` | Description used in error messages and `about()` |

Combinable: `#[zyn(0, default = ".", about = "working directory")]`

## Required vs Optional

- Non-`Option<T>` fields without `#[zyn(default)]` or `#[zyn(skip)]` → **required**
- `Option<T>` → always optional (absent → `None`)
- `bool` → always optional (absent → `false`)

## Duplicate Key Behavior

- Duplicate named keys within a single attribute → error
- `Vec<T>` fields collect multiple occurrences of the same key
- Multiple attribute occurrences on same item → merged unless `#[zyn(unique)]`

## Generated Code Example

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
impl SerdeConfig {
    fn from_args(args: &::zyn::Args) -> ::syn::Result<Self> {
        Ok(Self {
            path: ::zyn::FromArg::from_arg(&args[0])?,
            casing: match args.get("rename_all") {
                Some(arg) => Some(::zyn::FromArg::from_arg(arg)?),
                None => None,
            },
            deny_unknown_fields: args.has("deny_unknown_fields"),
            format: match args.get("format") {
                Some(arg) => ::zyn::FromArg::from_arg(arg)?,
                None => ::std::string::String::from("json"),
            },
        })
    }

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

impl ::zyn::FromInput for SerdeConfig {
    type Error = ::syn::Error;

    fn from_input(input: &::zyn::Input) -> ::std::result::Result<Self, Self::Error> {
        let matches: Vec<_> = input.attrs().iter()
            .filter(|a| a.path().is_ident("serde"))
            .collect();

        if matches.len() > 1 {
            return Err(::syn::Error::new(
                ::proc_macro2::Span::call_site(),
                "only one #[serde(...)] allowed",
            ));
        }

        match matches.first() {
            Some(attr) => {
                let args: ::zyn::Args = attr.parse_args()?;
                Self::from_args(&args)
            }
            None => Self::from_args(&::zyn::Args::new()),
        }
    }
}
```

`FromInput` is only generated for **attribute mode** structs. Argument mode structs only get `from_args`.

## Recursive Nesting

A field whose type also derives `Attribute` (argument mode) is parsed from a nested `List` arg by calling `T::from_args(args)`:

```rust
#[derive(Attribute)]
struct Inner { a: i64, b: String }

#[derive(Attribute)]
#[zyn("my_attr")]
struct Outer {
    inner: Inner,  // parsed from: my_attr(inner(a = 1, b = "x"))
}
```

`inner` matches `Arg::List("inner", args)` and calls `Inner::from_args(args)`.

## `about()` Generation

Generated on attribute mode structs only:

- Header: `#[name(...)]: about text` (or just `#[name(...)]` if no struct-level `about`)
- Blank line, then `Arguments:` label
- One line per field (skip fields omitted):
  - Positional: `[N] name: Type (required|optional|default: "val") — about text`
  - Named: `name: Type (required|optional|default: "val") — about text`
  - `— about text` omitted if no field `about`

## Files to Create / Modify

| File | Change |
|---|---|
| `crates/derive/src/attribute/mod.rs` | **New** — entrypoint `pub fn expand(input: TokenStream) -> TokenStream`, dispatches to `structs` or `enums` |
| `crates/derive/src/attribute/structs.rs` | **New** — struct codegen (both modes) |
| `crates/derive/src/lib.rs` | Register `#[proc_macro_derive(Attribute, attributes(zyn))]` |

## Tests

### Attribute mode struct
- Full extraction with multiple typed fields
- Positional args (`#[zyn(0)]`)
- Name override (`#[zyn("key")]`)
- Missing optional → `None`
- Missing required → error
- `default` annotation
- `skip` annotation → `Default::default()`
- `unique` → error on multiple attribute occurrences
- Non-unique → multiple attributes merged
- Absent attribute entirely → defaults apply
- `FromInput` impl generated → usable as element extractor param

### Argument mode struct
- `from_args` extraction
- Nested within an attribute mode struct
- No `FromInput` impl generated

### Recursive nesting
- Nested struct field parsed from `List` arg
- Multiple levels of nesting

### `about()` generation
- With `about` on struct and fields
- No `about` annotations → minimal output
- `skip` fields omitted
- Positional fields show `[N]` prefix
- Default values shown in parenthetical
