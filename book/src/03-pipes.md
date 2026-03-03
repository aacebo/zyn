# Pipes

Pipes transform interpolated values. Add them after a `|`:

```zyn
zyn! {
    fn {{ name | snake }}() {}
}
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

> [!warning]
> `kebab` returns a **string literal**, not an identifier, because hyphens are not valid in Rust identifiers.

## Chaining

Pipes can be chained. Each pipe receives the output of the previous one:

```zyn
zyn! { {{ name | snake | upper }} }
// HelloWorld -> hello_world -> HELLO_WORLD
```

## Format Pipes

The `ident` and `fmt` pipes take a format pattern via `:` syntax. Use `{}` as the placeholder:

```zyn
zyn! {
    fn {{ name | ident:"get_{}" }}() {}     // hello -> get_hello (as ident)
    fn {{ name | ident:"{}_impl" }}() {}    // hello -> hello_impl (as ident)
    const NAME: &str = {{ name | fmt:"{}" }};  // hello -> "hello" (as string literal)
}
```

`ident` produces an identifier, `fmt` produces a string literal.

Combine with case pipes:

```zyn
zyn! { {{ name | snake | ident:"get_{}" }} }
// HelloWorld -> hello_world -> get_hello_world
```
