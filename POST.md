---
title: I Got Tired of Fighting quote! — So I Built a Template Engine for Proc Macros
published: false
tags: rust, macros, opensource, showdev
---

I've been writing proc macros for a while now. Derive macros for internal tools, attribute macros for instrumentation. And every time, the same two problems: `quote!` doesn't compose (you end up passing `TokenStream` fragments through five layers of helper functions), and debugging generated code means `cargo expand` and then squinting at unformatted token output hoping something jumps out.

Because of this I ended up writing the same helper methods, composite AST parsing and tokenizing types, extractors etc. I would have to copy these from project to project as needed, and eventually just decided to publish a crate so I never have to do it again.

So I built [zyn](https://github.com/aacebo/zyn) — a proc macro framework with a template language, composable components, and compile-time diagnostics.

## Goals

1. Template syntax that supports expressions, looping, and composition of reusable custom elements.
2. Automated attribute arguments parsing.
3. Diagnostic pattern that supports more than just hard compiler errors and can emit more than one at a time, linked to the span it originated from. Ideally with editor integration.
4. Extensions for `syn` AST types to make querying the parsed AST easier.
5. Testing features like `debug` and assertion macros so I don't have to use `cargo expand` or stringify token streams and make fuzzy assertions.

## Example: [Builder](https://github.com/aacebo/zyn/tree/main/examples/builder)

```sh
cargo add zyn
```

I'm going to build a `#[derive(Builder)]` macro with it, start to finish. The whole thing comes out to about 60 lines.

## The Template Language

The core is the `zyn!` macro. You write output that looks like the code you're generating:

```rust
let name = zyn::format_ident!("hello_world");
zyn::zyn!(fn {{ name }}() {})
// → fn hello_world() {}
```

Double braces for interpolation. Pipes are where it gets interesting though — inline transforms that chain:

```rust
zyn::zyn!(fn {{ name | pascal }}() {})
// name = "hello_world" → fn HelloWorld() {}

zyn::zyn!(fn {{ name | snake | ident:"get_{}" }}() {})
// name = "HelloWorld" → fn get_hello_world() {}
```

13 built-in pipes — `snake`, `pascal`, `camel`, `screaming`, `kebab`, `upper`, `lower`, `str`, `plural`, `singular`, `trim`, and format patterns `ident:"prefix_{}"` and `fmt:"prefix_{}"`. All hand-rolled, no `heck` dependency, benchmarks at about 6x faster. You can write your own too.

Control flow uses `@` directives:

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

`@if`, `@for`, `@match`. The template is fully type-checked at compile time — if you reference a variable that doesn't exist, you get a normal Rust compiler error.

## Building a Builder

What we want the user to write:

```rust
#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    #[builder(default)]
    verbose: bool,
    #[builder(default_value = "30")]
    timeout: i64,
}
```

And what we want to generate:

```rust
struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    verbose: Option<bool>,
    timeout: Option<i64>,
}

impl ConfigBuilder {
    fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }
    // ... setters for each field ...

    fn build(self) -> Config {
        Config {
            host: self.host.expect("field `host` is required"),
            port: self.port.expect("field `port` is required"),
            verbose: self.verbose.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_else(|| 30),
        }
    }
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder {
            host: None,
            port: None,
            verbose: None,
            timeout: None,
        }
    }
}
```

With raw `quote!`, this gets messy fast — nested iterations, conditional logic for defaults, splicing field names and types everywhere.

## Typed Attribute Parsing

First, parsing `#[builder(default)]` and `#[builder(default = expr)]`. Doing this by hand means a `syn::parse::Parse` impl, handling every variant, producing decent errors. With zyn:

```rust
#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default)]
    skip: bool,
    #[zyn(default)]
    default: bool,
    default_value: Option<String>,
}
```

That generates `from_args()` and `from_input()` methods. We add a convenience `from_field` that extracts from a field's attributes using the `ext` feature:

```rust
use zyn::ext::AttrExt;

impl BuilderConfig {
    fn from_field(field: &zyn::syn::Field) -> Self {
        let attr = field.attrs.iter().find(|a| a.is("builder"));

        match attr {
            Some(attr) => {
                let args = attr.args().unwrap();
                Self::from_args(&args).unwrap()
            }
            None => Self {
                skip: false,
                default: false,
                default_value: None,
            },
        }
    }
}
```

Typo suggestions come free:

```
error: unknown argument `skiip`
  |
5 | #[builder(skiip)]
  |           ^^^^^
  |
  = help: did you mean `skip`?
```

Levenshtein distance. Your users get `did you mean skip?` instead of `unexpected token`.

## Composable Elements

Instead of one giant `quote!` block, you break the macro into **elements** — reusable template components with typed props.

```rust
#[zyn::element]
fn setter(
    name: zyn::syn::Ident,
    ty: zyn::syn::Type,
) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}
```

If you wanted methods like `with_host` instead of `host`, pipes handle it inline: `{{ name | ident:"with_{}" }}`. They compose — `{{ name | upper | ident:"SET_{}" }}` would produce `SET_HOST` from `host`.

The build method, where defaults come in:

```rust
#[zyn::element]
fn build_field(
    name: zyn::syn::Ident,
    config: BuilderConfig,
) -> zyn::TokenStream {
    let name_str = name.to_string();

    if config.default {
        zyn::zyn!({{ name }}: self.{{ name }}.unwrap_or_default())
    } else if let Some(ref expr) = config.default_value {
        let default_expr: zyn::syn::Expr = zyn::syn::parse_str(expr).unwrap();
        zyn::zyn!({{ name }}: self.{{ name }}.unwrap_or_else(|| {{ default_expr }}))
    } else {
        zyn::zyn!({{ name }}: self.{{ name }}.expect(
            ::std::concat!("field `", {{ name_str | str }}, "` is required")
        ))
    }
}
```

The setter doesn't care about defaults — that's `build_field`'s job.

Custom pipes work the same way. Say we want one that wraps any type in `Option`:

```rust
#[zyn::pipe]
fn optional(input: String) -> zyn::syn::Type {
    zyn::syn::parse_str(&format!("Option<{}>", input)).unwrap()
}

// {{ ty | optional }} → Option<String>
```

Takes a string, returns a type. Works in any `{{ value | pipe }}` expression.

## The Derive Entry Point

The derive uses **extractors** — typed parameters that zyn resolves from the macro input automatically:

```rust
#[zyn::derive("Builder", attributes(builder))]
fn builder(
    #[zyn(input)] ident: zyn::syn::Ident,
    #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
) -> zyn::TokenStream {

    zyn::zyn! {
        struct {{ ident | ident:"{}Builder" }} {
            @for (field in fields.named.iter()) {
                {{ field.ident }}: Option<{{ field.ty }}>,
            }
        }

        impl {{ ident | ident:"{}Builder" }} {
            @for (field in fields.named.iter()) {
                @setter(
                    name = field.ident.clone().unwrap(),
                    ty = field.ty.clone(),
                )
            }

            fn build(self) -> {{ ident }} {
                {{ ident }} {
                    @for (field in fields.named.iter()) {
                        @build_field(
                            name = field.ident.clone().unwrap(),
                            config = BuilderConfig::from_field(field),
                        ),
                    }
                }
            }
        }

        impl {{ ident }} {
            fn builder() -> {{ ident | ident:"{}Builder" }} {
                {{ ident | ident:"{}Builder" }} {
                    @for (field in fields.named.iter()) {
                        {{ field.ident }}: None,
                    }
                }
            }
        }
    }
}
```

Parameters marked `#[zyn(input)]` are extractors — `ident` gets resolved from the derive input automatically, `Fields<FieldsNamed>` pulls the named fields. If someone puts `#[derive(Builder)]` on an enum, zyn emits a compile error automatically.

The `@for` loops iterate fields. The `@setter` and `@build_field` calls compose the pieces. The template reads top-to-bottom as one block, no splicing iterator chains back together like you would with `quote!`.

## Diagnostics

Standard proc macros bail on the first error. Fix, recompile, hit the next one.

zyn accumulates them. Add some validation to the builder:

```rust
for field in fields.named.iter() {
    let config = BuilderConfig::from_field(field);

    if config.skip && config.default {
        error!(
            "`skip` and `default` are mutually exclusive on field `{}`",
            field.ident.as_ref().unwrap();
            span = field.ident.as_ref().unwrap().span()
        );
    }

    if config.skip && config.default_value.is_some() {
        warn!(
            "`default_value` is ignored when `skip` is set";
            span = field.ident.as_ref().unwrap().span()
        );
    }
}

// stop here if any errors accumulated, otherwise continue to codegen
bail!();
```

`error!`, `warn!`, `note!`, `help!` are injected into every `#[zyn::derive]`, `#[zyn::element]`, and `#[zyn::attribute]` body. `bail!()` with no arguments checks if any errors were accumulated and returns early — but only if there are errors. Warnings pass through.

Users see every problem in one compile pass.

## Debugging

I wrote the debug system after spending two days on a bug where a generated impl block was missing a lifetime bound. `cargo expand` spat out 400 lines of tokens and I couldn't find it, so I built a debug system.

Add `debug = "pretty"` to any element, derive, or attribute macro:

```rust
#[zyn::element(debug = "pretty")]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    // ...
}
```

```sh
ZYN_DEBUG="Setter" cargo build
```

Generated code shows up as a compiler note — in your terminal, in your IDE's Problems panel. `pretty` runs it through `prettyplease` so you get formatted Rust instead of token soup. Wildcard patterns work: `ZYN_DEBUG="*"` dumps everything.

## Testing

zyn's test module gives you assertion macros that compare token streams structurally. Here's how we test the `setter` element from the builder:

```rust
use zyn::quote::quote;

#[zyn::element(debug = "pretty")]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}

#[test]
fn setter_generates_expected_signature() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("port"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("u16").unwrap(),
        )
    );
    let expected = quote! {
        fn port(mut self, value: u16) -> Self {
            self.port = Some(value);
            self
        }
    };

    zyn::assert_tokens!(output, expected);
}

#[test]
fn setter_pretty_output() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("host"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("String").unwrap(),
        )
    );

    zyn::assert_tokens_contain_pretty!(output, "fn host(mut self, value: String) -> Self");
}
```

`assert_tokens!` compares structurally — no `to_string()` comparisons that break on whitespace. `assert_tokens_contain!` does substring matching on the cleaned output. `assert_tokens_contain_pretty!` (behind the `pretty` feature) gives you human-readable diffs when things fail.

## Performance

Benchmarks are run via CI on push and also on a schedule.

> The full pipeline (parse → extract → codegen) compared to equivalent hand-written `syn` + `quote!`:

<a href="https://bencher.dev/perf/zyn?lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=false&x_axis=date_time&branches=d618e093-bbbc-439f-82af-4502c72cd2bd&testbeds=dbe8a0e5-b945-4f98-9cd3-303f96426cd4&benchmarks=19886ff1-468f-4126-a1a8-01b680c66df3,bc8919bf-3786-4119-8232-c86165c96c50&measures=f051294e-7710-4809-a4b7-1181628e464b&tab=plots&key=true&title=Full%20Pipeline&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=zyn"><img src="https://api.bencher.dev/v0/projects/zyn/perf/img?branches=d618e093-bbbc-439f-82af-4502c72cd2bd&testbeds=dbe8a0e5-b945-4f98-9cd3-303f96426cd4&benchmarks=19886ff1-468f-4126-a1a8-01b680c66df3,bc8919bf-3786-4119-8232-c86165c96c50&measures=f051294e-7710-4809-a4b7-1181628e464b&title=Full%20Pipeline" title="Full Pipeline" alt="Full Pipeline - Bencher" /></a>

[more benchmarks](https://github.com/aacebo/zyn/blob/main/BENCH.md).

## Try It

```sh
cargo add zyn
```

There's also extension traits behind the `ext` feature for common `syn` operations — `field.is_option()`, `attr.exists("builder")`, keyed field access. Saves some repetitive `syn` traversal.

The [getting started guide](https://aacebo.github.io/zyn) walks through everything. The [API docs](https://docs.rs/zyn) cover every type and trait. The full [builder example](https://github.com/aacebo/zyn/tree/main/examples/builder) from this post is in the repo with tests.

I built zyn because `quote!` was making me miserable. It's not done — there are rough edges around macro hygiene in some edge cases — but it's how I write every proc macro now.

[GitHub](https://github.com/aacebo/zyn) | [crates.io](https://crates.io/crates/zyn) | [Docs](https://docs.rs/zyn)
