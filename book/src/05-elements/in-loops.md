# In Loops

Elements compose naturally with `@for`:

```rust,zyn
zyn! {
    @for (name in names) {
        @greeting(name = name.clone())
    }
}
```

## Generating Per-Field Code

The most common pattern in proc macros — emit one element per struct field:

```rust,zyn
zyn! {
    impl {{ struct_name }} {
        @for (field in fields.iter()) {
            @getter(
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
```

## Combining with `@if`

Filter which items get expanded:

```rust,zyn
zyn! {
    @for (field in fields.iter()) {
        @if (field.attrs.has_attr("skip")) {}
        @else {
            @field_decl(
                vis = field.vis.clone(),
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
```

## Multiple Elements Per Iteration

Emit more than one element per loop iteration:

```rust,zyn
zyn! {
    impl {{ name }} {
        @for (field in fields.iter()) {
            @getter(name = field.ident.clone().unwrap(), ty = field.ty.clone())
            @setter(name = field.ident.clone().unwrap(), ty = field.ty.clone())
        }
    }
}
```

## Elements That Accept Children in Loops

Pass a children block with per-item content:

```rust,zyn
zyn! {
    @for (variant in variants.iter()) {
        @arm(pattern = variant.pat.clone()) {
            Self::{{ variant.name }} => {{ variant.index }},
        }
    }
}
```
