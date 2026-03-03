# Interpolation

Insert any expression that implements `ToTokens` with double braces:

```zyn
zyn! { fn {{ name }}() -> {{ ret_type }} {} }
```

## Field Access

Dot notation works inside interpolation:

```zyn
zyn! {
    {{ item.field.name }}: {{ item.field.ty }},
}
```

## Method Calls

```zyn
zyn! {
    {{ names.len() }}
}
```

## Groups

Interpolation works inside parenthesized and bracketed groups:

```zyn
zyn! { fn foo(x: {{ ty }}) }
zyn! { type Foo = [{{ ty }}; 4]; }
```
