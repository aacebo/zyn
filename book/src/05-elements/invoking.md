# Invoking

Reference an element by its snake_case name prefixed with `@`. Props are passed as `name = value` pairs:

```rust,zyn
zyn! {
    @field_decl(
        vis = field.vis.clone(),
        name = field.ident.clone().unwrap(),
        ty = field.ty.clone(),
    )
}
// output: pub age: u32,
```

## Multiple Props

Each prop maps to one field on the generated struct. Pass as many as the element defines:

```rust,zyn
let body = zyn::zyn! { self.{{ field.ident }}.clone() };

zyn! {
    @method_decl(
        vis = field.vis.clone(),
        name = field.ident.clone().unwrap(),
        ret = field.ty.clone(),
        body = body,
    )
}
```

## Inline Expressions

Prop values are raw Rust expressions — any expression that produces the right type works:

```rust,zyn
let fields = extract_fields(&input);

zyn! {
    @for (field in fields.iter()) {
        @field_decl(
            vis = field.vis.clone(),
            name = field.ident.clone().unwrap(),
            ty = field.ty.clone(),
        )
    }
}
// output (given fields = [pub name: String, pub age: u32]):
//   pub name: String,
//   pub age: u32,
```

## Trailing Comma

Trailing commas in prop lists are allowed:

```rust,zyn
zyn! {
    @greeting(
        name = input.ident.clone(),  // trailing comma ok
    )
}
```

## Calling the Same Element Multiple Times

Elements are just structs — invoke them as many times as needed:

```rust,zyn
zyn! {
    @for (field in fields.iter()) {
        @field_decl(
            vis = field.vis.clone(),
            name = field.ident.clone().unwrap(),
            ty = field.ty.clone(),
        )
    }
}
// output:
//   pub id: u64,
//   pub name: String,
//   data: Vec<u8>,
```
