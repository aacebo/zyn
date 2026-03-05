# zyn

A template engine for Rust procedural macros. Write code-generation templates with control flow, interpolation pipes, and composable elements.

<a href="https://aacebo.github.io/zyn" target="_blank">
    <img src="https://img.shields.io/badge/📖 Getting Started-blue?style=for-the-badge" />
</a>

## Quick Start

```rust
use zyn::prelude::*;

zyn! {
    @if (input.vis == syn::Visibility::Public(..)) { pub }
    fn {{ input.ident | snake }}() {
        println!("hello!");
    }
}
// output: pub fn hello_world() { println!("hello!"); }
```

## Template Syntax

### Interpolation

Insert any expression — field access, method calls, anything that implements `ToTokens`:

```rust
zyn! {
    fn {{ input.ident }}() -> {{ field.ty }} {}
    {{ input.ident | str }}
    {{ fields.len() }}
}
```

### Pipes

Transform values inline:

```rust
zyn! {
    fn {{ name | snake }}() {}                 // HelloWorld -> hello_world
    const {{ name | screaming }}: &str = "";   // HelloWorld -> HELLO_WORLD
    {{ name | upper }}                         // hello -> HELLO
    {{ name | lower }}                         // HELLO -> hello
    {{ name | camel }}                         // hello_world -> helloWorld
    {{ name | pascal }}                        // hello_world -> HelloWorld
    {{ name | kebab }}                         // HelloWorld -> "hello-world"
    {{ name | str }}                           // hello -> "hello"
    {{ name | trim }}                          // __foo -> foo
    {{ name | plural }}                        // User -> Users
    {{ name | singular }}                      // users -> user
    {{ name | snake | upper }}                 // HelloWorld -> HELLO_WORLD
    fn {{ name | ident:"get_{}" }}() {}        // hello -> get_hello
    const X: &str = {{ name | fmt:"{}!" }};    // hello -> "hello!"
}
```

### Control Flow

```rust
zyn! {
    @if (input.is_pub) { pub }
    @else if (input.is_crate) { pub(crate) }

    struct {{ input.ident }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }

    @match (input.kind) {
        Kind::Struct => { impl {{ input.ident }} {} }
        _ => {}
    }

    @for (fields.len()) {
        ,
    }
}
```

### Diagnostics

```rust
zyn! {
    @if (fields.is_empty()) {
        @throw "expected at least one field" {
            @note "empty structs are not supported"
            @help "add a field to the struct"
        }
    }
    @if (input.is_legacy) {
        @warn "this derive is deprecated"
    }
}
```

### Elements

Reusable template components:

```rust
#[zyn::element]
fn field_decl(
    vis: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,
) -> zyn::TokenStream {
    zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
}

zyn! {
    struct {{ input.ident }} {
        @for (field in fields.iter()) {
            @field_decl(
                vis = field.vis.clone(),
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
// output: struct User { pub name: String, pub age: u32, }
```

Children:

```rust
#[zyn::element]
fn wrapper(
    vis: syn::Visibility,
    children: zyn::TokenStream,
) -> zyn::TokenStream {
    zyn::zyn! { {{ vis }} struct Foo { {{ children }} } }
}

zyn! {
    @wrapper(vis = input.vis.clone()) {
        name: String,
    }
}
```

Zero parameters:

```rust
#[zyn::element]
fn divider() -> zyn::TokenStream {
    zyn::zyn!(const DIVIDER: &str = "---";)
}

zyn! { @divider }
```

### Custom Pipes

```rust
#[zyn::pipe]
fn prefix(input: String) -> syn::Ident {
    syn::Ident::new(
        &format!("pfx_{}", input),
        zyn::Span::call_site(),
    )
}

zyn! { {{ name | prefix }} }
// hello -> pfx_hello
```

### Attribute Parsing

Derive typed attribute structs — `FromInput` handles extraction automatically:

```rust
#[derive(zyn::Attribute)]
#[zyn("builder", about = "Configure the builder derive")]
struct BuilderConfig {
    #[zyn(default = "build".to_string())]
    method: String,
    skip: bool,
}

// In your proc macro:
let input: zyn::Input = real_derive_input.into();
let cfg = BuilderConfig::from_input(&input)?;
```

For element params, use `zyn::Attr<T>` to auto-resolve from the `input` context:

```rust
#[zyn::element]
fn builder_method(
    #[zyn(input)] cfg: zyn::Attr<BuilderConfig>,   // auto-resolved from input, not a prop
    name: syn::Ident,                  // regular prop, passed at @call site
) -> zyn::TokenStream {
    let method = zyn::format_ident!("{}", cfg.method);
    zyn::zyn! { pub fn {{ method }}(self) -> Self { self } }
}
```

### Case Conversion

Available outside templates via the `case` module:

```rust
zyn::case::to_snake("HelloWorld")     // "hello_world"
zyn::case::to_pascal("hello_world")   // "HelloWorld"
zyn::case::to_camel("hello_world")    // "helloWorld"
zyn::case::to_screaming("HelloWorld") // "HELLO_WORLD"
zyn::case::to_kebab("HelloWorld")     // "hello-world"
```

## License

MIT
