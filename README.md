<img src="https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/banner.svg" style="border-radius:10px">

[![Getting Started](https://img.shields.io/badge/📖%20Getting%20Started-blue)](https://aacebo.github.io/zyn)
[![docs.rs](https://img.shields.io/docsrs/zyn)](https://docs.rs/zyn)
[![crates.io](https://img.shields.io/crates/v/zyn)](https://crates.io/crates/zyn)
[![bencher.dev](https://img.shields.io/badge/bencher.dev-passing-brightgreen)](https://bencher.dev/perf/zyn/plots)
[![Crates.io Size](https://img.shields.io/crates/size/zyn)](https://crates.io/crates/zyn)

A proc macro framework with templates, composable elements, and built-in diagnostics.

[🗺️ Roadmap](./ROADMAP.md)

```sh
cargo add zyn
```

## Table of Contents

- [Templates](#templates)
- [Elements](#elements)
- [Pipes](#pipes)
- [Attributes](#attributes)
  - [Auto-suggest](#auto-suggest)
- [Testing](#testing)
  - [Assertions](#assertions)
  - [Debugging](#debugging)
- [Features](#features)
  - [ext](#ext)
  - [pretty](#pretty)
  - [diagnostics](#diagnostics)
- [Performance](./BENCH.md)

---

## Templates

> Templates are fully type-checked at compile time — errors appear inline, just like regular Rust code.

![Compile-time type safety](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-1.png)

The `zyn!` macro is the core of zyn. Write token output as if it were source code,
with `{{ }}` interpolation and `@` control flow directives.

**Interpolation** — any `ToTokens` value:

```rust
let name = zyn::format_ident!("hello_world");
zyn::zyn!(fn {{ name }}() {})
// → fn hello_world() {}
```

**Pipes** — transform values inline:

```rust
zyn::zyn!(fn {{ name | pascal }}() {})
// name = "hello_world" → fn HelloWorld() {}
```

**Control flow:**

```rust
zyn::zyn!(
    @if (is_pub) { pub }
    @for (field in fields.named.iter()) {
        fn {{ field.ident }}(&self) -> &{{ field.ty }} {
            &self.{{ field.ident }}
        }
    }
)
```

**Full template syntax:**

| Syntax | Purpose |
|--------|---------|
| `{{ expr }}` | Interpolate any `ToTokens` value |
| `{{ expr \| pipe }}` | Transform value through a [pipe](#pipes) before inserting |
| `@if (cond) { ... }` | Conditional token emission |
| `@else { ... }` | Else branch |
| `@else if (cond) { ... }` | Else-if branch |
| `@for (x in iter) { ... }` | Loop over an iterator |
| `@for (N) { ... }` | Repeat N times |
| `@match (expr) { pat => { ... } }` | Pattern-based emission |
| `@element_name(prop = val)` | Invoke a `#[element]` component |

---

## Elements

Elements are reusable template components defined with `#[zyn::element]`.
They encapsulate a fragment of token output and accept typed props.

**Define an element:**

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

**Invoke it inside any template with `@`:**

```rust
zyn::zyn! {
    impl {{ ident }} {
        @for (field in fields.named.iter()) {
            @getter(name = field.ident.clone().unwrap(), ty = field.ty.clone())
        }
    }
}
```

Elements can also receive **extractors** — values resolved automatically from proc macro
input — by marking a param with `#[zyn(input)]`:

```rust
#[zyn::derive]
fn my_getters(
    #[zyn(input)] ident: zyn::Extract<zyn::syn::Ident>,
    #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
) -> zyn::TokenStream {
    zyn::zyn! {
        impl {{ ident }} {
            @for (field in fields.named.iter()) {
                pub fn {{ field.ident | snake | ident:"get_{}" }}(&self) -> &{{ field.ty }} {
                    &self.{{ field.ident }}
                }
            }
        }
    }
}
// Applied to: struct User { first_name: String, age: u32 }
// Generates:
// impl User {
//     pub fn get_first_name(&self) -> &String { &self.first_name }
//     pub fn get_age(&self) -> &u32 { &self.age }
// }
```

---

## Pipes

Pipes transform interpolated values: `{{ expr | pipe }}`. They chain left to right:

```rust
zyn::zyn!(fn {{ name | snake | ident:"get_{}" }}() {})
// name = "HelloWorld" → fn get_hello_world() {}
```

**Built-in pipes:**

| Pipe | Input example | Output |
|------|--------------|--------|
| `snake` | `HelloWorld` | `hello_world` |
| `pascal` | `hello_world` | `HelloWorld` |
| `camel` | `hello_world` | `helloWorld` |
| `screaming` | `HelloWorld` | `HELLO_WORLD` |
| `kebab` | `HelloWorld` | `"hello-world"` |
| `upper` | `hello` | `HELLO` |
| `lower` | `HELLO` | `hello` |
| `str` | `hello` | `"hello"` |
| `trim` | `__foo__` | `foo` |
| `plural` | `user` | `users` |
| `singular` | `users` | `user` |
| `ident:"pattern_{}"` | `hello` | `pattern_hello` (ident) |
| `fmt:"pattern_{}"` | `hello` | `"pattern_hello"` (string) |

**Custom pipes** via `#[zyn::pipe]`:

```rust
#[zyn::pipe]
fn shout(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(&format!("{}_BANG", input.to_uppercase()), zyn::Span::call_site())
}

zyn::zyn!(fn {{ name | shout }}() {})
// name = "hello" → fn HELLO_BANG() {}
```

---

## Attributes

zyn provides two tools for attribute handling: a derive macro for typed parsing and a
proc macro attribute for writing attribute macros.

**Typed attribute structs** via `#[derive(Attribute)]`:

```rust
#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default)]
    skip: bool,
    #[zyn(default = "build".to_string())]
    method: String,
}
// users write: #[builder(skip)] or #[builder(method = "create")]
```

The derive generates `from_args`, `FromArg`, and `FromInput` implementations, as well as
a human-readable `about()` string for error messages.

### Auto-suggest

When a user misspells an argument name, zyn automatically suggests the closest known
field. No extra setup required:

```text
error: unknown argument `skiip`
  --> src/main.rs:5:12
   |
5  | #[builder(skiip)]
   |           ^^^^^
   |
   = help: did you mean `skip`?
```

Suggestions are offered when the edit distance is ≤ 3 characters. Distant or completely
unknown keys produce only the "unknown argument" error without a suggestion.

**Attribute proc macros** via `#[zyn::attribute]`:

```rust
#[zyn::attribute]
fn my_attr(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> zyn::TokenStream {
    // args: parsed key=value arguments from the attribute invocation
    zyn::zyn!({ { item } })
}
```

---

## Testing

### Assertions

`zyn!` returns [`Output`](https://docs.rs/zyn/latest/zyn/struct.Output.html) — test both tokens and diagnostics directly:

```rust
#[test]
fn generates_function() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput).unwrap().into();
    let output = zyn::zyn!(fn hello() {});
    let expected = zyn::quote::quote!(fn hello() {});
    zyn::assert_tokens!(output, expected);
}
```

Diagnostic assertions check error messages from `error!`, `warn!`, `bail!`:

```rust
#[zyn::element]
fn validated(name: zyn::syn::Ident) -> zyn::TokenStream {
    if name == "forbidden" {
        bail!("reserved identifier `{}`", name);
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn rejects_forbidden_name() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput).unwrap().into();
    let output = zyn::zyn!(@validated(name = zyn::format_ident!("forbidden")));
    zyn::assert_diagnostic_error!(output, "reserved identifier");
    zyn::assert_tokens_empty!(output);
    // ✓ error diagnostic present, no tokens produced
}

#[test]
fn accepts_valid_name() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput).unwrap().into();
    let output = zyn::zyn!(@validated(name = zyn::format_ident!("hello")));
    zyn::assert_tokens_contain!(output, "fn hello");
    // ✓ tokens contain "fn hello"
}
```

| Macro | Purpose |
|-------|---------|
| `assert_tokens!` | Compare two token streams |
| `assert_tokens_empty!` | Assert no tokens produced |
| `assert_tokens_contain!` | Check for substring in output |
| `assert_diagnostic_error!` | Assert error diagnostic with message |
| `assert_diagnostic_warning!` | Assert warning diagnostic |
| `assert_diagnostic_note!` | Assert note diagnostic |
| `assert_diagnostic_help!` | Assert help diagnostic |
| `assert_compile_error!` | Alias for `assert_diagnostic_error!` |

With the `pretty` feature:

| Macro | Purpose |
|-------|---------|
| `assert_tokens_pretty!` | Compare using `prettyplease`-formatted output |
| `assert_tokens_contain_pretty!` | Substring check on pretty-printed output |

### Debugging

Inspect generated code by adding `debug` to any zyn attribute macro. Set `ZYN_DEBUG` to the generated type name (or `*` for all) to enable output:

```rust
#[zyn::element(debug)]
fn greeting(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```sh
ZYN_DEBUG="*" cargo build
```

```text
note: zyn::element ─── Greeting

      fn {{ name }}() {}
  --> src/lib.rs:1:1
```

Without injection, props show as `{{ name }}` placeholders. With the `pretty` feature, use `debug(pretty)` for formatted output:

```rust
#[zyn::element(debug(pretty))]
fn greeting(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```text
note: zyn::element ─── Greeting

      fn {{ name }}() {}
  --> src/lib.rs:1:1
```

Use `debug(full)` to emit the full generated struct + impl instead of just the body. Combine with `pretty`: `debug(pretty, full)`.

Supply `key = "value"` injection pairs to substitute static values for props — useful when the real value isn't known at proc-macro time:

```rust
#[zyn::element(debug(name = "Foo", ty = "String"))]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}(value: {{ ty }}) -> Self { self })
}
```

Output: `fn Foo(value : String) -> Self { self }`. Without injection, props show as `{{ name }}`, `{{ ty }}`.

![Static injection — inline diagnostic](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-10.png)

All macros support `debug`: `#[zyn::element]`, `#[zyn::pipe]`, `#[zyn::derive]`, `#[zyn::attribute]`.

`ZYN_DEBUG` accepts comma-separated `*`-wildcard patterns matched against the generated PascalCase type name:

```sh
ZYN_DEBUG="Greeting" cargo build        # exact match
ZYN_DEBUG="Greet*" cargo build          # prefix wildcard
ZYN_DEBUG="*Element" cargo build        # suffix wildcard
ZYN_DEBUG="Greeting,Shout" cargo build  # multiple patterns
```

---

## Features

| Feature | Default | Description |
|---------|:-------:|-------------|
| `derive` | ✓ | All proc macro attributes: `#[element]`, `#[pipe]`, `#[derive]`, `#[attribute]`, and `#[derive(Attribute)]` |
| `ext` | | Extension traits for common `syn` AST types (`AttrExt`, `FieldExt`, `TypeExt`, etc.) |
| `pretty` | | Pretty-printed token output in debug mode |
| `diagnostics` | | Error accumulation — collect multiple errors before aborting |

### ext

The `ext` module adds ergonomic extension traits for navigating `syn` AST types.

```toml
zyn = { features = ["ext"] }
```

```rust
use zyn::ext::{AttrExt, TypeExt};

// check and read attribute arguments
if attr.is("serde") {
    let rename: Option<_> = attr.get("rename"); // → Some(Meta::NameValue)
    let skip: bool = attr.exists("skip");
}

// inspect field types
if field.is_option() {
    let inner = field.inner_type().unwrap();
}
```

### pretty

Pretty-printed debug output and `assert_tokens_pretty!` / `assert_tokens_contain_pretty!` assertion macros via `prettyplease`. See [Debugging](#debugging) for usage.

```toml
zyn = { features = ["pretty"] }
```

![Pretty debug output](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-5.png)

![Pretty full output](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-9.png)

### diagnostics

The `diagnostics` feature enables error accumulation — collecting multiple compiler
errors before aborting, so users see all problems at once instead of one at a time.

```toml
zyn = { features = ["diagnostics"] }
```

Inside any `#[zyn::element]`, `#[zyn::derive]`, or `#[zyn::attribute]` body, use the
built-in diagnostic macros directly — no setup required:

```rust
#[zyn::element]
fn my_element(name: zyn::syn::Ident) -> zyn::TokenStream {
    if name == "forbidden" {
        bail!("reserved identifier `{}`", name);
    }

    if name.to_string().starts_with('_') {
        warn!("identifiers starting with `_` are conventionally unused");
    }

    zyn::zyn!(fn {{ name }}() {})
}
```

| Macro | Level | Behaviour |
|-------|-------|-----------|
| `error!(msg)` | error | accumulates, does not stop execution |
| `warn!(msg)` | warning | accumulates, does not stop execution |
| `note!(msg)` | note | accumulates, does not stop execution |
| `help!(msg)` | help | accumulates, does not stop execution |
| `bail!(msg)` | error | accumulates and immediately returns |

All accumulated diagnostics are emitted together at the end of the element or macro body,
so users see every error at once instead of fixing them one by one.

```text
error: reserved identifier `forbidden`
 --> src/main.rs:3:1

error: reserved identifier `forbidden`
 --> src/main.rs:7:1
```

---

## Performance

Benchmarks confirm the zero-overhead claim: the full pipeline (parse, extract, codegen) matches vanilla `syn` + `quote` for both structs and enums. Where zyn replaces external crates, it's faster — case conversion is ~6x faster than `heck`, and attribute parsing is ~14% faster than `darling`.

[Live benchmark charts on bencher.dev](./BENCH.md)

## Discussions

- [Diagnostics API](https://github.com/aacebo/zyn/discussions/7)
- [Extractor API](https://github.com/aacebo/zyn/discussions/8)

## License

MIT
