# Phase 1: AST Types + Render Trait

## Scope

Define the AST node types and the `Render` trait. The `Render` trait enables Dioxus-style elements — reusable units of code generation that accept named props and return a `TokenStream`.

## Files to Create

- `crates/derive/src/template/ast.rs`
- `crates/derive/src/template/mod.rs` (stub)
- `crates/derive/src/lib.rs` (add `mod template;`)

## Files to Modify

- `src/lib.rs` — add `Render` trait, re-export `zyn_derive::zyn`

## Render Trait (in `src/lib.rs`)

```rust
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}
```

Elements are types implementing `Render`. The struct fields are the element's props. Inside `zyn!`, elements are invoked via `@ElementName { prop: value }`.

Example element definition (manual):

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

Example usage inside a template:

```rust
zyn::zyn! {
    struct {{ name }} {
        @for (field of fields) {
            @FieldDecl { vis: field.vis, name: field.name, ty: field.ty }
        }
    }
}
```

## AST Types (in `crates/derive/src/template/ast.rs`)

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

    Group {
        delimiter: Delimiter,
        body: Template,
    },

    Throw {
        message: TokenStream,
    },

    Element {
        name: TokenStream,
        props: Vec<(Ident, TokenStream)>,
        children: Option<Template>,
    },
}

struct Pipe {
    name: Ident,
    args: Vec<TokenStream>,
}
```

`Node::Element` captures:
- `name` — the element type path (e.g., `FieldDecl`, `my_mod::Header`)
- `props` — named key-value pairs: `prop_name: expr`
- `children` — optional nested template body, passed as a `children` field on the constructed struct (the element's `render()` can use it)

`TokenStream` = `proc_macro2::TokenStream`. Conditions, expressions, and patterns are kept as opaque token streams — they are Rust code emitted as-is into generated output.

`Node::Group` is needed because brace/paren/bracket groups may contain `{{ }}` or `@` directives inside them, so their contents must be recursively parsed.

`Node::Tokens` accumulates consecutive raw tokens that pass through verbatim (like `quote!`).

## Acceptance Criteria

- `cargo build --workspace` compiles
- `cargo clippy --workspace --all-features -- -D warnings` passes
