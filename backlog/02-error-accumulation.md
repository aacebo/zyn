# Phase 2: Error Accumulation

## Goal

When `Attribute` extraction encounters multiple errors (missing fields, wrong types, unknown keys), all errors are collected and emitted together instead of failing on the first one. Error messages include `about` text when available.

## Problem

Phase 1 uses `?` to short-circuit on the first error. Users see one error, fix it, recompile, see the next — frustrating feedback loop.

```bash
# Today: one error at a time
error: missing required field `name` in #[my_derive(...)]
  --> src/lib.rs:5:1
```

```bash
# After: all errors together, with about text
error: missing required field `path` in #[serde(...)]
       [0] path: String (required) — the input path
  --> src/lib.rs:5:1

error: unknown attribute key `naem` in #[serde(...)]
  --> src/lib.rs:5:20
   = help: did you mean `name`?

error: expected string literal for `format`, got integer
       format: String (default: "json") — output format
  --> src/lib.rs:5:35
```

## Design

### Accumulator

The generated `from_args` collects errors instead of returning early:

```rust
impl Attribute for SerdeConfig {
    fn from_args(args: &Args) -> syn::Result<Self> {
        let mut errors: Vec<syn::Error> = Vec::new();

        let path = match args.get(0) {
            Some(arg) if arg.is_lit() => match String::from_arg(arg) {
                Ok(v) => Some(v),
                Err(e) => { errors.push(e); None }
            },
            Some(_) => {
                errors.push(syn::Error::new(Span::call_site(), "expected literal at position 0"));
                None
            }
            None => {
                errors.push(syn::Error::new(
                    Span::call_site(),
                    "missing required positional argument [0] `path`\n\
                     [0] path: String (required) — the input path",
                ));
                None
            }
        };

        let casing = match args.get("rename_all") {
            Some(arg) => match String::from_arg(arg) {
                Ok(v) => Some(Some(v)),
                Err(e) => { errors.push(e); None }
            },
            None => Some(None),
        };

        let deny_unknown_fields = args.has("deny_unknown_fields");

        let format = match args.get("format") {
            Some(arg) => match String::from_arg(arg) {
                Ok(v) => Some(v),
                Err(e) => { errors.push(e); None }
            },
            None => Some(String::from("json")),
        };

        // ... unknown key detection ...

        if !errors.is_empty() {
            let mut combined = errors.remove(0);
            for e in errors {
                combined.combine(e);
            }
            return Err(combined);
        }

        Ok(Self {
            path: path.unwrap(),
            casing: casing.unwrap(),
            deny_unknown_fields,
            format: format.unwrap(),
        })
    }
}
```

### Error Messages with `about` Text

When a field has an `about` annotation, error messages include the field's description as context:

- Missing required: `"missing required field \`path\`\n[0] path: String (required) — the input path"`
- Type mismatch: `"expected string literal for \`format\`, got integer\nformat: String (default: \"json\") — output format"`

### Unknown Key Detection

After extracting all known fields, remaining named keys in `Args` are flagged:

```rust
let known = &["rename_all", "deny_unknown_fields", "format"];

for arg in args.iter() {
    if let Some(ident) = arg.name() {
        let key = ident.to_string();
        if !known.contains(&key.as_str()) {
            let mut msg = format!("unknown attribute key `{key}`");

            if let Some(suggestion) = closest_match(&key, known) {
                msg = format!("unknown attribute key `{key}`, did you mean `{suggestion}`?");
            }

            errors.push(syn::Error::new(ident.span(), msg));
        }
    }
}
```

Positional fields (`#[zyn(0)]`) are not included in the `known` list — they match by position, not by name.

### "Did You Mean?" via Edit Distance

Levenshtein distance against known field names. Suggest the closest match if the distance is ≤ 3:

```rust
fn closest_match<'a>(input: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .map(|c| (c, levenshtein(input, c)))
        .filter(|(_, d)| *d <= 3)
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| *c)
}
```

### Struct-level `about` in Error Context

When a struct has `#[zyn(about = "...")]`, errors include the description:

```bash
error: missing required field `path` in #[serde(...)]
       Configure serialization
       [0] path: String (required) — the input path
```

## Files to Modify

| File | Change |
|---|---|
| `crates/derive/src/attribute.rs` | Retrofit `from_args` codegen to use error accumulator + unknown key detection + `about` in errors |
| `crates/core/src/meta/` | Add `closest_match` / `levenshtein` utility |

## Tests

- Two missing required fields → both errors shown
- Missing positional argument → error with `about` text
- Unknown attribute key → error with "did you mean?" suggestion
- Unknown key with no close match → error without suggestion
- Mix of valid + invalid fields → valid ones still extracted, all errors emitted
- Type mismatch + missing field → both errors with `about` context
- Zero errors → normal success path unchanged
- Struct-level `about` included in error output
- Field-level `about` included in field-specific errors
