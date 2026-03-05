# Phase 1: `FromInput` Trait + `Arg` Accessors

## Goal

Define the `FromInput` trait in `zyn-core` as the single extraction abstraction. Add typed accessor methods to `Arg`. Provide built-in `FromInput` implementations for scalars, containers, and `syn` types used when parsing individual `Arg` values. No derive macro yet — that is Phase 2 and 3.

## The `FromInput` Trait

```rust
pub trait FromInput: Sized {
    type Error: Into<syn::Error>;

    fn from_input(input: &Input) -> Result<Self, Self::Error>;
}
```

- `input` — the `Input` context (wraps `DeriveInput` | `ItemInput`), providing access to attrs, ident, generics, etc.
- Types implementing `FromInput` can be used as extractor params in `#[element]` functions (Phase 4).

## New Type: `Input` (in `crates/core/src/input/`)

A unified enum wrapping the two macro input contexts:

```rust
pub enum Input {
    Derive(DeriveInput),
    Item(ItemInput),
}
```

Common accessors:
- `.attrs() -> &[syn::Attribute]`
- `.ident() -> &syn::Ident`
- `.generics() -> &syn::Generics`

## Arg Enhancements

Add typed panicking accessor methods to `Arg`, following the existing `as_expr` / `as_args` / `as_lit` pattern:

| Method | Returns | Variant(s) |
|---|---|---|
| `as_flag(&self) -> &Ident` | The flag ident | `Flag` |
| `as_str(&self) -> String` | String value | `Expr` (string lit) or `Lit` (string) |
| `as_int<T: std::str::FromStr>(&self) -> T` | Parsed integer | `Expr` (int lit) or `Lit` (int) |
| `as_float<T: std::str::FromStr>(&self) -> T` | Parsed float | `Expr` (float lit) or `Lit` (float) |
| `as_char(&self) -> char` | Char value | `Expr` (char lit) or `Lit` (char) |
| `as_bool(&self) -> bool` | Bool value | `Expr` (bool lit) or `Lit` (bool) |

## Built-in `FromInput` Impls

These cover scalar and container types used when `#[derive(Attribute)]` extracts individual field values from `Arg`s. They are **not** used directly as element extractors — they are leaf-level value parsers called by generated struct `from_input` logic.

A secondary helper trait is defined for this purpose:

```rust
pub trait FromArg: Sized {
    fn from_arg(arg: &Arg) -> syn::Result<Self>;
}
```

Built-in `FromArg` impls:

### Scalars

| Type | From `Flag(ident)` | From `Expr(_, lit)` | From `List(_, args)` | From `Lit(lit)` |
|---|---|---|---|---|
| `bool` | `true` | error | error | error |
| `String` | error | string `.value()` | error | string `.value()` |
| `i8`..`i128`, `u8`..`u128` | error | int literal | error | int literal |
| `f32`, `f64` | error | float literal | error | float literal |
| `char` | error | char literal | error | char literal |
| `syn::Ident` | the ident | error | error | error |
| `syn::Path` | as path | error | error | error |
| `syn::Expr` | error | the expr | error | error |
| `syn::LitStr` | error | string lit | error | string lit |
| `syn::LitInt` | error | int lit | error | int lit |

### Containers

| Type | From `Flag` | From `Expr` | From `List(_, args)` | From absent |
|---|---|---|---|---|
| `Option<T: FromArg>` | `Some(T::from_arg(arg)?)` | `Some(T::from_arg(arg)?)` | `Some(T::from_arg(arg)?)` | `None` |
| `Vec<T: FromArg>` | error | error | each inner arg via `T::from_arg` | `vec![]` |
| `Args` | error | error | nested args directly | `Args::new()` |

## Files to Create / Modify

| File | Change |
|---|---|
| `crates/core/src/meta/arg.rs` | Add `as_flag`, `as_str`, `as_int`, `as_float`, `as_char`, `as_bool` |
| `crates/core/src/input/mod.rs` | Add `Input` enum with common accessors |
| `crates/core/src/extract.rs` | **New** — `FromInput` trait + `FromArg` trait + all built-in `FromArg` impls |
| `crates/core/src/lib.rs` | `pub mod extract; pub use extract::*;` |

## Tests

### Arg accessors
- `as_flag` on `Flag` → returns ident
- `as_str` on `Expr` with string lit → returns value
- `as_str` on `Lit` with string → returns value
- `as_int::<i64>` on `Lit` with int → returns value
- `as_flag` on non-Flag → panics
- `as_str` on `Flag` → panics

### `FromArg` impls
- `bool::from_arg` with `Flag` → `true`
- `String::from_arg` with string expr → value
- `i64::from_arg` with int lit → value
- `Option<String>::from_arg` present → `Some(value)`
- `Vec<String>::from_arg` with `List` → collected values
- `syn::Ident::from_arg` with `Flag` → the ident
- Wrong type → `Err`
