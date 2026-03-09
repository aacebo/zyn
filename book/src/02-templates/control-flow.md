# Control Flow

All control flow directives start with `@`.

## `@if`

Conditionally include tokens. The condition is any Rust expression that evaluates to `bool`:

```rust,zyn
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

Conditions can use field access and method calls:

```rust,zyn
zyn! {
    @if (opts.is_pub) { pub }
    @if (!fields.is_empty()) {
        impl {{ name }} {
            pub fn len(&self) -> usize { {{ fields.len() }} }
        }
    }
}
```

A common pattern is toggling `pub` based on a flag:

```rust,zyn
let is_pub = true;
let name = &input.ident;

zyn! {
    @if (is_pub) { pub } fn {{ name }}() {}
}
// output: pub fn my_fn() {}
```

`@if` and `{{ }}` compose freely:

```rust,zyn
zyn! {
    @if (field.is_optional) {
        pub {{ field.name }}: Option<{{ field.ty }}>,
    } @else {
        pub {{ field.name }}: {{ field.ty }},
    }
}
```

## `@for`

Iterate over any value that produces an iterator:

```rust,zyn
zyn! {
    @for (name in names) {
        pub {{ name }}: f64,
    }
}
// output: pub x: f64, pub y: f64, pub z: f64,
```

The iterator expression can be any Rust expression:

```rust,zyn
let field_names = fields.iter().map(|f| f.ident.clone().unwrap());

zyn! {
    @for (name in field_names) {
        pub {{ name }}: f64,
    }
}
```

### Iterating Over Struct Fields

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

### `.enumerate()`

```rust,zyn
zyn! {
    @for ((i, variant) in variants.iter().enumerate()) {
        const {{ variant.ident | screaming }}: usize = {{ i }};
    }
}
// output: const RED: usize = 0; const GREEN: usize = 1; const BLUE: usize = 2;
```

### Filtering

```rust,zyn
zyn! {
    @for (field in fields.iter().filter(|f| f.is_pub)) {
        {{ field.ident }}: {{ field.ty }},
    }
}
```

### Count-based Loops

`@for` also accepts a count expression without a binding:

```rust,zyn
zyn! {
    @for (3) { x, }
}
// output: x, x, x,
```

For indexed access, use a range:

```rust,zyn
zyn! {
    @for (i in 0..fields.len()) {
        {{ fields[i].ident }}: {{ fields[i].ty }},
    }
}
```

### Comma-separated Expansion

In `quote!`, repeating with separators uses `#(#items),*`. In zyn, put the separator in the loop:

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
```

### Empty Iterators

If the iterator is empty, the body emits nothing — no error.

## `@match`

Generate different code based on a value:

```rust,zyn
zyn! {
    @match (kind) {
        Kind::Struct => { struct {{ name }} {} }
        Kind::Enum => { enum {{ name }} {} }
        _ => {}
    }
}
```

### Expression Subjects

```rust,zyn
zyn! {
    @match (value.len()) {
        0 => { compile_error!("at least one field is required"); }
        1 => { struct {{ name }}({{ fields[0].ty }}); }
        _ => {
            struct {{ name }} {
                @for (field in fields.iter()) {
                    pub {{ field.ident }}: {{ field.ty }},
                }
            }
        }
    }
}
```

### String Patterns

```rust,zyn
zyn! {
    @match (repr.as_str()) {
        "u8"  => { impl From<{{ name }}> for u8  { fn from(v: {{ name }}) -> u8  { v.0 }} }
        "u16" => { impl From<{{ name }}> for u16 { fn from(v: {{ name }}) -> u16 { v.0 }} }
        "u32" => { impl From<{{ name }}> for u32 { fn from(v: {{ name }}) -> u32 { v.0 }} }
        _ => { compile_error!("unsupported repr"); }
    }
}
```

### Multiple Patterns per Arm

```rust,zyn
zyn! {
    @match (kind) {
        Kind::Add | Kind::Sub => { fn apply(a: i32, b: i32) -> i32 { a {{ op }} b }}
        Kind::Mul | Kind::Div => { fn apply(a: f64, b: f64) -> f64 { a {{ op }} b }}
    }
}
```

## Nesting

All directives nest freely:

```rust,zyn
zyn! {
    @for (variant in variants.iter()) {
        @if (variant.is_enabled) {
            @match (variant.kind) {
                VariantKind::Unit => {
                    {{ variant.name }},
                }
                VariantKind::Tuple => {
                    {{ variant.name }}({{ variant.ty }}),
                }
                VariantKind::Struct => {
                    {{ variant.name }} {
                        @for (field in variant.fields.iter()) {
                            {{ field.name }}: {{ field.ty }},
                        }
                    },
                }
            }
        }
    }
}
```
