# Nesting

All control flow directives nest freely inside each other.

## `@for` + `@if`

Filter or conditionally emit within a loop:

```rust,zyn
zyn! {
    @for (item in items) {
        @if (item.1) {
            fn {{ item.0 }}() {}
        }
    }
}
// output (given items = [(foo, true), (bar, false), (baz, true)]):
//   fn foo() {}
//   fn baz() {}
```

## `@for` + `@match`

Dispatch per-item inside a loop:

```rust,zyn
zyn! {
    impl {{ name }} {
        @for (field in fields.iter()) {
            @match (field.vis) {
                Visibility::Public => {
                    pub fn {{ field.name }}(&self) -> &{{ field.ty }} {
                        &self.{{ field.name }}
                    }
                }
                _ => {}
            }
        }
    }
}
```

## `@if` + `@for`

Wrap a whole loop in a condition:

```rust,zyn
zyn! {
    @if (!fields.is_empty()) {
        impl {{ name }} {
            @for (field in fields.iter()) {
                pub {{ field.name }}: {{ field.ty }},
            }
        }
    }
}
// output (given name = User, fields = [age: u32, email: String]):
//   impl User {
//       pub age: u32,
//       pub email: String,
//   }
```

## Three Levels Deep

Directives can nest as deep as needed:

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
