# Zero Parameters

Elements with no parameters can be invoked without parentheses:

```rust,zyn
#[zyn::element]
fn divider() -> proc_macro2::TokenStream {
    zyn::zyn!(const DIVIDER: &str = "---";)
}

zyn! { @divider }
zyn! { @divider() }  // also valid
```

Both forms are equivalent — the `()` is optional when there are no props.

## Common Use Cases

Zero-parameter elements are useful for shared boilerplate that doesn't vary:

```rust,zyn
#[zyn::element]
fn derive_common() -> proc_macro2::TokenStream {
    zyn::zyn!(#[derive(Debug, Clone, PartialEq)])
}

#[zyn::element]
fn serde_attrs() -> proc_macro2::TokenStream {
    quote::quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
    }
}
```

```rust,zyn
zyn! {
    @derive_common
    @serde_attrs
    pub struct {{ name }} {
        @for (field in fields.iter()) {
            @field_decl(vis = field.vis.clone(), name = field.ident.clone().unwrap(), ty = field.ty.clone())
        }
    }
}
```

## With Children

Zero-parameter elements can still accept children — omit the parens entirely:

```rust,zyn
#[zyn::element]
fn section(children: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote! { pub mod section { #children } }
}

zyn! {
    @section {
        pub struct Foo;
        pub struct Bar;
    }
}
```
