# Phase 3: `#[derive(Attribute)]` — Enums

## Goal

Generate extraction logic for enums. Enums always derive in argument mode — they are used as field types within attribute structs, not as top-level extractors. Depends on Phase 2.

## Enum Derive (Discriminated Unions)

Enums generate a `from_arg(arg: &Arg) -> syn::Result<Self>` associated function. Dispatch is by snake_cased variant name:

- **Unit variants** → match as `Arg::Flag`: `fast` → `Mode::Fast`
- **Struct variants** → match as `Arg::List`: `custom(speed = 5)` → `Mode::Custom { speed: 5 }`
- **Tuple variants** → match as `Arg::List` with positional args: `hex("ff0000")` → `Color::Hex("ff0000".into())`
- **Single-field tuple variants** → match as `Arg::Expr`: `name = "blue"` → `Color::Named("blue".into())`

Enums do **not** implement `FromInput` — they are argument types, not top-level extractors.

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

## Generated Code Example

```rust
impl Mode {
    fn from_arg(arg: &::zyn::Arg) -> ::syn::Result<Self> {
        match arg {
            ::zyn::Arg::Flag(ident) => match ident.to_string().as_str() {
                "fast" => Ok(Self::Fast),
                "slow" => Ok(Self::Slow),
                other => Err(::syn::Error::new(
                    ident.span(),
                    format!("unknown variant `{other}`, expected one of: fast, slow, custom"),
                )),
            },
            ::zyn::Arg::List(ident, args) => match ident.to_string().as_str() {
                "custom" => Ok(Self::Custom {
                    speed: match args.get("speed") {
                        Some(arg) => ::zyn::FromArg::from_arg(arg)?,
                        None => return Err(::syn::Error::new(
                            ident.span(),
                            "missing required field `speed`",
                        )),
                    },
                }),
                other => Err(::syn::Error::new(
                    ident.span(),
                    format!("unknown variant `{other}`, expected one of: fast, slow, custom"),
                )),
            },
            _ => Err(::syn::Error::new(
                ::proc_macro2::Span::call_site(),
                "expected flag or list argument",
            )),
        }
    }
}
```

## Variant Name Mapping

Variant names are converted to snake_case for matching. `MyVariant` → `my_variant`.

## Files to Modify

| File | Change |
|---|---|
| `crates/derive/src/attribute/enums.rs` | **New** — enum codegen |
| `crates/derive/src/attribute/mod.rs` | Wire enum dispatch |

## Tests

- Unit variant from `Flag` → correct variant
- Struct variant from `List` → correct variant with fields extracted
- Tuple variant from `List` with positional args
- Single-field tuple variant from `Expr`
- Unknown variant → error listing valid options
- Wrong arg shape → error
- Enum as field type inside an attribute mode struct (integration)
- No `FromInput` impl generated
