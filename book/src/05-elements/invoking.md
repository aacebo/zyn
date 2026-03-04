# Invoking

Reference an element by its snake_case name prefixed with `@`. Props are passed as `name = value` pairs:

```rust,zyn
zyn! {
    @field_decl(
        vis = syn::parse_quote!(pub),
        name = quote::format_ident!("age"),
        ty = syn::parse_quote!(u32),
    )
}
// output: pub age: u32,
```

## Multiple Props

Each prop maps to one field on the generated struct. Pass as many as the element defines:

```rust,zyn
zyn! {
    @method_decl(
        vis = syn::parse_quote!(pub),
        name = quote::format_ident!("get_name"),
        ret = syn::parse_quote!(String),
        body = quote::quote! { self.name.clone() },
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
```

## Trailing Comma

Trailing commas in prop lists are allowed:

```rust,zyn
zyn! {
    @greeting(
        name = quote::format_ident!("hello"),  // trailing comma ok
    )
}
```

## Calling the Same Element Multiple Times

Elements are just structs — invoke them as many times as needed:

```rust,zyn
zyn! {
    @field_decl(vis = syn::parse_quote!(pub), name = quote::format_ident!("id"),   ty = syn::parse_quote!(u64))
    @field_decl(vis = syn::parse_quote!(pub), name = quote::format_ident!("name"), ty = syn::parse_quote!(String))
    @field_decl(vis = syn::parse_quote!(),    name = quote::format_ident!("data"), ty = syn::parse_quote!(Vec<u8>))
}
```
