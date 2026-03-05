# Diagnostics

Elements handle all diagnostics through **template syntax** — `@throw`, `@warn`, `@note`, and `@help` directives. Elements always return `zyn::TokenStream` and are infallible; any errors or warnings are expressed inline in the template.

## Compile Errors

Use `@throw` inside an element to halt compilation with an error:

```rust,zyn
#[zyn::element]
fn validated(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn! {
        @if (name.to_string() == "forbidden") {
            @throw "reserved identifier"
        } @else {
            fn {{ name }}() {}
        }
    }
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

Attach notes and help suggestions using a body block:

```rust,zyn
#[zyn::element]
fn validated(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn! {
        @if (name.to_string() == "forbidden") {
            @throw "reserved identifier" {
                @note "this name is reserved by the compiler"
                @help "try a different name like `my_handler`"
            }
        } @else {
            fn {{ name }}() {}
        }
    }
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

Use `@warn` to emit a non-fatal warning:

```rust,zyn
#[zyn::element]
fn legacy(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn! {
        @warn "deprecated, use `new_elem` instead" {
            @help "replace `@legacy` with `@new_elem`"
        }
        fn {{ name }}() {}
    }
}
```

```bash
warning: deprecated, use `new_elem` instead
help: replace `@legacy` with `@new_elem`
  --> src/lib.rs:5:5
   |
 5 |     @legacy(name = my_fn)
   |     ^^^^^^^^^^^^^^^^^^^^^
```

`@warn` does not halt compilation — the rest of the template continues to expand normally.

## Standalone Notes and Help

`@note` and `@help` can appear standalone as informational diagnostics:

```rust,zyn
zyn! {
    @note "this field is deprecated"
    @help "consider using `new_field` instead"
}
```
