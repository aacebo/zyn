# Introduction

<p align="center">
    <img
        src="https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/banner.svg"
        style="border-radius:4px"
        alt="zyn"
    />
</p>

Zyn is a proc macro framework with templates, composable elements, and built-in diagnostics.

```rust,zyn
zyn! {
    @for (field in fields.iter()) {
        pub fn {{ field.ident | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
            &self.{{ field.ident }}
        }
    }
}
```

> [!NOTE]
> Templates are fully type-checked at compile time — errors appear inline, just like regular Rust code.

![Compile-time type safety](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-1.png)

---

## Why?

### The problem

Building a proc macro in Rust today means pulling in a handful of loosely related crates and gluing them together yourself. Here's what each pain point looks like — and how zyn solves it.

---

#### 1. `quote!` has no control flow

Every conditional or loop forces you out of the template and back into Rust.

<table>
<tr><th>❌ Before</th><th>✅ After</th></tr>
<tr>
<td>

```rust
let vis = if is_pub {
    quote!(pub)
} else {
    quote!()
};

let fields_ts: Vec<_> = fields
    .iter()
    .map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty, }
    })
    .collect();

quote! {
    #vis struct #ident {
        #(#fields_ts)*
    }
}
```

</td>
<td>

```rust
zyn! {
    @if (is_pub) { pub }
    struct {{ ident }} {
        @for (field in fields.iter()) {
            {{ field.ident }}: {{ field.ty }},
        }
    }
}
```

</td>
</tr>
</table>

---

#### 2. Case conversion needs external crates

Renaming an identifier means importing `heck`, calling a conversion function, then wrapping it in `format_ident!`.

<table>
<tr><th>❌ Before</th><th>✅ After</th></tr>
<tr>
<td>

```rust
use heck::ToSnakeCase;

let getter = format_ident!(
    "get_{}",
    name.to_string().to_snake_case()
);
```

</td>
<td>

```rust
{{ name | snake | ident:"get_{}" }}
```

</td>
</tr>
</table>

---

#### 3. Diagnostics are fragmented

Errors, warnings, notes, and help messages each use a different mechanism — or aren't possible at all.

<table>
<tr><th>❌ Before</th><th>✅ After</th></tr>
<tr>
<td>

```rust
// errors
return syn::Error::new_spanned(
    &input,
    "expected a struct",
).to_compile_error().into();

// warnings — needs proc-macro-error crate
emit_warning!(span, "deprecated");

// notes/help — not possible on stable
```

</td>
<td>

```rust
#[zyn::element]
fn validated(
    #[zyn(input)] ident: syn::Ident,
) -> zyn::TokenStream {
    error!("expected a struct";
        span = ident.span());
    note!("only named structs supported");
    help!("change input to a struct");
    bail!();

    warn!("this derive is deprecated");
    zyn::zyn! { /* ... */ }
}
```

</td>
</tr>
</table>

---

#### 4. Attribute parsing is reinvented every time

Every project writes its own parser for `#[my_attr(skip, rename = "foo")]`.

<table>
<tr><th>❌ Before</th><th>✅ After</th></tr>
<tr>
<td>

```rust
for attr in &input.attrs {
    if attr.path().is_ident("my_attr") {
        let args = attr.parse_args_with(
            Punctuated::<Meta, Token![,]>
                ::parse_terminated
        )?;
        for meta in &args {
            match meta {
                Meta::Path(p)
                    if p.is_ident("skip") => {}
                Meta::NameValue(nv)
                    if nv.path.is_ident("rename") => {}
                _ => {}
            }
        }
    }
}
```

</td>
<td>

```rust
use zyn::ext::{AttrExt, AttrsExt};

let args = input.attrs
    .find_args("my_attr")?;

if args.has("skip") { /* ... */ }

if let Some(rename) = args.get("rename") {
    /* ... */
}
```

</td>
</tr>
</table>

---

#### 5. Reusable codegen means manual helper functions

There's no composition model — just functions returning `TokenStream`.

<table>
<tr><th>❌ Before</th><th>✅ After</th></tr>
<tr>
<td>

```rust
fn render_field(
    vis: &Visibility,
    name: &Ident,
    ty: &Type,
) -> TokenStream {
    quote! { #vis #name: #ty, }
}

let tokens: Vec<_> = fields
    .iter()
    .map(|f| render_field(
        &f.vis,
        f.ident.as_ref().unwrap(),
        &f.ty,
    ))
    .collect();

quote! { struct #ident { #(#tokens)* }}
```

</td>
<td>

```rust
#[zyn::element]
fn field_decl(
    vis: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,
) -> zyn::TokenStream {
    zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, }
}

zyn! {
    struct {{ ident }} {
        @for (field in fields.iter()) {
            @field_decl(
                vis = field.vis.clone(),
                name = field.ident.clone().unwrap(),
                ty = field.ty.clone(),
            )
        }
    }
}
```

</td>
</tr>
</table>

---

#### 6. Five crates doing five things

<table>
<tr><th>❌ Before</th><th>✅ After</th></tr>
<tr>
<td>

```toml
[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
heck = "0.5"
proc-macro-error = "1"
```

</td>
<td>

```toml
[dependencies]
zyn = "0.4"
```

</td>
</tr>
</table>

### What zyn does differently

Zyn replaces the entire stack with a single `zyn!` template macro and a set of companion tools:

```rust
zyn! {
    @for (field in fields.iter()) {
        pub fn {{ field.ident | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
            &self.{{ field.ident }}
        }
    }
}
```

| Concern | Zyn approach |
|---|---|
| **Code generation** | `zyn!` template with `{{ }}` interpolation — reads like the code it generates |
| **Control flow** | `@if`, `@for`, `@match` directives inline — no `.iter().map().collect()` |
| **Case conversion** | Built-in pipes: `{{ name \| snake }}`, `{{ name \| pascal }}`, `{{ name \| screaming }}` — no extra crate |
| **Name formatting** | `{{ name \| ident:"get_{}" }}` — one expression, no `let` binding |
| **Diagnostics** | `error!`, `warn!`, `note!`, `help!`, `bail!` macros in [`#[zyn::element]`](./03-elements/diagnostics.md) bodies — one API for all diagnostic levels |
| **Attribute parsing** | `#[derive(Attribute)]` for typed attribute structs — built-in, no `darling` dependency |
| **Reusable codegen** | `#[zyn::element]` — composable template components invoked with `@name(props)` |
| **Value transforms** | `#[zyn::pipe]` — custom pipes that chain with built-ins |
| **Proc macro entry points** | `#[zyn::derive]` and `#[zyn::attribute]` — replace `#[proc_macro_derive]`/`#[proc_macro_attribute]` with auto-parsed `Input` and diagnostics |
| **Debugging** | [`debug` / `debug = "pretty"`](./07-testing/debugging.md) + `ZYN_DEBUG` env var — inspect generated code as compiler diagnostics |
| **String output** | `{{ name \| str }}` — stringify to a `LitStr` without ceremony |

One dependency. No runtime cost. Everything expands at compile time into the same `TokenStream`-building code you'd write by hand — just without the boilerplate.

---

## Features

- **Interpolation** — `{{ expr }}` inserts any `ToTokens` value, with field access and method calls
- **Pipes** — `{{ name | snake }}`, `{{ name | ident:"get_{}" }}`, `{{ name | str }}` — 13 built-in pipes plus custom
- **Control flow** — `@if`, `@for`, `@match` with full nesting
- **Diagnostics** — `error!`, `warn!`, `note!`, `help!`, `bail!` macros in [`#[zyn::element]`](./03-elements/diagnostics.md) bodies
- **Elements** — reusable template components via `#[zyn::element]`
- **Custom pipes** — define transforms with `#[zyn::pipe]`
- **Proc macro entry points** — `#[zyn::derive]` and `#[zyn::attribute]` with auto-parsed input, extractors, and diagnostics
- **Debugging** — [`debug` / `debug = "pretty"`](./07-testing/debugging.md) with `ZYN_DEBUG` env var
- **Attribute parsing** — `#[derive(Attribute)]` for typed attribute structs
- **Case conversion** — `snake`, `camel`, `pascal`, `screaming`, `kebab`, `upper`, `lower`, `trim`, `plural`, `singular`
