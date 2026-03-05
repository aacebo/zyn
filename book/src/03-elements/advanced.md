# Advanced

## Custom Names

By default the template name is derived from the function name. Pass a string to `#[zyn::element]` to override it:

```rust,zyn
#[zyn::element("say_hello")]
fn internal_greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

zyn! { @say_hello(name = input.ident.clone()) }
```

The generated struct is still named `SayHello` (PascalCase of the custom name).

Custom names are useful when:

- The natural Rust function name is verbose or internal: `fn render_field_declaration` → `@field`
- You want a domain-specific vocabulary: `fn emit_getter_method` → `@getter`
- The function name conflicts with a Rust keyword: `fn type_decl` → `@type_def`

```rust,zyn
#[zyn::element("getter")]
fn emit_getter_method(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        pub fn {{ name | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    }
}

#[zyn::element("setter")]
fn emit_setter_method(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        pub fn {{ name | ident:"set_{}" }}(&mut self, value: {{ ty }}) {
            self.{{ name }} = value;
        }
    }
}
```

## Namespaced Elements

Elements defined in submodules are referenced with `::` path syntax:

```rust,zyn
mod components {
    #[zyn::element]
    pub fn field_decl(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
        zyn::zyn!({{ name }}: {{ ty }},)
    }
}

zyn! {
    @components::field_decl(
        name = field.ident.clone().unwrap(),
        ty = field.ty.clone(),
    )
}
```

Only the last path segment is PascalCased — `components::field_decl` resolves to `components::FieldDecl`.

### Organizing a Component Library

```rust,zyn
mod impls {
    #[zyn::element]
    pub fn display(name: syn::Ident) -> zyn::TokenStream {
        zyn::zyn! {
            impl ::std::fmt::Display for {{ name }} {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.name)
                }
            }
        }
    }
}

mod fields {
    #[zyn::element]
    pub fn required(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
        zyn::zyn!(pub {{ name }}: {{ ty }},)
    }

    #[zyn::element]
    pub fn optional(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
        zyn::zyn!(pub {{ name }}: Option<{{ ty }}>,)
    }
}
```

```rust,zyn
zyn! {
    pub struct {{ name }} {
        @for (field in fields.iter()) {
            @if (field.is_optional) {
                @fields::optional(name = field.ident.clone().unwrap(), ty = field.ty.clone())
            } @else {
                @fields::required(name = field.ident.clone().unwrap(), ty = field.ty.clone())
            }
        }
    }

    @impls::display(name = name.clone())
}
```

Paths can be as deep as needed — `@a::b::c::my_element(...)` resolves to `a::b::c::MyElement`.

## Elements in Loops

Elements compose naturally with `@for`:

```rust,zyn
zyn! {
    impl {{ struct_name }} {
        @for (field in fields.iter()) {
            @getter(
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
            @setter(
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
```

### Combining with `@if`

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

### Children Blocks in Loops

```rust,zyn
zyn! {
    @for (variant in variants.iter()) {
        @arm(pattern = variant.pat.clone()) {
            Self::{{ variant.name }} => {{ variant.index }},
        }
    }
}
```
