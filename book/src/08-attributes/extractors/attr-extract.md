# `Attr<T>` and `Extract<T>`

Generic wrappers for any `FromInput` type. Both delegate to `T::from_input(input)` and implement `Deref`/`DerefMut` to `T`.

## `Attr<T>`

Signals the value comes from a named attribute (a `#[derive(Attribute)]` struct):

```rust
#[derive(zyn::Attribute)]
#[zyn("my_derive")]
struct MyConfig {
    skip: bool,
    rename: Option<String>,
}

#[zyn::element]
fn my_element(
    #[zyn(input)] cfg: zyn::Attr<MyConfig>,
    name: syn::Ident,
) -> zyn::TokenStream {
    if cfg.skip { return zyn::TokenStream::new(); }
    let rename = cfg.rename.as_deref().unwrap_or("default");
    zyn::zyn! { /* ... */ }
}
```

## `Extract<T>`

General-purpose wrapper for any `FromInput` type:

```rust
#[zyn::element]
fn my_element(
    #[zyn(input)] generics: zyn::Extract<syn::Generics>,
) -> zyn::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    zyn::zyn! { /* ... */ }
}
```

## `inner()`

Both types provide `inner(self) -> T` to take ownership of the wrapped value:

```rust
let config: MyConfig = attr.inner();
let generics: syn::Generics = extract.inner();
```
