# Debugging

Inspect generated code by adding the `debug` argument to any zyn attribute macro. Debug output is emitted as a compiler `note` diagnostic, visible in both terminal and IDE.

## Setup

Two conditions must be met for debug output:

1. Add `debug` (or `debug = "pretty"`) to the macro attribute
2. Set the `ZYN_DEBUG` environment variable to match the generated type name

```rust
#[zyn::element(debug)]
fn greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```bash
ZYN_DEBUG="*" cargo build
```

Without `ZYN_DEBUG` set, the `debug` argument is inert — no output, no overhead. This makes it safe to leave in source code during development.

> [!NOTE]
> An element annotated with `debug` — the argument is inert until `ZYN_DEBUG` is set.

![Element with debug arg](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-2.png)

## Supported macros

| Macro | Syntax |
|-------|--------|
| `#[zyn::element]` | `#[zyn::element(debug)]`, `#[zyn::element("name", debug)]` |
| `#[zyn::pipe]` | `#[zyn::pipe(debug)]`, `#[zyn::pipe("name", debug)]` |
| `#[zyn::derive]` | `#[zyn::derive("Name", debug)]`, `#[zyn::derive("Name", attributes(skip), debug)]` |
| `#[zyn::attribute]` | `#[zyn::attribute(debug)]` |

All macros also accept `debug = "pretty"` in place of `debug`:

| Macro | Pretty syntax |
|-------|---------------|
| `#[zyn::element]` | `#[zyn::element(debug = "pretty")]`, `#[zyn::element("name", debug = "pretty")]` |
| `#[zyn::pipe]` | `#[zyn::pipe(debug = "pretty")]`, `#[zyn::pipe("name", debug = "pretty")]` |
| `#[zyn::derive]` | `#[zyn::derive("Name", debug = "pretty")]` |
| `#[zyn::attribute]` | `#[zyn::attribute(debug = "pretty")]` |

## Output formats

### Raw (default)

The default format emits the raw `TokenStream::to_string()` output. This is a flat, single-line string with fully-qualified paths and spaces between all tokens. No extra dependencies are required.

```rust
#[zyn::element(debug)]
fn greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```bash
ZYN_DEBUG="Greeting" cargo build
```

```text
note: zyn::element ─── Greeting

      struct Greeting { pub name : zyn :: syn :: Ident , } impl :: zyn :: Render
      for Greeting { fn render (& self , input : & :: zyn :: Input) -> :: zyn ::
      proc_macro2 :: TokenStream { ... } }
  --> src/lib.rs:1:1
```

The raw format is useful for quick checks and when you want to see the exact tokens being generated.

> [!NOTE]
> Raw debug output shown as an inline compiler diagnostic in the editor.

![Raw debug output — inline diagnostic](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-3.png)

> [!NOTE]
> The same raw output surfaced in the Problems panel for easy navigation.

![Raw debug output — Problems panel](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-4.png)

> [!NOTE]
> Pretty-printed debug output in the console for a pipe.

![Pretty debug console output](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-6.png)

### Pretty (feature-gated)

The `pretty` format uses [`prettyplease`](https://crates.io/crates/prettyplease) to produce properly formatted Rust code with indentation and line breaks.

Enable the `pretty` feature in your `Cargo.toml`:

```toml
[dependencies]
zyn = { version = "0.4", features = ["pretty"] }
```

Then use `debug = "pretty"`:

```rust
#[zyn::element(debug = "pretty")]
fn greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```bash
ZYN_DEBUG="Greeting" cargo build
```

```text
note: zyn::element ─── Greeting

      struct Greeting {
          pub name: zyn::syn::Ident,
      }
      impl ::zyn::Render for Greeting {
          fn render(&self, input: &::zyn::Input) -> ::zyn::Output {
              let mut diagnostics = ::zyn::mark::new();
              let name = &self.name;
              let __body = { zyn::zyn!(fn {{ name }}() {}) };
              ::zyn::Output::new()
                  .tokens(__body)
                  .diagnostic(diagnostics)
                  .build()
          }
      }
  --> src/lib.rs:1:1
```

> [!NOTE]
> Pretty-printed debug output — formatted with `prettyplease` for readable, indented Rust code.

![Pretty debug output](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-5.png)

If `debug = "pretty"` is used without the `pretty` feature enabled, you'll get a helpful compile error:

```text
error: enable the `pretty` feature to use `debug = "pretty"`
 --> src/lib.rs:1:24
  |
1 | #[zyn::element(debug = "pretty")]
  |                        ^^^^^^^^
```

## ZYN_DEBUG environment variable

The `ZYN_DEBUG` environment variable controls which items produce debug output. It accepts comma-separated patterns with `*` wildcards, matched against the **generated type name** (the PascalCase struct name, not the function name).

For an element defined as `fn greeting(...)`, the generated type is `Greeting`. For a pipe `fn shout(...)`, the type is `Shout`.

| Pattern | Matches |
|---------|---------|
| `*` | Everything |
| `Greeting` | Exact match only |
| `Greet*` | `Greeting`, `GreetingElement`, etc. |
| `*Element` | `FieldElement`, `GreetingElement`, etc. |
| `Greeting,Shout` | `Greeting` and `Shout` |
| `Greet*,Shout,*Pipe` | Mix wildcards and exact |

```bash
ZYN_DEBUG="*" cargo build
ZYN_DEBUG="Greeting" cargo build
ZYN_DEBUG="Greet*" cargo build
ZYN_DEBUG="Greeting,Shout" cargo build
```

## Noise stripping

Before formatting (in both raw and pretty modes), zyn strips internal boilerplate from the generated code:

- **`#[doc = "..."]` attributes** — removes the doc comment blocks on generated diagnostic macros
- **`#[allow(...)]` attributes** — removes `#[allow(unused)]` and similar
- **`macro_rules!` definitions** — removes the internal `error!`, `warn!`, `note!`, `help!`, and `bail!` macro definitions

This keeps the debug output focused on the code you care about: the generated struct and its `Render` / `Pipe` implementation.

## Full example

Given this element:

```rust
#[zyn::element(debug = "pretty")]
fn field_getter(
    name: syn::Ident,
    ty: syn::Type,
) -> zyn::TokenStream {
    zyn::zyn!(
        pub fn {{ name | ident:"get_{}" }}(&self) -> &{{ ty }} {
            &self.{{ name }}
        }
    )
}
```

Running with `ZYN_DEBUG="FieldGetter" cargo build` produces:

```text
note: zyn::element ─── FieldGetter

      struct FieldGetter {
          pub name: syn::Ident,
          pub ty: syn::Type,
      }
      impl ::zyn::Render for FieldGetter {
          fn render(&self, input: &::zyn::Input) -> ::zyn::Output {
              let mut diagnostics = ::zyn::mark::new();
              let name = &self.name;
              let ty = &self.ty;
              let __body = {
                  zyn::zyn!(
                      pub fn {{ name | ident:"get_{}" }}(&self) -> &{{ ty }} {
                          &self.{{ name }}
                      }
                  )
              };
              ::zyn::Output::new()
                  .tokens(__body)
                  .diagnostic(diagnostics)
                  .build()
          }
      }
```

## Pipeline API

For library authors building on top of zyn, the debug module exposes a pipeline API via the `DebugExt` trait:

```rust
use zyn::debug::DebugExt;

// Raw format — always available
let raw: String = tokens.debug().raw();

// Pretty format — requires `pretty` feature
#[cfg(feature = "pretty")]
let pretty: String = tokens.debug().pretty();
```
