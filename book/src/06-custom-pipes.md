# Custom Pipes

Define custom pipes with `#[zyn::pipe]` to create reusable value transforms.

## Defining

Annotate a function where the first parameter is the input and the return type is the output:

```rust,zyn
#[zyn::pipe]
fn prefix(input: String) -> zyn::types::Ident {
    zyn::types::Ident::new(
        &format!("pfx_{}", input),
        zyn::Span::call_site(),
    )
}
```

This generates a unit struct `Prefix` that implements the `Pipe` trait. The function name (snake_case) becomes the pipe name in templates.

## Using

Reference the pipe by its snake_case name:

```rust,zyn
zyn! { {{ name | prefix }} }
// hello -> pfx_hello
```

## Custom Names

Override the template name:

```rust,zyn
#[zyn::pipe("yell")]
fn make_loud(input: String) -> zyn::types::Ident {
    zyn::types::Ident::new(
        &format!("{}__LOUD", input.to_uppercase()),
        zyn::Span::call_site(),
    )
}

zyn! { {{ name | yell }} }
// hello -> HELLO__LOUD
```

## Chaining with Built-ins

Custom pipes chain with built-in pipes:

```rust,zyn
zyn! { {{ name | snake | prefix }} }
// HelloWorld -> hello_world -> pfx_hello_world
```
