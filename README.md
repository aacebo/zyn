# zyn

A template engine for Rust procedural macros. Write code-generation templates with control flow, interpolation pipes, and composable elements.

<a href="https://aacebo.github.io/zyn" target="_blank">
    <img src="https://img.shields.io/badge/📖 Getting Started-blue?style=for-the-badge" />
</a>

## Why?

Building proc macros in Rust means pulling together `syn`, `quote`, `proc_macro2`, a case-conversion crate, `proc-macro-error` for diagnostics, and often hand-rolling the same attribute-parsing and helper-function patterns across every project. The result is verbose, fragmented, and hard to maintain:

- **Reimplementing the same helpers everywhere** — every proc macro crate writes its own `extract_fields`, `get_ident_name`, attribute-argument parser, etc.
- **`quote!` syntax is limited** — no control flow, no pipes, no components. You end up with `let` bindings and value coercion outside the template, then `#(#tokens)*` collection patterns inside it.
- **Low-level APIs make simple things complex** — generating a getter for each field means `.iter().map(|f| { ... }).collect::<Vec<_>>()` wrapped in `quote!`, with `format_ident!` for name construction.
- **No consistent diagnostics** — `compile_error!` for errors, the deprecated-item trick for warnings, `proc-macro-error` crate for richer messages — all different approaches, none built-in.
- **No standard attribute parsing types** — every project re-invents `Args`, `Arg`, and attribute extension traits from scratch.
- **Fragmented toolchain** — `proc-macro-error`, `proc-macro2-diagnostics`, `darling`, `heck` for case conversion — five crates doing five things that should be one.

zyn replaces all of that with a single framework:

```rust
zyn! {
    @for (field in fields.iter()) {
        pub fn {{ field.ident | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
            &self.{{ field.ident }}
        }
    }
}
```

- **Control flow lives inline** — `@if`, `@for`, `@match` replace the `if/else` + `quote!` + collect pattern
- **Pipes replace string manipulation** — `{{ name | snake }}`, `{{ name | ident:"get_{}" }}` — no external crates needed
- **Elements replace helper functions** — `@field_decl(...)` instead of manual `fn render_field(...) -> TokenStream`
- **Diagnostics are first-class** — `@throw`, `@warn`, `@note`, `@help` with nested context — one syntax, not three crates
- **Attribute parsing built in** — `Arg`, `Args`, `AttrExt`, `AttrsExt` — no reinventing the wheel
- **One dependency** — case conversion, diagnostics, attribute parsing, template expansion — all in one crate

No runtime cost. No new dependencies for your users. Everything expands at compile time into the same `TokenStream`-building code you'd write by hand.

## Quick Start

```rust
use zyn::prelude::*;

zyn! {
    @if (input.vis == zyn::syn::Visibility::Public(..)) { pub }
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
    vis: zyn::syn::Visibility,
    name: zyn::syn::Ident,
    ty: zyn::syn::Type,
) -> zyn::proc_macro2::TokenStream {
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
    vis: zyn::syn::Visibility,
    children: zyn::proc_macro2::TokenStream,
) -> zyn::proc_macro2::TokenStream {
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
fn divider() -> zyn::proc_macro2::TokenStream {
    zyn::zyn!(const DIVIDER: &str = "---";)
}

zyn! { @divider }
```

### Custom Pipes

```rust
#[zyn::pipe]
fn prefix(input: String) -> zyn::proc_macro2::Ident {
    zyn::proc_macro2::Ident::new(
        &format!("pfx_{}", input),
        zyn::proc_macro2::Span::call_site(),
    )
}

zyn! { {{ name | prefix }} }
// hello -> pfx_hello
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
