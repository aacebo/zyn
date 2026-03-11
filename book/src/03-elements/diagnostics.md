# Diagnostics

The `#[zyn::element]` attribute generates `error!`, `warn!`, `note!`, `help!`, and `bail!` macros that push diagnostics to an auto-injected `diagnostics` accumulator.

## `error!` — Compile Error

```rust
#[zyn::element]
fn validated(#[zyn(input)] ident: syn::Ident) -> zyn::TokenStream {
    if ident.to_string() == "forbidden" {
        error!("reserved identifier");
    }
    bail!();

    zyn::zyn! { fn {{ ident }}() {} }
}
```

```bash
error: reserved identifier
  --> src/lib.rs:8:24
```

## `warn!` — Compiler Warning

Does not halt compilation:

```rust
#[zyn::element]
fn legacy(#[zyn(input)] ident: syn::Ident) -> zyn::TokenStream {
    warn!("this usage is deprecated, use `new_api` instead");

    zyn::zyn! { fn {{ ident }}() {} }
}
```

## `note!` — Informational Note

```rust
note!("only named fields are supported");
note!("expected `{}`", expected; span = ident.span());
```

## `help!` — Help Suggestion

```rust
help!("consider using `Builder::new()` instead");
help!("try `{}` instead", suggestion; span = ident.span());
```

## `bail!` — Early Return on Errors

Returns early if any errors have accumulated, or pushes an error and returns immediately:

```rust
bail!();                             // return if any errors
bail!("struct must have fields");    // push error + return
```

## Errors with Notes and Help

Accumulate multiple diagnostics before returning:

```rust
#[zyn::element]
fn validated(name: syn::Ident) -> zyn::TokenStream {
    if name.to_string() == "forbidden" {
        error!("reserved identifier"; span = name.span());
        note!("this name is reserved by the compiler");
        help!("try a different name like `my_handler`");
    }
    bail!();

    zyn::zyn! { fn {{ name }}() {} }
}
```

```bash
error: reserved identifier
note: this name is reserved by the compiler
help: try a different name like `my_handler`
  --> src/lib.rs:8:24
   |
 8 |     @validated(name = forbidden)
   |                       ^^^^^^^^^
```

## Format String Interpolation

All macros accept `format!`-style arguments:

```rust
error!("field `{}` is required", name);
warn!("type `{}` is deprecated", ty);
```

## Custom Spans

Override the default span with `; span = expr`:

```rust
error!("invalid field"; span = field.span());
bail!("missing `{}`", name; span = ident.span());
```

## Accessing the Accumulator Directly

The `diagnostics` variable is a `zyn::mark::Builder` and can be used directly:

```rust
#[zyn::element]
fn my_element(#[zyn(input)] fields: zyn::Fields<syn::Field>) -> zyn::TokenStream {
    for field in fields.iter() {
        if field.ident.is_none() {
            error!("all fields must be named"; span = field.span());
        }
    }

    bail!();

    zyn::zyn! { struct Validated; }
}
```
