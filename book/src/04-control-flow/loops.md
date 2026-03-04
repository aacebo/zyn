# Loops

Use `@for (binding in iter)` to iterate over any value that produces an iterator. The body is repeated for each item:

```rust,zyn
zyn! {
    @for (name in names) {
        pub {{ name }}: f64,
    }
}
// output: pub x: f64, pub y: f64, pub z: f64,
```

## Inline Iterators

The iterator expression can be any Rust expression, including method chains and closures:

```rust,zyn
zyn! {
    @for (name in ["x", "y", "z"].map(|s| quote::format_ident!("{}", s))) {
        pub {{ name }}: f64,
    }
}
```

## Iterating Over Struct Fields

A common use case is generating code for each field of an input struct:

```rust,zyn
zyn! {
    impl {{ input.ident }} {
        @for (field in input.fields.iter()) {
            pub fn {{ field.ident | snake }}(&self) -> &{{ field.ty }} {
                &self.{{ field.ident }}
            }
        }
    }
}
```

## Using `.enumerate()`

Access the index alongside the value using standard iterator adapters:

```rust,zyn
zyn! {
    @for ((i, variant) in variants.iter().enumerate()) {
        const {{ variant.ident | screaming }}: usize = {{ i }};
    }
}
```

## Filtering

Chain `.filter()` to skip items:

```rust,zyn
zyn! {
    @for (field in fields.iter().filter(|f| f.is_pub)) {
        {{ field.ident }}: {{ field.ty }},
    }
}
```

## Empty Iterators

If the iterator is empty, the body emits nothing and compilation continues normally — there is no error.
