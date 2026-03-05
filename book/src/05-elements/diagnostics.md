# Diagnostics

Elements handle diagnostics through the `error!`, `warn!`, `note!`, `help!`, and `bail!` macros. The `#[element]` attribute introduces a `diagnostics` accumulator automatically — macros push to it and `bail!` returns early if errors exist.

## Compile Errors

Use `error!` to push an error, and `bail!` to return early:

```rust
#[zyn::element]
fn validated(name: syn::Ident) -> zyn::TokenStream {
    if name.to_string() == "forbidden" {
        bail!("reserved identifier"; span = name.span());
    }

    zyn::zyn! { fn {{ name }}() {} }
}
```

```bash
error: reserved identifier
  --> src/lib.rs:8:24
   |
 8 |     @validated(name = forbidden)
   |                       ^^^^^^^^^
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

## Warnings

Use `warn!` to emit a non-fatal warning. Does not halt compilation:

```rust
#[zyn::element]
fn legacy(name: syn::Ident) -> zyn::TokenStream {
    warn!("deprecated, use `new_elem` instead");
    help!("replace `@legacy` with `@new_elem`");

    zyn::zyn! { fn {{ name }}() {} }
}
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
```

## Accessing the Accumulator Directly

The `diagnostics` variable is a `zyn::Diagnostics` and can be used directly:

```rust
#[zyn::element]
fn my_element(#[zyn(input)] fields: zyn::Fields<syn::Field>) -> zyn::TokenStream {
    for field in fields.iter() {
        if field.ident.is_none() {
            error!("all fields must be named"; span = field.span());
        }
    }

    if diagnostics.has_errors() {
        return diagnostics.emit();
    }

    zyn::zyn! { struct Validated; }
}
```
