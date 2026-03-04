# zyn vs raw syn + quote

Side-by-side comparisons showing what proc macro code looks like with and without zyn.

---

## 1. Simple struct generation

### Raw syn + quote

```rust
let name = &input.ident;
let fields = extract_fields(&input);

let field_tokens: Vec<_> = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    quote! { pub #name: #ty, }
}).collect();

quote! {
    struct #name {
        #(#field_tokens)*
    }
}
```

### zyn

```rust
zyn! {
    struct {{ input.ident }} {
        @for (f in extract_fields(&input)) {
            pub {{ f.ident }}: {{ f.ty }},
        }
    }
}
```

---

## 2. Conditional visibility

### Raw syn + quote

```rust
let vis = if is_pub {
    quote! { pub }
} else {
    quote! {}
};

let output = quote! {
    #vis fn #name() -> #ret_type {
        #body
    }
};
```

### zyn

```rust
zyn! {
    @if (is_pub) { pub }
    fn {{ name }}() -> {{ ret_type }} {
        {{ body }}
    }
}
```

---

## 3. Enum variant generation with pattern matching

### Raw syn + quote

```rust
let output = match kind {
    Kind::Struct => {
        let field_tokens: Vec<_> = fields.iter().map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            quote! { #name: #ty, }
        }).collect();

        quote! {
            struct #name {
                #(#field_tokens)*
            }
        }
    }
    Kind::Enum => {
        let variant_tokens: Vec<_> = variants.iter().map(|v| {
            let name = &v.ident;
            quote! { #name, }
        }).collect();

        quote! {
            enum #name {
                #(#variant_tokens)*
            }
        }
    }
    _ => {
        quote! { type #name = (); }
    }
};
```

### zyn

```rust
zyn! {
    @match (kind) {
        Kind::Struct => {
            struct {{ name }} {
                @for (f in fields) {
                    {{ f.ident }}: {{ f.ty }},
                }
            }
        }
        Kind::Enum => {
            enum {{ name }} {
                @for (v in variants) {
                    {{ v.ident }},
                }
            }
        }
        _ => {
            type {{ name }} = ();
        }
    }
}
```

---

## 4. Builder pattern with getters/setters

### Raw syn + quote

```rust
let setter_fns: Vec<_> = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    let setter_name = format_ident!("set_{}", name.as_ref().unwrap());

    quote! {
        pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
            self.#name = Some(value);
            self
        }
    }
}).collect();

let getter_fns: Vec<_> = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    let getter_name = format_ident!("get_{}", name.as_ref().unwrap());

    quote! {
        pub fn #getter_name(&self) -> Option<&#ty> {
            self.#name.as_ref()
        }
    }
}).collect();

quote! {
    impl #struct_name {
        #(#setter_fns)*
        #(#getter_fns)*
    }
}
```

### zyn

```rust
zyn! {
    impl {{ struct_name }} {
        @for (f in fields) {
            pub fn {{ f.ident | ident:"set_{}" }}(&mut self, value: {{ f.ty }}) -> &mut Self {
                self.{{ f.ident }} = Some(value);
                self
            }

            pub fn {{ f.ident | ident:"get_{}" }}(&self) -> Option<&{{ f.ty }}> {
                self.{{ f.ident }}.as_ref()
            }
        }
    }
}
```

---

## 5. Reusable components with elements

### Raw syn + quote

```rust
fn render_field(vis: &syn::Visibility, name: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
    quote! { #vis #name: #ty, }
}

fn render_struct(name: &syn::Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let field_tokens: Vec<_> = fields.iter().map(|f| {
        render_field(&f.vis, &f.ident, &f.ty)
    }).collect();

    quote! {
        struct #name {
            #(#field_tokens)*
        }
    }
}
```

### zyn

```rust
#[zyn::element]
fn field_decl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> proc_macro2::TokenStream {
    zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
}

zyn! {
    struct {{ name }} {
        @for (f in fields) {
            @field_decl(vis = f.vis.clone(), name = f.ident.clone(), ty = f.ty.clone())
        }
    }
}
```

---

## 6. Case conversion for derived names

### Raw syn + quote

```rust
use heck::{ToSnakeCase, ToUpperCamelCase, ToShoutySnakeCase};

let name_str = name.to_string();
let snake = format_ident!("{}", name_str.to_snake_case());
let screaming = format_ident!("{}", name_str.to_shouty_snake_case());
let getter = format_ident!("get_{}", name_str.to_snake_case());

quote! {
    fn #snake() {}
    const #screaming: &str = stringify!(#name);
    fn #getter(&self) -> &Self { self }
}
```

### zyn

```rust
zyn! {
    fn {{ name | snake }}() {}
    const {{ name | screaming }}: &str = stringify!({{ name }});
    fn {{ name | ident:"get_{}" }}(&self) -> &Self { self }
}
```

---

## 7. Compile-time validation

### Raw syn + quote

```rust
if fields.is_empty() {
    return syn::Error::new_spanned(&input.ident, "struct must have at least one field")
        .to_compile_error()
        .into();
}

// ... rest of generation
```

### zyn

```rust
zyn! {
    @if (fields.is_empty()) {
        @throw "struct must have at least one field"
    }

    struct {{ name }} {
        @for (f in fields) {
            {{ f.ident }}: {{ f.ty }},
        }
    }
}
```

---

## Summary

| Aspect | Raw syn + quote | zyn |
|--------|----------------|-----|
| Iteration | `.iter().map().collect()` + `#(#tokens)*` | `@for (item in iter) { ... }` |
| Conditionals | `if/else` blocks wrapping `quote!` | `@if (cond) { ... } @else { ... }` inline |
| Pattern matching | `match` wrapping `quote!` | `@match (expr) { arm => { ... } }` inline |
| Case conversion | External crate + `format_ident!` | `{{ name \| snake }}` pipe syntax |
| Name formatting | `format_ident!("get_{}", name)` | `{{ name \| ident:"get_{}" }}` |
| Reusable components | Manual functions returning `TokenStream` | `#[element]` + `@component(props)` |
| Compile errors | Early return with `syn::Error` | `@throw "message"` inline |
| Nesting depth | Deep: closures inside closures | Flat: template reads top-to-bottom |
