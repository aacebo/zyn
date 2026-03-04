# Custom Names

By default the template name is derived from the function name. Pass a string to `#[zyn::element]` to override it:

```rust,zyn
#[zyn::element("say_hello")]
fn internal_greeting(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

zyn! { @say_hello(name = input.ident.clone()) }
```

The generated struct is still named `SayHello` (PascalCase of the custom name).

## When to Use Custom Names

Custom names are useful when:

- The natural Rust function name is verbose or internal: `fn render_field_declaration` → `@field`
- You want a domain-specific vocabulary: `fn emit_getter_method` → `@getter`
- The function name conflicts with a Rust keyword: `fn type_decl` → `@type_def`

```rust,zyn
#[zyn::element("getter")]
fn emit_getter_method(name: proc_macro2::Ident, ty: syn::Type) -> proc_macro2::TokenStream {
    zyn::zyn! {
        pub fn {{ name | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    }
}

#[zyn::element("setter")]
fn emit_setter_method(name: proc_macro2::Ident, ty: syn::Type) -> proc_macro2::TokenStream {
    zyn::zyn! {
        pub fn {{ name | ident:"set_{}" }}(&mut self, value: {{ ty }}) {
            self.{{ name }} = value;
        }
    }
}
```

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
