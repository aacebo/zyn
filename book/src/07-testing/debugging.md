# Debugging

Inspect generated code by adding the `debug` argument to any zyn attribute macro. Debug output is emitted as a compiler `note` diagnostic, visible in both terminal and IDE.

## Setup

Two conditions must be met for debug output:

1. Add `debug` (or `debug(pretty)`) to the macro attribute
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

## Options

Options are passed as a comma-separated list inside parentheses:

| Syntax | Output | Format |
|--------|--------|--------|
| `debug` | Body only, `{{ prop }}` placeholders | Raw |
| `debug(pretty)` | Body only, `{{ prop }}` placeholders | Pretty-printed |
| `debug(full)` | Full struct + impl | Raw |
| `debug(pretty, full)` | Full struct + impl | Pretty-printed |
| `debug(key = "val", ...)` | Body only, matched props substituted | Raw |
| `debug(pretty, key = "val", ...)` | Body only, matched props substituted | Pretty-printed |

Options can be combined in any order: `debug(full, pretty)` is equivalent to `debug(pretty, full)`. Injection pairs can be mixed with `pretty` and `full` freely.

## Static injection

Debug output runs at proc-macro time — runtime values don't exist yet. When a prop like `name` or `ty` appears in a `{{ name }}` interpolation, the debug note shows `{{ name }}` as a placeholder. Static injection lets you supply a representative value so the output shows something meaningful.

### Syntax

```rust
// No injection — placeholders in output
#[zyn::element(debug)]

// Inject a single prop
#[zyn::element(debug(name = "Foo"))]

// Inject multiple props
#[zyn::element(debug(name = "Foo", ty = "String"))]

// Combine with pretty
#[zyn::element(debug(pretty, name = "Foo", ty = "Vec<u8>"))]
```

All injection values are **string literals**. The string content is parsed as a `proc_macro2::TokenStream` — any valid Rust token sequence works: identifiers, types, expressions, literals.

### Example

```rust
#[zyn::element(debug(pretty, name = "Foo", ty = "String"))]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}
```

`ZYN_DEBUG="Setter" cargo build`:

```text
note: zyn::element ─── Setter

      fn Foo(mut self, value: String) -> Self {
          self.Foo = Some(value);
          self
      }
  --> src/lib.rs:1:1
```

Without injection:

```text
note: zyn::element ─── Setter

      fn {{ name }}(mut self, value: {{ ty }}) -> Self {
          self.{{ name }} = Some(value);
          self
      }
  --> src/lib.rs:1:1
```

### Uninjected props

Any prop without a matching injection key renders as `{{ prop_name }}`. This is intentional — it makes unresolved placeholders visually distinct from real tokens.

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

      fn name () { }
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

The `pretty` option uses [`prettyplease`](https://crates.io/crates/prettyplease) to produce properly formatted Rust code with indentation and line breaks.

Enable the `pretty` feature in your `Cargo.toml`:

```toml
[dependencies]
zyn = { version = "0.4", features = ["pretty"] }
```

Then use `debug(pretty)`:

```rust
#[zyn::element(debug(pretty))]
fn greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```bash
ZYN_DEBUG="Greeting" cargo build
```

```text
note: zyn::element ─── Greeting

      fn name() {}
  --> src/lib.rs:1:1
```

> [!NOTE]
> Pretty-printed debug output — formatted with `prettyplease` for readable, indented Rust code.

![Pretty debug output](https://raw.githubusercontent.com/aacebo/zyn/refs/heads/main/assets/screenshots/screenshot-5.png)

If `debug(pretty)` is used without the `pretty` feature enabled, you'll get a helpful compile error:

```text
error: enable the `pretty` feature to use `debug(pretty)`
 --> src/lib.rs:1:16
  |
1 | #[zyn::element(debug(pretty))]
  |                ^^^^^^^^^^^^^^
```

### Full output

Use `debug(full)` to emit the entire generated struct and impl instead of just the body:

```rust
#[zyn::element(debug(full))]
fn greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
```

```text
note: zyn::element ─── Greeting

      struct Greeting { pub name : syn :: Ident , } impl :: zyn :: Render
      for Greeting { fn render (& self , input : & :: zyn :: Input) -> :: zyn ::
      Output { ... } }
  --> src/lib.rs:1:1
```

Combine with `pretty` for formatted full output:

```rust
#[zyn::element(debug(pretty, full))]
fn greeting(name: syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}
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

This keeps the debug output focused on the code you care about.

## Full example

Given this element:

```rust
#[zyn::element(debug(pretty))]
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

      pub fn get_name(&self) -> &ty {
          &self.name
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
