# Case Conversion

Zyn provides case conversion utilities that work both inside and outside templates.

## Functions

The `zyn::case` module has standalone conversion functions:

```rust
use zyn::case;

case::to_snake("HelloWorld")     // "hello_world"
case::to_pascal("hello_world")   // "HelloWorld"
case::to_camel("hello_world")    // "helloWorld"
case::to_screaming("HelloWorld") // "HELLO_WORLD"
case::to_kebab("HelloWorld")     // "hello-world"
```

## Macros

Case conversion macros work with strings, `syn::Ident`, and token streams:

```rust
// String input -> String output
zyn::pascal!("hello_world")        // "HelloWorld"
zyn::snake!("HelloWorld")          // "hello_world"

// Ident input -> Ident output
zyn::pascal!(ident => ident)       // syn::Ident in PascalCase
zyn::snake!(ident => ident)        // syn::Ident in snake_case

// Ident input -> TokenStream output
zyn::pascal!(ident => token_stream)
```

## In Templates

Inside `zyn!`, use the equivalent pipes:

```rust,zyn
zyn! {
    fn {{ name | snake }}() {}
    struct {{ name | pascal }} {}
    const {{ name | screaming }}: &str = "";
}
```

See [Pipes](./pipes.md) for the full list.
