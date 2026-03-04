# Conditionals

Use `@if` to conditionally include tokens. The condition is any Rust expression that evaluates to `bool`:

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

## Field Access and Method Calls

Conditions can reach into values with field access and method calls:

```rust,zyn
zyn! {
    @if (opts.is_pub) { pub }
    @if (items.is_empty()) { @throw "no items provided" }
    @if (name.to_string().starts_with('_')) {
        @warn "leading underscore is unconventional"
    }
}
```

## Inline Visibility

A common pattern is toggling `pub` based on a flag:

```rust,zyn
let is_pub = true;
let name = quote::format_ident!("my_fn");

zyn! {
    @if (is_pub) { pub } fn {{ name }}() {}
}
// output: pub fn my_fn() {}
```

## Negation and Comparisons

Any boolean expression works in conditions:

```rust,zyn
zyn! {
    @if (!fields.is_empty()) {
        impl {{ name }} {
            pub fn len(&self) -> usize { {{ fields.len() }} }
        }
    }

    @if (variant_count > 1) {
        impl Default for {{ name }} {
            fn default() -> Self { Self::{{ default_variant }} }
        }
    }
}
```

## Combining with Interpolation

`@if` and `{{ }}` compose freely within the same statement:

```rust,zyn
zyn! {
    @if (field.is_optional) {
        pub {{ field.name }}: Option<{{ field.ty }}>,
    } @else {
        pub {{ field.name }}: {{ field.ty }},
    }
}
```
