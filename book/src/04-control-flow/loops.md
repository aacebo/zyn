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
let field_names = fields.iter().map(|f| f.ident.clone().unwrap());

zyn! {
    @for (name in field_names) {
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
// output (given ident = User, fields = [name: String, age: u32]):
//   impl User {
//       pub fn name(&self) -> &String { &self.name }
//       pub fn age(&self) -> &u32 { &self.age }
//   }
```

## Using `.enumerate()`

Access the index alongside the value using standard iterator adapters:

```rust,zyn
zyn! {
    @for ((i, variant) in variants.iter().enumerate()) {
        const {{ variant.ident | screaming }}: usize = {{ i }};
    }
}
// output (given variants = [Red, Green, Blue]):
//   const RED: usize = 0;
//   const GREEN: usize = 1;
//   const BLUE: usize = 2;
```

## Filtering

Chain `.filter()` to skip items:

```rust,zyn
zyn! {
    @for (field in fields.iter().filter(|f| f.is_pub)) {
        {{ field.ident }}: {{ field.ty }},
    }
}
// output (given fields = [pub name: String, age: u32, pub email: String]):
//   name: String,
//   email: String,
```

## Count-based Loops

`@for` also accepts a count expression without a binding — the body is repeated N times:

```rust,zyn
zyn! {
    @for (3) { x, }
}
// output: x, x, x,
```

Any expression that evaluates to a range-compatible value works:

```rust,zyn
zyn! {
    @for (fields.len()) {
        ,
    }
}
// output (given fields.len() = 3): , , ,
```

For indexed access, use the standard range form with a binding:

```rust,zyn
zyn! {
    @for (i in 0..fields.len()) {
        {{ fields[i].ident }}: {{ fields[i].ty }},
    }
}
// output (given fields = [name: String, age: u32]):
//   name: String,
//   age: u32,
```

## Comma-separated Expansion

In `quote!`, repeating a list with separators uses the `#(#items),*` pattern. In zyn, just put the separator inside the loop body:

```rust,zyn
zyn! {
    [
        @for (field in fields.iter()) {
            {{ field.ident | str }},
        }
    ]
}
// output (given fields = [name, age, email]):
//   ["name", "age", "email",]
```

This works for any separator — commas, pipes, semicolons, `|`, etc. The trailing separator is always emitted; Rust's grammar allows trailing commas in most positions.

For function-style argument lists:

```rust,zyn
zyn! {
    fn new(
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    ) -> Self {
        Self {
            @for (field in fields.iter()) {
                {{ field.ident }},
            }
        }
    }
}
// output (given fields = [name: String, age: u32]):
//   fn new(name: String, age: u32,) -> Self {
//       Self { name, age, }
//   }
```

## Empty Iterators

If the iterator is empty, the body emits nothing and compilation continues normally — there is no error.
