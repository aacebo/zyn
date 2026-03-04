# Errors and Warnings

Emit a `compile_error!` with `@throw`:

```rust,zyn
zyn! {
    @if (!valid) {
        @throw "expected a struct"
    }
}
```

```bash
error: expected a struct
  --> src/lib.rs:12:9
   |
12 |         @throw "expected a struct"
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Emit a compiler warning with `@warn`:

```rust,zyn
zyn! {
    @if (deprecated) {
        @warn "this usage is deprecated, use `new_api` instead"
    }
}
```

```bash
warning: this usage is deprecated, use `new_api` instead
  --> src/lib.rs:8:9
   |
 8 |         @warn "this usage is deprecated, use `new_api` instead"
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default
```

`@warn` does not halt compilation. `@throw` is a hard error.
