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

Supports field access, method calls, and any Rust expression:

```rust
zyn! {
    {{ item.field.name }}: {{ item.field.ty }},
    {{ names.len() }}
}
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

#### Format pipes

Pipes can take arguments via `:` syntax. The `ident` and `fmt` pipes use a `{}` placeholder:

```rust
zyn! {
    fn {{ name | ident:"get_{}" }}() {}     // hello -> fn get_hello() {}
    fn {{ name | ident:"{}_impl" }}() {}    // hello -> fn hello_impl() {}
    const NAME: &str = {{ name | fmt:"{}" }};  // hello -> const NAME: &str = "hello";
}
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

Conditions support field access and method calls:

```rust
zyn! {
    @if (opts.is_pub) { pub }
    @if (items.is_empty()) { @throw "no items" }
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
    @field_decl(
        vis = syn::parse_quote!(pub),
        name = quote::format_ident!("age"),
        ty = syn::parse_quote!(u32),
    )
}
// output: pub age: u32,
```

Elements can have zero parameters. Parentheses are optional when there are no props:

```rust
#[zyn::element]
fn divider() -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn!(const DIVIDER: &str = "---";))
}

// All equivalent:
zyn! { @divider }
zyn! { @divider() }
```

Elements support children via a `children` parameter:

```rust
#[zyn::element]
fn wrapper(vis: syn::Visibility, children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote::quote!(#vis struct Foo { #children }))
}

zyn! {
    @wrapper(vis = quote::quote!(pub)) {
        name: String,
        age: u32,
    }
}
```

Children-only elements can omit parens entirely:

```rust
#[zyn::element]
fn container(children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote::quote!(mod inner { #children }))
}

zyn! {
    @container {
        struct Foo;
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

// Generates struct Prefix, used as {{ name | prefix }}:
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
