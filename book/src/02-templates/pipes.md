# Pipes

Pipes transform interpolated values. Add them after a `|`:

```rust,zyn
zyn! {
    fn {{ name | snake }}() {}
}
// output: fn hello_world() {}
```

Pipe names are written in snake_case in templates — they resolve to PascalCase structs automatically.

## Built-in Pipes

| Pipe | Input | Output | Example |
|------|-------|--------|---------|
| `upper` | `HelloWorld` | `HELLOWORLD` | `{{ name \| upper }}` |
| `lower` | `HELLO` | `hello` | `{{ name \| lower }}` |
| `snake` | `HelloWorld` | `hello_world` | `{{ name \| snake }}` |
| `camel` | `hello_world` | `helloWorld` | `{{ name \| camel }}` |
| `pascal` | `hello_world` | `HelloWorld` | `{{ name \| pascal }}` |
| `screaming` | `HelloWorld` | `HELLO_WORLD` | `{{ name \| screaming }}` |
| `kebab` | `HelloWorld` | `"hello-world"` | `{{ name \| kebab }}` |
| `str` | `hello` | `"hello"` | `{{ name \| str }}` |
| `trim` | `__foo__` | `foo` | `{{ name \| trim }}` |
| `plural` | `User` | `Users` | `{{ name \| plural }}` |
| `singular` | `users` | `user` | `{{ name \| singular }}` |

> [!warning]
> `kebab` and `str` return **string literals**, not identifiers, because their output may contain characters invalid in Rust identifiers.

## Chaining

Pipes can be chained. Each pipe receives the output of the previous one:

```rust,zyn
zyn! { {{ name | snake | upper }} }
// HelloWorld -> hello_world -> HELLO_WORLD
```

## Format Pipes

The `ident` and `fmt` pipes take a format pattern via `:` syntax. Use `{}` as the placeholder:

```rust,zyn
zyn! {
    fn {{ name | ident:"get_{}" }}() {}     // hello -> get_hello (as ident)
    fn {{ name | ident:"{}_impl" }}() {}    // hello -> hello_impl (as ident)
    const NAME: &str = {{ name | fmt:"{}" }};  // hello -> "hello" (as string literal)
}
```

`ident` produces an identifier, `fmt` produces a string literal.

Combine with case pipes:

```rust,zyn
zyn! { {{ name | snake | ident:"get_{}" }} }
// HelloWorld -> hello_world -> get_hello_world
```

## Custom Pipes

Define custom pipes with `#[zyn::pipe]`. The first parameter is the input, the return type is the output:

```rust,zyn
#[zyn::pipe]
fn prefix(input: String) -> syn::Ident {
    syn::Ident::new(
        &format!("pfx_{}", input),
        zyn::Span::call_site(),
    )
}
```

This generates a unit struct `Prefix` implementing the `Pipe` trait. Use it by its snake_case name:

```rust,zyn
zyn! { {{ name | prefix }} }
// hello -> pfx_hello
```

### Custom Names

Override the template name:

```rust,zyn
#[zyn::pipe("yell")]
fn make_loud(input: String) -> syn::Ident {
    syn::Ident::new(
        &format!("{}__LOUD", input.to_uppercase()),
        zyn::Span::call_site(),
    )
}

zyn! { {{ name | yell }} }
// hello -> HELLO__LOUD
```

### Chaining with Built-ins

Custom pipes chain with built-in pipes:

```rust,zyn
zyn! { {{ name | snake | prefix }} }
// HelloWorld -> hello_world -> pfx_hello_world
```
