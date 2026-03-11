# Getting Started

## Installation

Add zyn to your proc-macro crate:

```toml
[dependencies]
zyn = "0.4"
```

Zyn re-exports `syn`, `quote`, and `proc-macro2` — you don't need to add them separately. Access them as `zyn::syn`, `zyn::quote`, and `zyn::proc_macro2`.

### Feature flags

| Feature | Default | Description |
|---------|---------|-------------|
| `derive` | yes | Proc macros (`zyn!`, `#[zyn::element]`, `#[zyn::pipe]`, `#[zyn::derive]`, `#[zyn::attribute]`), extractors (`Extract<T>`, `Attr<T>`, `Fields`, `Variants`, `Data<T>`), diagnostics (`error!`, `warn!`, `note!`, `help!`, `bail!`), and `#[derive(Attribute)]` |
| `ext` | no | Extension traits (`AttrExt`, `MetaExt`, `FieldExt`, `FieldsExt`, `TypeExt`, `DataExt`, `ItemExt`, `VariantExt`) for working with `syn` AST types |
| `pretty` | no | Pretty-printed debug output via `prettyplease` — use `debug = "pretty"` on any macro attribute |
| `diagnostics` | no | Rich compiler diagnostics via `proc-macro2-diagnostics` — errors, warnings, notes, and help messages with proper spans |

To enable optional features:

```toml
[dependencies]
zyn = { version = "0.4", features = ["ext", "pretty", "diagnostics"] }
```

## Your first template

The `zyn!` macro is a template engine that returns a `zyn::TokenStream`. Everything outside `{{ }}` and `@` directives passes through as literal tokens, just like `quote!`:

```rust
use zyn::prelude::*;

let name = &input.ident;
let tokens: zyn::TokenStream = zyn! {
    pub struct {{ name }}Builder {
        ready: bool,
    }
};
```

`{{ expr }}` interpolates any value that implements `ToTokens` — identifiers, types, expressions, even other token streams.

## Pipes

Pipes transform interpolated values inline with `|`:

```rust
zyn! {
    pub fn {{ name | snake }}(&self) -> &Self {
        &self
    }
}
```

Chain multiple pipes and format identifiers:

```rust
{{ name | snake | ident:"get_{}" }}
```

Built-in pipes include `snake`, `camel`, `pascal`, `screaming`, `kebab`, `upper`, `lower`, `plural`, `singular`, `trim`, `str`, `ident`, and `fmt`. See [Pipes](./02-templates/pipes.md) for the full list.

## Control flow

Templates support `@if`, `@for`, and `@match` directives:

```rust
zyn! {
    impl {{ ident }} {
        @for (field in fields.iter()) {
            pub fn {{ field.ident | snake }}(&self) -> &{{ field.ty }} {
                &self.{{ field.ident }}
            }
        }
    }
}
```

Conditionals:

```rust
zyn! {
    @if (is_pub) { pub } struct {{ name }} {
        @for (field in fields.iter()) {
            @if (field.vis == syn::Visibility::Public(Default::default())) {
                {{ field.ident }}: {{ field.ty }},
            }
        }
    }
}
```

See [Control Flow](./02-templates/control-flow.md) for `@match` and nested directives.

## Elements

When a template pattern repeats, extract it into a reusable element with `#[zyn::element]`:

```rust
#[zyn::element]
fn getter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        pub fn {{ name | snake | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    }
}
```

Invoke elements inside templates with `@`:

```rust
zyn! {
    impl {{ ident }} {
        @for (field in fields.iter()) {
            @getter(
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
```

Elements are compiled like function calls — they accept typed parameters and return a `TokenStream`. See [Elements](./03-elements/index.html) for children, extractors, and diagnostics.

## Wiring it up

Use `#[zyn::derive]` to turn your templates into a real proc macro. Parameters marked `#[zyn(input)]` are automatically extracted from the derive input:

```rust
#[zyn::element]
fn getter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        pub fn {{ name | snake | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    }
}

#[zyn::derive]
fn my_getters(
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
    #[zyn(input)] fields: zyn::Fields,
) -> zyn::TokenStream {
    zyn::zyn! {
        impl {{ ident }} {
            @for (field in fields.iter()) {
                @getter(
                    name = field.ident.clone().unwrap(),
                    ty = field.ty.clone(),
                )
            }
        }
    }
}
```

Users apply it like any derive macro — the function name `my_getters` becomes `MyGetters`:

```rust
#[derive(MyGetters)]
struct User {
    name: String,
    age: u32,
}

// Generated:
// impl User {
//     pub fn get_name(&self) -> &String { &self.name }
//     pub fn get_age(&self) -> &u32 { &self.age }
// }
```

See [Proc Macro Entry Points](./06-macros/index.html) for `#[zyn::attribute]`, custom names, helper attributes, and more.

## Next steps

- [Templates](./02-templates/index.html) — interpolation, control flow, pipes, case conversion
- [Elements](./03-elements/index.html) — children, extractors, input context, advanced patterns
- [Diagnostics](./03-elements/diagnostics.md) — `error!`, `warn!`, `note!`, `help!`, `bail!`
- [derive(Attribute)](./04-attributes/index.html) — typed attribute parsing with `#[derive(Attribute)]`
- [Proc Macro Entry Points](./06-macros/index.html) — `#[zyn::derive]` and `#[zyn::attribute]`
- [Debugging](./07-testing/debugging.md) — `debug` attribute arg with `ZYN_DEBUG` env var
