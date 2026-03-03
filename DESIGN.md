# Zyn Template Engine — High-Level Design

A `zyn!` proc macro that extends `quote!` with control flow directives (`@if`, `@for`, `@match`), `{{ expr }}` interpolation, `@throw` compile errors, and Dioxus-style `@Element { props }` composition. Output is a `TokenStream`.

## Template Syntax

### Interpolation

Expressions inside `{{ }}` are interpolated into the output `TokenStream`. The expression must implement `ToTokens`. Supports any Rust expression: literals, variables, field access chains (`var.field.inner`), method calls (`foo.bar()`), function calls (`compute(x, y)`), arithmetic, indexing, etc.

```rust
zyn! {
    fn {{ name }}() -> {{ ret_type }} {
        {{ body }}
    }
}
```

Pipes transform the interpolated value before it's converted to tokens:

```rust
zyn! {
    {{ name | upper }}
    {{ val | slice:0:5 }}
    {{ name | trim | upper }}
}
```

### Rust Tokens (passthrough)

Any tokens outside `{{ }}` and `@` directives are passed through verbatim to the output `TokenStream`, exactly like `quote!`:

```rust
zyn! {
    let x: i32 = 42;
    println!("hello");
}
```

### Conditionals

Evaluated at macro-expansion time. `cond` is a Rust boolean expression.

```rust
zyn! {
    fn {{ name }}() {
        @if (is_async) {
            async move { {{ body }} }
        } @else if (is_unsafe) {
            unsafe { {{ body }} }
        } @else {
            {{ body }}
        }
    }
}
```

### Loops

Evaluated at macro-expansion time. `items` must implement `IntoIterator`.

```rust
zyn! {
    @for (field of fields) {
        pub {{ field.name }}: {{ field.ty }},
    }
}
```

### Pattern Matching

Evaluated at macro-expansion time.

```rust
zyn! {
    @match (kind) {
        Kind::Struct => {
            struct {{ name }} { {{ body }} }
        },
        Kind::Enum => {
            enum {{ name }} { {{ body }} }
        },
        _ => {}
    }
}
```

### Compile Errors

Emit a compile error at macro-expansion time.

```rust
zyn! {
    @if (!valid) {
        @throw("invalid input: expected a struct")
    }
}
```

### Elements

Types implementing the `Render` trait can be invoked as elements via `@Name { prop: value }`. The `@` prefix followed by a non-keyword identifier is treated as a element invocation.

```rust
zyn! {
    struct {{ name }} {
        @for (field of fields) {
            @FieldDecl { vis: field.vis, name: field.name, ty: field.ty }
        }
    }
}
```

Elements support path syntax:

```rust
zyn! {
    @elements::Header { title: name }
}
```

Elements support children — a second brace group after props that becomes a nested template passed as the `children` field:

```rust
zyn! {
    @Wrapper { vis: vis_tokens } {
        {{ field_name }}: {{ field_ty }},
    }
}
```

---

## Traits

### Render (in `src/lib.rs`)

```rust
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}
```

Elements are types implementing `Render`. The struct fields are the element's props. Defining a element manually:

```rust
struct FieldDecl {
    vis: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,
}

impl zyn::Render for FieldDecl {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream> {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        Ok(zyn::zyn! {
            {{ vis }} {{ name }}: {{ ty }},
        })
    }
}
```

Or using the `#[zyn::element]` attribute macro:

```rust
#[zyn::element]
fn FieldDecl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    })
}
```

### Pipe (in `src/lib.rs`)

```rust
pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn transform(&self, input: Self::Input) -> Self::Output;
}
```

Pipes transform interpolated values before they're emitted as tokens. Built-in pipes (`upper`, `lower`, `snake`, `camel`, `pascal`, `screaming`) are expanded inline without trait dispatch. Unknown pipe names are treated as expressions implementing `Pipe` and use trait dispatch.

Custom pipe example (manual):

```rust
struct Prefix(String);

impl zyn::Pipe for Prefix {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn transform(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(
            &format!("{}_{}", self.0, input),
            proc_macro2::Span::call_site(),
        )
    }
}
```

Or using the `#[zyn::pipe]` attribute macro:

```rust
#[zyn::pipe]
fn prefix(input: String, pre: &str) -> proc_macro2::Ident {
    proc_macro2::Ident::new(
        &format!("{}_{}", pre, input),
        proc_macro2::Span::call_site(),
    )
}
```

The attribute macro generates the struct + `Render` impl automatically.

---

## Architecture

```
src/
  lib.rs              main crate: Render trait, Pipe trait, re-exports

crates/derive/src/
  lib.rs              crate root, exports zyn! + #[element]
  template/
    mod.rs            public API: expand(TokenStream) -> TokenStream
    parse.rs          parse input token trees into template AST
    ast.rs            AST node types
    expand.rs         AST -> generated Rust code (TokenStream)
    pipe.rs           built-in pipe definitions
  element.rs        #[element] attribute macro
  pipe_macro.rs     #[pipe] attribute macro
```

---

## How It Works

### Phase 1: Parse

The `zyn!` macro receives a `TokenStream` (Rust token trees). The parser (using `syn::parse`) walks the token trees and identifies:

- `{{ ... }}` — double-brace groups → `Interpolation` node
- `@if ( ... ) { ... }` — conditional directive
- `@for ( ... ) { ... }` — loop directive
- `@match ( ... ) { ... }` — match directive
- `@throw( ... )` — compile error directive
- `@Name { prop: value }` — element invocation (non-keyword ident after `@`)
- Everything else — raw tokens passed through

Token tree structure:
- `{{ expr }}` = `Group(Brace, Group(Brace, expr_tokens))` — nested brace groups
- `@` = `Punct('@')` followed by `Ident`

### Phase 2: Expand

The expander walks the AST and generates Rust code that builds a `TokenStream`:

```rust
// Input:
zyn! {
    fn {{ name }}() {
        @if (is_async) {
            async { {{ body }} }
        }
    }
}

// Generated output (conceptual):
{
    let mut __zyn_ts = ::proc_macro2::TokenStream::new();
    ::quote::quote!(fn).to_tokens(&mut __zyn_ts);
    ::quote::ToTokens::to_tokens(&(name), &mut __zyn_ts);
    ::quote::quote!(()).to_tokens(&mut __zyn_ts);
    {
        let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
        if is_async {
            ::quote::quote!(async).to_tokens(&mut __zyn_ts_0);
            {
                let mut __zyn_ts_1 = ::proc_macro2::TokenStream::new();
                ::quote::ToTokens::to_tokens(&(body), &mut __zyn_ts_1);
                ::proc_macro2::Group::new(
                    ::proc_macro2::Delimiter::Brace,
                    __zyn_ts_1,
                ).to_tokens(&mut __zyn_ts_0);
            }
        }
        ::proc_macro2::Group::new(
            ::proc_macro2::Delimiter::Brace,
            __zyn_ts_0,
        ).to_tokens(&mut __zyn_ts);
    }
    __zyn_ts
}
```

---

## Key Types

### AST (`ast.rs`)

```rust
struct Template {
    nodes: Vec<Node>,
}

enum Node {
    Tokens(TokenStream),

    Interpolation {
        expr: TokenStream,
        pipes: Vec<Pipe>,
    },

    If {
        branches: Vec<(TokenStream, Template)>,
        else_body: Option<Template>,
    },

    For {
        binding: Ident,
        iter: TokenStream,
        body: Template,
    },

    Match {
        expr: TokenStream,
        arms: Vec<(TokenStream, Template)>,
    },

    Throw {
        message: TokenStream,
    },

    Element {
        name: TokenStream,
        props: Vec<(Ident, TokenStream)>,
        children: Option<Template>,
    },

    Group {
        delimiter: Delimiter,
        body: Template,
    },
}

struct Pipe {
    name: Ident,
    args: Vec<TokenStream>,
}
```

`TokenStream` = `proc_macro2::TokenStream`. Conditions, expressions, and patterns are kept as opaque token streams — they are Rust code emitted as-is into generated output.

### Group handling

When the parser encounters a `Group` (brace/paren/bracket delimited tokens), it recursively parses the group's contents for template nodes. This is critical because `{{ }}` and `@` directives can appear inside any group.

---

## Parser Design (`parse.rs`)

The parser implements `syn::parse::Parse` for `Template`, using `ParseStream` methods for lookahead and consumption.

```rust
impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> { ... }
}
```

### Parsing algorithm

```
Template::parse(input):
    while !input.is_empty():
        if peek(@) + known keyword → parse directive
        if peek(@) + non-keyword Ident → parse element
        if peek(Brace) and is_interpolation(fork) → parse interpolation
        if peek(Brace|Paren|Bracket) → recursively parse group contents, emit Node::Group
        else → accumulate token into Node::Tokens

parse_interpolation():
    syn::braced!(outer in input)
    syn::braced!(inner in outer)
    split on top-level Pipe punct → expr + pipe chain

parse_if():
    consume @if, parenthesized condition, braced body
    loop @else if / @else

parse_for():
    consume @for, parenthesized (binding of iter), braced body

parse_match():
    consume @match, parenthesized expr, braced arms (pattern => { body })

parse_throw():
    consume @throw, parenthesized message

parse_element():
    consume @ + name/path tokens
    consume braced props: { name: expr, ... }
    optionally consume braced children: { template }
```

### Interpolation detection

`{{ expr }}` in Rust token trees is a `Group(Brace)` containing a single `Group(Brace)`. Use `input.fork()` to speculatively check without consuming.

### Pipe parsing inside interpolation

Inside the inner brace group of `{{ ... }}`, scan for `|` (Pipe) tokens at the top level (not inside sub-groups). Split on pipes:
- First segment = expression
- Subsequent segments = pipe name, optionally followed by `:arg` pairs

---

## Expander Design (`expand.rs`)

The expander takes a `Template` AST and produces a `TokenStream` of Rust code.

```rust
fn expand(template: &Template) -> TokenStream
```

Strategy: generate code that builds a `TokenStream` incrementally using `::quote::ToTokens::to_tokens()`.

### Generated code structure

Each expansion uses a unique identifier (`__zyn_ts_0`, `__zyn_ts_1`, etc.) via a counter to avoid name collisions.

### Node expansion rules

**`Node::Tokens(ts)`** — emit verbatim via `::quote::quote!`

**`Node::Interpolation`** — `::quote::ToTokens::to_tokens(&(expr), &mut __zyn_ts)`, with pipe wrapping if pipes present

**`Node::If`** — Rust `if cond { ... } else if cond { ... } else { ... }`

**`Node::For`** — Rust `for binding in iter { ... }`

**`Node::Match`** — Rust `match expr { pat => { ... }, ... }`

**`Node::Throw`** — `::core::compile_error!(message)`

**`Node::Element`** (no children) — construct struct with props, call `::zyn::Render::render()`, splice result:

```rust
{
    let __zyn_rendered = ::zyn::Render::render(&ElementName {
        prop1: value1,
        prop2: value2,
    })?;
    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut __zyn_ts);
}
```

**`Node::Element`** (with children) — render children first, then pass as `children` field:

```rust
{
    let mut __zyn_ts_N = ::proc_macro2::TokenStream::new();
    // expand children body into __zyn_ts_N
    let __zyn_rendered = ::zyn::Render::render(&ElementName {
        prop1: value1,
        children: __zyn_ts_N,
    })?;
    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut __zyn_ts);
}
```

**`Node::Group`** — create inner ts, expand body, wrap in `::proc_macro2::Group::new(delimiter, inner)`

All generated code uses fully qualified paths per CLAUDE.md: `::proc_macro2::`, `::quote::`, `::core::`, `::zyn::`.

---

## Built-in Pipes (`pipe.rs`)

Pipes are applied at macro-expansion time. They transform values before calling `to_tokens()`.

| Pipe | Description |
|--------|-------------|
| `upper` | Uppercase the string |
| `lower` | Lowercase the string |
| `snake` | Convert to snake_case |
| `camel` | Convert to camelCase |
| `pascal` | Convert to PascalCase |
| `screaming` | Convert to SCREAMING_SNAKE_CASE |

Pipe expansion generates code like:

```rust
// {{ name | upper }}
{
    let __zyn_val = (#name).to_string().to_uppercase();
    let __zyn_ident = ::proc_macro2::Ident::new(&__zyn_val, ::proc_macro2::Span::call_site());
    ::quote::ToTokens::to_tokens(&__zyn_ident, &mut __zyn_ts);
}
```

---

## #[element] Attribute Macro

Transforms a function into a element struct + `Render` impl.

### Input

```rust
#[zyn::element]
fn FieldDecl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    })
}
```

### Generated output

```rust
struct FieldDecl {
    vis: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,
}

impl ::zyn::Render for FieldDecl {
    fn render(&self) -> ::syn::Result<::proc_macro2::TokenStream> {
        let vis = &self.vis;
        let name = &self.name;
        let ty = &self.ty;
        Ok(zyn::zyn! {
            {{ vis }} {{ name }}: {{ ty }},
        })
    }
}
```

### Children support

A parameter named `children` with type `proc_macro2::TokenStream` becomes the children slot:

```rust
#[zyn::element]
fn Wrapper(vis: syn::Visibility, children: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} struct Foo {
            {{ children }}
        }
    })
}

// Usage:
zyn::zyn! {
    @Wrapper { vis: quote::quote!(pub) } {
        name: String,
        age: u32,
    }
}
```

---

## Public API

### Proc macros (in `crates/derive/src/lib.rs`)

```rust
#[proc_macro]
pub fn zyn(input: TokenStream) -> TokenStream { ... }

#[proc_macro_attribute]
pub fn element(attr: TokenStream, item: TokenStream) -> TokenStream { ... }

#[proc_macro_attribute]
pub fn pipe(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
```

### Traits (in `src/lib.rs`)

```rust
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn transform(&self, input: Self::Input) -> Self::Output;
}
```

### Usage from a derive macro

```rust
#[zyn::element]
fn FieldDecl(vis: syn::Visibility, name: syn::Ident, ty: syn::Type) -> syn::Result<proc_macro2::TokenStream> {
    Ok(zyn::zyn! {
        {{ vis }} {{ name }}: {{ ty }},
    })
}

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let fields = extract_fields(&input);

    let output = zyn::zyn! {
        struct {{ name }}Builder {
            @for (field of fields) {
                @FieldDecl { vis: field.vis, name: field.name, ty: field.ty }
            }
        }
    };

    TokenStream::from(output)
}
```

---

## Error Handling

Parse errors use `syn::Error::new_spanned` with the offending token to produce compile errors pointing at the right location.

Element `render()` returns `syn::Result`, so element errors propagate via `?` in the generated code. The surrounding function must return `syn::Result`.

---

## Implementation Order

1. **AST types + Render/Pipe traits** — `ast.rs`, `src/lib.rs`
2. **Parser** — `parse.rs` (syn::parse-based, interpolation detection, directives, elements)
3. **Expander** — `expand.rs` (AST → generated code with fully qualified paths)
4. **Pipes** — `pipe.rs` (built-in pipes + `Pipe` trait dispatch for custom pipes)
5. **Entry point** — `lib.rs` (proc macro definitions)
6. **#[element] macro** — `element.rs`
7. **#[pipe] macro** — `pipe_macro.rs`
8. **Tests** — integration tests in `crates/derive/tests/`
