# Namespaced Elements

Elements defined in submodules are referenced with `::` path syntax:

```rust,zyn
mod components {
    #[zyn::element]
    pub fn field_decl(name: proc_macro2::Ident, ty: proc_macro2::Ident) -> proc_macro2::TokenStream {
        zyn::zyn!({{ name }}: {{ ty }},)
    }
}

zyn! {
    @components::field_decl(
        name = quote::format_ident!("age"),
        ty = quote::format_ident!("u32"),
    )
}
```

Only the last path segment is PascalCased for struct resolution — `components::field_decl` resolves to `components::FieldDecl`.

## Organizing a Component Library

Namespacing lets you group related elements by concern:

```rust,zyn
mod impls {
    #[zyn::element]
    pub fn display(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
        zyn::zyn! {
            impl ::std::fmt::Display for {{ name }} {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.name)
                }
            }
        }
    }

    #[zyn::element]
    pub fn debug(name: proc_macro2::Ident) -> proc_macro2::TokenStream {
        zyn::zyn! {
            impl ::std::fmt::Debug for {{ name }} {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.debug_struct(stringify!({{ name }})).finish()
                }
            }
        }
    }
}

mod fields {
    #[zyn::element]
    pub fn required(name: proc_macro2::Ident, ty: syn::Type) -> proc_macro2::TokenStream {
        zyn::zyn!(pub {{ name }}: {{ ty }},)
    }

    #[zyn::element]
    pub fn optional(name: proc_macro2::Ident, ty: syn::Type) -> proc_macro2::TokenStream {
        zyn::zyn!(pub {{ name }}: Option<{{ ty }}>,)
    }
}
```

```rust,zyn
zyn! {
    pub struct {{ name }} {
        @fields::required(name = quote::format_ident!("id"), ty = syn::parse_quote!(u64))
        @fields::optional(name = quote::format_ident!("label"), ty = syn::parse_quote!(String))
    }

    @impls::display(name = quote::format_ident!("MyStruct"))
    @impls::debug(name = quote::format_ident!("MyStruct"))
}
```

## Deeper Paths

Paths can be as deep as needed — `@a::b::c::my_element(...)` resolves to `a::b::c::MyElement`.
