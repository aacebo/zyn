# Diagnostics

Fallible elements return `zyn::Result` and can emit rich errors with notes and help text:

```rust,zyn
#[zyn::element]
fn validated(name: syn::Ident) -> zyn::Result {
    if name == "forbidden" {
        return Err(zyn::Diagnostic::error(name.span(), "reserved identifier")
            .note("this name is reserved by the compiler")
            .help("try a different name like `my_handler`"));
    }
    Ok(zyn::zyn!(fn {{ name }}() {}))
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

Warnings can be included in infallible element output by prepending warning tokens:

```rust,zyn
#[zyn::element]
fn legacy(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
    let warning = zyn::Diagnostic::warning(name.span(), "deprecated, use `new_elem` instead")
        .to_token_stream();
    let body = zyn::zyn!(fn {{ name }}() {});
    quote::quote! { #warning #body }
}
```

```bash
warning: deprecated, use `new_elem` instead
  --> src/lib.rs:5:5
   |
 5 |     @legacy(name = my_fn)
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default
```
