# Pattern Matching

Use `@match` when you need to generate different code based on a value. The subject is any Rust expression; arms use standard Rust pattern syntax:

```rust,zyn
zyn! {
    @match (kind) {
        Kind::Struct => { struct {{ name }} {} }
        Kind::Enum => { enum {{ name }} {} }
        _ => {}
    }
}
```

## Expression Subjects

The match subject can be any expression, including method calls:

```rust,zyn
zyn! {
    @match (value.len()) {
        0 => { @throw "at least one field is required" }
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

## String Patterns

String slices work as match patterns:

```rust,zyn
zyn! {
    @match (repr.as_str()) {
        "u8"  => { impl From<{{ name }}> for u8  { fn from(v: {{ name }}) -> u8  { v.0 } } }
        "u16" => { impl From<{{ name }}> for u16 { fn from(v: {{ name }}) -> u16 { v.0 } } }
        "u32" => { impl From<{{ name }}> for u32 { fn from(v: {{ name }}) -> u32 { v.0 } } }
        _ => { @throw "unsupported repr" }
    }
}
```

## Multiple Patterns per Arm

Use `|` to match multiple values in one arm:

```rust,zyn
zyn! {
    @match (kind) {
        Kind::Add | Kind::Sub => { fn apply(a: i32, b: i32) -> i32 { a {{ op }} b } }
        Kind::Mul | Kind::Div => { fn apply(a: f64, b: f64) -> f64 { a {{ op }} b } }
    }
}
```

## Combining with `@for`

`@match` and `@for` nest naturally — use `@match` inside a loop to dispatch per-item:

```rust,zyn
zyn! {
    @for (field in fields.iter()) {
        @match (field.kind) {
            FieldKind::Optional => {
                pub {{ field.name }}: Option<{{ field.ty }}>,
            }
            FieldKind::Required => {
                pub {{ field.name }}: {{ field.ty }},
            }
        }
    }
}
```
