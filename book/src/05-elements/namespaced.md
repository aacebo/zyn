# Namespaced Elements

Elements defined in submodules are referenced with `::` path syntax:

```rust,zyn
mod components {
    #[zyn::element]
    pub fn field_decl(name: zyn::types::Ident, ty: zyn::types::Type) -> zyn::TokenStream {
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

Only the last path segment is PascalCased for struct resolution — `components::field_decl` resolves to `components::FieldDecl`.

## Organizing a Component Library

Namespacing lets you group related elements by concern:

```rust,zyn
mod impls {
    #[zyn::element]
    pub fn display(name: zyn::types::Ident) -> zyn::TokenStream {
        zyn::zyn! {
            impl ::std::fmt::Display for {{ name }} {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.name)
                }
            }
        }
    }

    #[zyn::element]
    pub fn debug(name: zyn::types::Ident) -> zyn::TokenStream {
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
    pub fn required(name: zyn::types::Ident, ty: zyn::types::Type) -> zyn::TokenStream {
        zyn::zyn!(pub {{ name }}: {{ ty }},)
    }

    #[zyn::element]
    pub fn optional(name: zyn::types::Ident, ty: zyn::types::Type) -> zyn::TokenStream {
        zyn::zyn!(pub {{ name }}: Option<{{ ty }}>,)
    }
}
```

```rust,zyn
let name = &input.ident;

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
    @impls::debug(name = name.clone())
}
```

## Deeper Paths

Paths can be as deep as needed — `@a::b::c::my_element(...)` resolves to `a::b::c::MyElement`.
