# zyn — a template engine for Rust proc macros

I kept rebuilding the same proc macro scaffolding across my own crates — `syn` for parsing, `quote` for codegen, `heck` for case conversion, `proc-macro-error` for diagnostics, hand-rolled attribute parsing, and a pile of helper functions returning `TokenStream`. Every project was the same patchwork. zyn started as a way to stop repeating myself, and turned into a framework that replaces all of it with a single crate.

<a href="https://aacebo.github.io/zyn" target="_blank">
    <img src="https://img.shields.io/badge/📖 Getting Started-blue?style=for-the-badge" />
</a>

`cargo add zyn`

## What it looks like

### Templates with control flow

With `quote!`, every conditional or loop forces you out of the template:

```rust
let fields_ts: Vec<_> = fields
    .iter()
    .map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty, }
    })
    .collect();

quote! {
    struct #ident {
        #(#fields_ts)*
    }
}
```

With zyn:

```rust
zyn! {
    struct {{ ident }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
}
// generates: struct User { name: String, age: u32, }
```

`@if`, `@for`, and `@match` all work inline. No `.iter().map().collect()`.

### Case conversion and formatting

Before:

```rust
use heck::ToSnakeCase;

let getter = format_ident!(
    "get_{}",
    name.to_string().to_snake_case()
);
```

After:

```rust
{{ name | snake | ident:"get_{}" }}
// HelloWorld -> get_hello_world
```

13 built-in pipes: `snake`, `camel`, `pascal`, `screaming`, `kebab`, `upper`, `lower`, `str`, `trim`, `plural`, `singular`, `ident`, `fmt`. They chain.

### Reusable components

`#[zyn::element]` turns a template into a callable component:

```rust
#[zyn::element]
fn getter(name: syn::Ident, ty: syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        pub fn {{ name | snake | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    }
}

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
// generates:
// impl User {
//     pub fn get_name(&self) -> &String { &self.name }
//     pub fn get_age(&self) -> &u32 { &self.age }
// }
```

Elements accept typed parameters, can receive children blocks, and compose with each other.

### Proc macro entry points

`#[zyn::derive]` and `#[zyn::attribute]` replace the raw `#[proc_macro_derive]` / `#[proc_macro_attribute]` annotations. Input is auto-parsed and extractors pull what you need:

```rust
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

Users write `#[derive(MyGetters)]` — the function name auto-converts to PascalCase:

```rust
#[derive(MyGetters)]
struct User {
    name: String,
    age: u32,
}

// generates:
// impl User {
//     pub fn get_name(&self) -> &String { &self.name }
//     pub fn get_age(&self) -> &u32 { &self.age }
// }
```

### Diagnostics

`error!`, `warn!`, `note!`, `help!`, and `bail!` work inside `#[zyn::element]`, `#[zyn::derive]`, and `#[zyn::attribute]` bodies:

```rust
#[zyn::derive]
fn my_derive(
    #[zyn(input)] fields: zyn::Fields,
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
) -> zyn::TokenStream {
    if fields.is_empty() {
        bail!("at least one field is required");
    }

    zyn::zyn!(impl {{ ident }} {})
}
```

The compiler output:

```
error: at least one field is required
 --> src/main.rs:3:10
  |
3 | #[derive(MyDerive)]
  |          ^^^^^^^^
```

No `syn::Error` ceremony, no external crate for warnings.

### Typed attribute parsing

`#[derive(Attribute)]` generates a typed struct from helper attributes:

```rust
#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default)]
    skip: bool,
    #[zyn(default = "build".to_string())]
    method: String,
}

#[zyn::derive("Builder", attributes(builder))]
fn builder(
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
    #[zyn(input)] fields: zyn::Fields,
    #[zyn(input)] cfg: zyn::Attr<BuilderConfig>,
) -> zyn::TokenStream {
    if cfg.skip {
        return zyn::zyn!();
    }

    let method = zyn::format_ident!("{}", cfg.method);
    zyn::zyn! {
        impl {{ ident }} {
            pub fn {{ method }}(self) -> Self { self }
        }
    }
}
```

`zyn::Attr<BuilderConfig>` auto-resolves from the input context — fields are parsed and defaulted automatically. Users write `#[builder(skip)]` or `#[builder(method = "create")]` on their structs.

## Full feature list

- `zyn!` template macro with `{{ }}` interpolation
- `@if` / `@for` / `@match` control flow
- 13 built-in pipes + custom pipes via `#[zyn::pipe]`
- `#[zyn::element]` — reusable template components with typed params and children
- `#[zyn::derive]` / `#[zyn::attribute]` — proc macro entry points with auto-parsed input
- Extractor system: `Extract<T>`, `Attr<T>`, `Fields`, `Variants`, `Data<T>`
- `error!`, `warn!`, `note!`, `help!`, `bail!` diagnostics
- `#[derive(Attribute)]` for typed attribute parsing
- `zyn::debug!` — drop-in `zyn!` replacement that prints expansions (`pretty`, `raw`, `ast` modes)
- Case conversion functions available outside templates (`zyn::case::to_snake()`, etc.)
- Re-exports `syn`, `quote`, and `proc-macro2` — one dependency in your `Cargo.toml`

## Benchmarks

zyn is benchmarked against `darling` for attribute parsing, vanilla `syn + quote` for
macro expansion, and `heck` for case conversion. Attribute extraction is faster than
darling and faster than the raw `syn::parse2` baseline. The full derive pipeline adds
~660 ns of overhead over vanilla on a 5-field struct — entirely at compile time.

Full results, methodology, and charts: [`benches/RESULTS.md`](benches/RESULTS.md)

## Links

- GitHub: https://github.com/aacebo/zyn
- Docs / Book: https://aacebo.github.io/zyn
- crates.io: https://crates.io/crates/zyn

This is v0.3.1. I'd appreciate any feedback — on the API design, the template syntax, the docs, or anything else. Happy to answer questions.

## License

MIT
