# Children

## Children Parameter

Elements can accept children by including a `children: zyn::TokenStream` parameter:

```rust,zyn
#[zyn::element]
fn wrapper(name: syn::Ident, children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::zyn!(struct {{ name }} { {{ children }} })
}

zyn! {
    @wrapper(name = input.ident.clone()) {
        x: i32,
    }
}
// output: struct Foo { x: i32, }
```

## Children-Only Elements

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

## Zero-Parameter Elements

Elements with no parameters can be invoked without parentheses:

```rust,zyn
#[zyn::element]
fn divider() -> zyn::TokenStream {
    zyn::zyn!(const DIVIDER: &str = "---";)
}

zyn! { @divider }
zyn! { @divider() }  // also valid
```

Both forms are equivalent — the `()` is optional when there are no props.

Zero-parameter elements are useful for shared boilerplate:

```rust,zyn
#[zyn::element]
fn derive_common() -> zyn::TokenStream {
    zyn::zyn!(#[derive(Debug, Clone, PartialEq)])
}

zyn! {
    @derive_common
    pub struct {{ name }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
}
```

Zero-parameter elements can still accept children:

```rust,zyn
#[zyn::element]
fn section(children: zyn::TokenStream) -> zyn::TokenStream {
    zyn::zyn! { pub mod section { {{ children }} }}
}

zyn! {
    @section {
        pub struct Foo;
        pub struct Bar;
    }
}
```
