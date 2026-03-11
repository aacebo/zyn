# Error Reference

## Parse Errors

Errors produced during `zyn!(...)` expansion:

| Situation | Error |
|---|---|
| `{{ }}` (empty interpolation) | `"empty interpolation"` |
| `@else` not preceded by `@if` | `"unexpected @else without @if"` |
| `@for (x foo ...)` — wrong keyword between binding and iter | `"expected 'in'"` |
| `@element(prop value)` — missing `=` in prop | syn parse error |
| Unrecognized token where expression expected | syn parse error |

## Generated Code Errors

| Situation | Error |
|---|---|
| Pipe `Output` type not implementing `ToTokens` | Rust type error in generated code |
| Element struct field type mismatch at call site | Rust type error in generated code |

## Attribute Extraction Errors

`#[derive(Attribute)]` accumulates all validation errors via the `Diagnostic` type (`zyn::Result<T>` = `Result<T, Diagnostic>`):

| Situation | Error |
|---|---|
| Missing required field | `"missing required field \`name\`"` with `about` text if available |
| Type mismatch | `"expected string literal"` etc. from `FromArg` |
| Unknown named argument | `"unknown argument \`naem\`"` with `"did you mean \`name\`?"` if a close match exists (Levenshtein distance ≤ 3) |

All errors are collected and returned together as a single `Diagnostic` value.

> **Typo recovery:** Unknown arguments within Levenshtein distance 3 of a known field
> automatically generate a `help: did you mean ...?` suggestion pointing at the typo span.

## Element Diagnostic Macros

`#[zyn::element]` generates local `error!`, `warn!`, `note!`, `help!`, and `bail!` macros that push diagnostics to the element's `diagnostics` accumulator. `bail!` returns early if errors exist. All accept `format!`-style arguments and an optional `; span = expr` suffix. These macros are only available inside `#[zyn::element]` bodies.

See [Diagnostics](../03-elements/diagnostics.md) for usage examples.
