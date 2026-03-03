# Control Flow

All control flow directives start with `@`.

## Conditionals

```zyn
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

Conditions support field access and method calls:

```zyn
zyn! {
    @if (opts.is_pub) { pub }
    @if (items.is_empty()) { @throw "no items" }
}
```

## Loops

Iterate with `@for`. The keyword is `of`, not `in`:

```zyn
zyn! {
    @for (name of names) {
        pub {{ name }}: f64,
    }
}
// output: pub x: f64, pub y: f64, pub z: f64,
```

> [!important]
> Use `of` instead of `in` — `in` is a reserved keyword in Rust's macro token stream.

Inline iterators work:

```zyn
zyn! {
    @for (name of ["x", "y", "z"].map(|s| quote::format_ident!("{}", s))) {
        pub {{ name }}: f64,
    }
}
```

## Pattern Matching

```zyn
zyn! {
    @match (kind) {
        Kind::Struct => { struct {{ name }} {} }
        Kind::Enum => { enum {{ name }} {} }
        _ => {}
    }
}
```

Expressions work in the match subject:

```zyn
zyn! {
    @match (value.len()) {
        5 => { struct Five; }
        _ => { struct Other; }
    }
}
```

## Compile Errors

Emit a `compile_error!` with `@throw`:

```zyn
zyn! {
    @if (!valid) {
        @throw "expected a struct"
    }
}
```

## Nesting

Control flow directives nest freely:

```zyn
zyn! {
    @for (item of items) {
        @if (item.1) {
            fn {{ item.0 }}() {}
        }
    }
}
```
