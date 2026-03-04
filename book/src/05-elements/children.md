# Children

Elements can accept children by including a `children: proc_macro2::TokenStream` parameter:

```rust,zyn
#[zyn::element]
fn wrapper(name: proc_macro2::Ident, children: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    zyn::zyn!(struct {{ name }} { {{ children }} })
}

zyn! {
    @wrapper(name = input.ident.clone()) {
        x: i32,
    }
}
// output: struct Foo { x: i32, }
```

Children-only elements can omit parentheses entirely:

```rust,zyn
#[zyn::element]
fn container(children: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    zyn::zyn!(mod inner { {{ children }} })
}

zyn! {
    @container {
        struct Foo;
    }
}
```
