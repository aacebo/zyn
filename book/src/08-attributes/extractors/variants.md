# `Variants`

Extracts `Vec<syn::Variant>` from an enum input. Errors at compile time if the input is not an enum.

Implements `Deref` and `DerefMut` to `Vec<syn::Variant>`.

```rust
#[zyn::element]
fn enum_element(
    #[zyn(input)] variants: zyn::Variants,
) -> zyn::TokenStream {
    let count = variants.len();
    zyn::zyn!(const COUNT: usize = {{ count }};)
}
```

Iterate over variants:

```rust
#[zyn::element]
fn variant_names(
    #[zyn(input)] variants: zyn::Variants,
) -> zyn::TokenStream {
    zyn::zyn! {
        @for (v in variants.iter()) {
            const {{ v.ident | screaming }}: &str = {{ v.ident | str }};
        }
    }
}
```

Works with both derive and item enum inputs.
