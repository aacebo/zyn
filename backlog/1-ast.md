# Phase 1 + 2: AST Types + Parsing + Traits ✅

## Status: Complete

## Structure

```
crates/core/src/
  lib.rs                  Expand trait, Render trait, Pipe trait, built-in pipe structs
  ident.rs                ident::Iter (infinite Iterator<Item = Ident>)
  ast/
    mod.rs                Node enum (4 variants), Element struct, Parse/Expand impls
    tokens_node.rs        TokensNode + Expand
    interp_node.rs        InterpNode + Parse + Expand
    pipe_node.rs          PipeNode struct + Parse + Expand
    group_node.rs         GroupNode + Parse + Expand
    at/
      mod.rs              AtNode enum (5 variants), Parse + Expand dispatch
      if_node.rs          IfNode + Parse + Expand
      for_node.rs         ForNode + Parse + Expand
      match_node.rs       MatchNode + Parse + Expand
      throw_node.rs       ThrowNode + Parse + Expand
      element_node.rs     ElementNode + Parse + Expand + parse_with_ident()

crates/derive/src/
  lib.rs                  #[proc_macro] pub fn zyn

src/
  lib.rs                  Re-exports from zyn-core and zyn-derive
```

## Traits (in `crates/core/src/lib.rs`)

```rust
pub trait Expand {
    fn expand(&self, output: &proc_macro2::Ident, idents: &mut ident::Iter) -> proc_macro2::TokenStream;
}

pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;
    fn pipe(&self, input: Self::Input) -> Self::Output;
}
```

## Built-in Pipes (in `crates/core/src/lib.rs`)

PascalCase unit structs implementing `Pipe`, each with `Input = String` and `Output = proc_macro2::Ident`:

| Struct | Behavior |
|--------|----------|
| `Upper` | `input.to_uppercase()` |
| `Lower` | `input.to_lowercase()` |
| `Snake` | snake_case conversion |
| `Camel` | camelCase conversion (delegates to `Pascal` + lowercase first char) |
| `Pascal` | PascalCase conversion |
| `Screaming` | SCREAMING_CASE conversion (delegates to `Snake` + uppercase) |

Users bring pipes into scope: `use zyn::Upper;` — same for built-in and custom pipes.

## Ident Iterator (`crates/core/src/ident.rs`)

`ident::Iter` — struct with counter, implements `Iterator<Item = Ident>`. Yields `__zyn_ts_0`, `__zyn_ts_1`, etc. infinitely. Used by `Expand` impls for unique variable names.

## Node enum (`ast/mod.rs`)

```rust
pub enum Node {
    Tokens(TokensNode),
    Interp(InterpNode),
    At(AtNode),
    Group(GroupNode),
}
```

4 top-level variants. The `@` directives are nested under `AtNode`.

## AtNode enum (`ast/at/mod.rs`)

```rust
pub enum AtNode {
    If(IfNode),
    For(ForNode),
    Match(MatchNode),
    Throw(ThrowNode),
    Element(ElementNode),
}
```

`Parse for AtNode` consumes `@` + ident (via `Ident::parse_any` for keywords), dispatches to the appropriate variant parser.

## PipeNode (`ast/pipe_node.rs`)

```rust
pub struct PipeNode {
    pub span: Span,
    pub name: syn::Ident,
    pub args: Vec<TokenStream>,
}
```

Single struct for all pipes (no built-in vs custom distinction). Expansion generates uniform `::zyn::Pipe::pipe(&(#name), __zyn_val)` for every pipe.

## Element (root container)

```rust
pub struct Element {
    pub nodes: Vec<Node>,
}
```

- `Parse for Element` — main loop: `@` → AtNode, `{{ }}` → InterpNode, groups → GroupNode, else → TokensNode accumulation
- `Expand for Element` — iterates nodes, calls `node.expand()` for each
- `Element::to_token_stream()` — top-level entry, wraps in `{ let mut __zyn_ts_0 = ...; ... __zyn_ts_0 }`

## Conventions

- Each struct has `pub` fields including `span: Span`
- Each struct has `span(&self) -> Span` method
- `From<T>` impls for all variant types into their parent enum
- `is_*` / `as_*` methods on Node and AtNode
- All generated code uses fully qualified paths: `::proc_macro2::`, `::quote::`, `::zyn::`, `::core::`
