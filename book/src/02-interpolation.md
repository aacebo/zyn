# Interpolation

Insert any expression that implements `ToTokens` with double braces:

```rust,zyn
zyn! { fn {{ name }}() -> {{ ret_type }} {} }
// output: fn hello() -> String {}
```

## Field Access

Dot notation works inside interpolation:

```rust,zyn
zyn! {
    {{ item.field.name }}: {{ item.field.ty }},
}
// output: age: u32,
```

## Method Calls

```rust,zyn
zyn! {
    {{ names.len() }}
}
// output: 3
```

## Groups

Interpolation works inside parenthesized and bracketed groups:

```rust,zyn
zyn! { fn foo(x: {{ ty }}) }
// output: fn foo(x: u32)
zyn! { type Foo = [{{ ty }}; 4]; }
// output: type Foo = [u32; 4];
```
