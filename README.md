# zyn

A template engine for Rust procedural macros. Write `quote!`-like code with control flow, interpolation pipes, and composable elements.

## Install

```toml
[dependencies]
zyn = "0.0.0"
```

## Quick Start

```rust
use zyn::prelude::*;

// Generate a greeting function with a configurable name
let name = quote::format_ident!("hello_world");
let is_pub = true;

let output: proc_macro2::TokenStream = zyn! {
    @if (is_pub) { pub }
    fn {{ name | snake }}() {
        println!("hello!");
    }
};

// output: pub fn hello_world() { println!("hello!"); }
```

## Template Syntax

### Interpolation

Insert any expression that implements `ToTokens`:

```rust
zyn! { fn {{ name }}() -> {{ ret_type }} {} }
```

### Pipes

Transform interpolated values with pipes. Reference them in snake_case — they resolve to PascalCase structs automatically:

```rust
zyn! {
    fn {{ name | snake }}() {}      // HelloWorld -> hello_world
    const {{ name | screaming }}: &str = ""; // HelloWorld -> HELLO_WORLD
    {{ name | upper }}              // hello -> HELLO
    {{ name | lower }}              // HELLO -> hello
    {{ name | camel }}              // hello_world -> helloWorld
    {{ name | pascal }}             // hello_world -> HelloWorld
    {{ name | kebab }}              // HelloWorld -> "hello-world" (string literal)
}
```

Chain pipes:

```rust
zyn! { {{ name | snake | upper }} }  // HelloWorld -> HELLO_WORLD
```

### Control Flow

#### Conditionals

```rust
zyn! {
    @if (is_async) {
        async fn {{ name }}() {}
    } @else if (is_unsafe) {
        unsafe fn {{ name }}() {}
    } @else {
        fn {{ name }}() {}
    }
}
```

#### Loops

```rust
zyn! {
    @for (name of ["x", "y", "z"].map(|s| quote::format_ident!("{}", s))) {
        pub {{ name }}: f64,
    }
}
// output: pub x: f64, pub y: f64, pub z: f64,
```

#### Pattern Matching

```rust
zyn! {
    @match (kind) {
        Kind::Struct => { struct {{ name }} {} }
        Kind::Enum => { enum {{ name }} {} }
        _ => {}
    }
}
```

#### Compile Errors

```rust
zyn! {
    @if (!valid) {
        @throw "expected a struct"
    }
}
```

### Elements

Reusable template components. Define with `#[element]`, invoke with `@`:

```rust
#[zyn::element]
fn field_decl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    })
}

// Generates struct FieldDecl, referenced as @field_decl in templates:
zyn! {
    @field_decl {
        vis: syn::parse_quote!(pub),
        name: quote::format_ident!("age"),
        ty: syn::parse_quote!(u32),
    }
}
// output: pub age: u32,
```

Elements support children:

```rust
#[zyn::element]
fn wrapper(vis: syn::Visibility, children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote::quote!(#vis struct Foo { #children }))
}

zyn! {
    @wrapper { vis: quote::quote!(pub) } {
        name: String,
        age: u32,
    }
}
```

### Custom Pipes

Define with `#[pipe]`:

```rust
#[zyn::pipe]
fn prefix(input: String) -> proc_macro2::Ident {
    proc_macro2::Ident::new(
        &format!("pfx_{}", input),
        proc_macro2::Span::call_site(),
    )
}

// Usage — generates struct Prefix, referenced as @prefix:
zyn! { {{ name | prefix }} }
```

### Custom Names

Override the template name for elements and pipes:

```rust
#[zyn::element("my_component")]
fn internal_impl(...) -> ... { ... }

// Referenced as @my_component in templates (resolves to MyComponent struct)
```

## Case Conversion Utilities

The `case` module and macros are available for use outside templates:

```rust
use zyn::case;

case::to_snake("HelloWorld")     // "hello_world"
case::to_pascal("hello_world")   // "HelloWorld"
case::to_camel("hello_world")    // "helloWorld"
case::to_screaming("HelloWorld") // "HELLO_WORLD"
case::to_kebab("HelloWorld")     // "hello-world"

// As macros (also work on syn::Ident):
zyn::pascal!("hello_world")           // "HelloWorld"
zyn::pascal!(ident => ident)          // syn::Ident in PascalCase
zyn::snake!(ident => ident)           // syn::Ident in snake_case
```

## License

MIT
