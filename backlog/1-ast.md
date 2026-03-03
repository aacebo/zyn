# Phase 1 + 2: AST Types + Parsing + Traits ✅

## Status: Complete

## Structure

```
src/
  lib.rs                  Expand trait, Render trait, Pipe trait
  ident.rs                ident::Iter (infinite Iterator<Item = Ident>)
  ast/
    mod.rs                Node enum (4 variants), Element struct, Parse/Expand impls
    tokens_node.rs        TokensNode + Expand
    interp_node.rs        InterpNode + Parse + Expand
    pipe_node.rs          PipeNode + Parse + Expand
    group_node.rs         GroupNode + Parse + Expand
    at/
      mod.rs              AtNode enum (5 variants), Parse + Expand dispatch
      if_node.rs          IfNode + Parse + Expand
      for_node.rs         ForNode + Parse + Expand
      match_node.rs       MatchNode + Parse + Expand
      throw_node.rs       ThrowNode + Parse + Expand
      element_node.rs     ElementNode + Parse + Expand + parse_with_ident()

crates/derive/src/
  lib.rs                  (empty — proc macro entry points go here later)
```

## Traits (in `src/lib.rs`)

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

## Ident Iterator (`src/ident.rs`)

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

`Parse for AtNode` consumes `@` + ident, dispatches to the appropriate variant parser.

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
