# Error Reference

Parse errors produced during `zyn!(...)` expansion.

## Parse Errors

| Situation | Error |
|---|---|
| `{{ }}` (empty interpolation) | `"empty interpolation"` |
| `@else` not preceded by `@if` | `"unexpected @else without @if"` |
| `@for (x foo ...)` — wrong keyword between binding and iter | `"expected 'in'"` |
| `@throw` / `@warn` / `@note` / `@help` without a string literal | `"expected string literal"` |
| Invalid directive inside `@throw { }` / `@warn { }` body | `"expected 'note' or 'help', found '...'"` |
| `@element(prop value)` — missing `=` in prop | syn parse error |
| Unrecognized token where expression expected | syn parse error |

## Runtime / Generated Code Errors

| Situation | Error |
|---|---|
| Pipe `Output` type not implementing `ToTokens` | Rust type error in generated code |
| Element struct field type mismatch at call site | Rust type error in generated code |

## Attribute Extraction Errors

`#[derive(Attribute)]` accumulates all validation errors via the `Diagnostics` type (`zyn::Result<T>` = `Result<T, Diagnostics>`) instead of short-circuiting on the first failure:

| Situation | Error |
|---|---|
| Missing required field | `"missing required field \`name\`"` with `about` text if available |
| Type mismatch | `"expected string literal"` etc. from `FromArg` |
| Unknown named argument | `"unknown argument \`naem\`"` with `"did you mean \`name\`?"` if a close match exists (Levenshtein distance ≤ 3) |

All errors are collected and returned together as a single `Diagnostics` value.

## Diagnostic Directives

`@throw` emits a hard compile error via `compile_error!` (or natively via `proc_macro::Diagnostic` on nightly). `@warn`, `@note`, and `@help` emit non-fatal diagnostics that do not halt compilation. All four accept an optional `{ @note "..." @help "..." }` body (where applicable) to attach child diagnostics.
