# Children

Elements can accept children by including a `children: zyn::TokenStream` parameter:

```rust,zyn
#[zyn::element]
fn wrapper(name: zyn::types::Ident, children: zyn::TokenStream) -> zyn::TokenStream {
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
fn container(children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::zyn!(mod inner { {{ children }} })
}

zyn! {
    @container {
        struct Foo;
    }
}
```
