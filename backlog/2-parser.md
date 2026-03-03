# Phase 2: Parser

## Scope

Parse a `proc_macro2::TokenStream` (the input to `zyn!`) into a `Template` AST using `syn::parse` infrastructure. The parser walks token trees and identifies interpolations, directives, element invocations, and passthrough tokens.

## Files to Create

- `crates/derive/src/template/parse.rs`

## Design

### Using `syn::parse`

Implement `syn::parse::Parse` for `Template` so the entry point is `syn::parse2::<Template>(input)`. The parser uses `ParseStream` methods (`peek`, `parse`, `fork`) and `syn::braced!` / `syn::parenthesized!` macros for group handling.

```rust
impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            // detect and parse nodes
        }
        Ok(Template { nodes })
    }
}
```

### Interpolation detection

`{{ expr }}` in Rust token trees is a `Group(Brace)` containing exactly one `Group(Brace)`. Detection:

```rust
if input.peek(syn::token::Brace) {
    let fork = input.fork();
    let content;
    syn::braced!(content in fork);
    if content.peek(syn::token::Brace) && {
        let inner_fork = content.fork();
        let inner;
        syn::braced!(inner in inner_fork);
        inner_fork.is_empty()
    } {
        // it's an interpolation — advance the real input
        input.advance_to(&fork);
        // parse inner brace group for expr + pipes
    }
}
```

Inside the inner brace group, scan for top-level `|` (Pipe) punctuation to split expr from pipes. First segment = expression, subsequent segments = pipe name with optional `:arg` pairs.

### Directive and element detection

`@if`, `@for`, `@match`, `@throw` — `input.peek(Token![@])` then parse `@`, peek at the next `Ident`, check if it matches a known keyword.

`@Element { props }` (element) — `@` followed by an `Ident` that is NOT a keyword. The parser collects the element name (which may be a path like `my_mod::Header`), then expects a `{ prop: value, ... }` brace group for props. Optionally followed by another `{ ... }` brace group for children.

### Parsing rules

```
Template::parse(input: ParseStream):
    while !input.is_empty():
        if peek(@) + known keyword → parse directive
        if peek(@) + non-keyword Ident → parse element
        if peek(Brace) and is_interpolation(fork) → parse interpolation
        if peek(Brace|Paren|Bracket) → syn::braced!/parenthesized!/bracketed!
            recursively parse contents as Template, emit Node::Group
        else → input.parse::<TokenTree>(), accumulate into Node::Tokens

parse_interpolation(input):
    syn::braced!(outer in input)
    syn::braced!(inner in outer)
    collect tokens from `inner`, split on top-level Pipe punct
    first segment → expr TokenStream
    each subsequent segment → Pipe { name: Ident, args: Vec<TokenStream> }

parse_if(input):
    input.parse::<Token![@]>()
    input.parse::<syn::Ident>()  // "if"
    syn::parenthesized!(cond in input) → condition as TokenStream
    syn::braced!(body in input) → syn::parse2::<Template>(body)
    while input.peek(Token![@]) && input.peek2(syn::Ident) == "else":
        input.parse::<Token![@]>()
        input.parse::<syn::Ident>()  // "else"
        if input.peek(syn::Ident) && == "if":
            input.parse::<syn::Ident>()  // "if"
            parse condition + body → push branch
        else:
            parse body → set else_body, break

parse_for(input):
    input.parse::<Token![@]>()
    input.parse::<syn::Ident>()  // "for"
    syn::parenthesized!(params in input):
        params.parse::<syn::Ident>()  // binding
        expect "of" ident
        collect remaining as iter TokenStream
    syn::braced!(body in input) → syn::parse2::<Template>(body)

parse_match(input):
    input.parse::<Token![@]>()
    input.parse::<syn::Ident>()  // "match"
    syn::parenthesized!(expr_content in input) → expr as TokenStream
    syn::braced!(arms_content in input):
        loop:
            collect tokens until FatArrow (=>) → pattern TokenStream
            syn::braced!(arm_body in arms_content) → syn::parse2::<Template>
            optional comma
            break if arms_content.is_empty()

parse_throw(input):
    input.parse::<Token![@]>()
    input.parse::<syn::Ident>()  // "throw"
    syn::parenthesized!(msg in input) → message as TokenStream

parse_element(input):
    input.parse::<Token![@]>()
    collect element name/path tokens (Ident, ::, Ident, ...)
    syn::braced!(props_content in input):
        loop:
            parse prop name: props_content.parse::<syn::Ident>()
            expect Token![:]
            collect value tokens until comma or end → prop value TokenStream
            optional comma
            break if props_content.is_empty()
    if input.peek(syn::token::Brace):
        syn::braced!(children_content in input) → syn::parse2::<Template>
        set children = Some(template)
    emit Node::Element { name, props, children }
```

### Element syntax

```rust
// Simple element, no children
@FieldDecl { vis: field.vis, name: field.name, ty: field.ty }

// Element with path
@elements::FieldDecl { vis: field.vis, name: field.name }

// Element with children
@Wrapper { vis: vis_tokens } {
    {{ field_name }}: {{ field_ty }},
}
```

The props brace group uses `name: expr` syntax (like Rust struct literals). The optional second brace group is the children body — a nested template that gets rendered and passed as the `children` field.

### Expression support

Interpolation `{{ }}` accepts any Rust expression: literals, variables, field access chains (`var.field.inner`), method calls (`foo.bar()`), function calls (`compute(x, y)`), arithmetic (`a + b`), indexing (`items[0]`), etc. The tokens are preserved as-is.

Directive conditions/iterables/match-exprs also accept arbitrary Rust expressions and patterns.

Element prop values accept arbitrary Rust expressions.

### Error handling

Use `syn::Error::new_spanned` with the offending token for all parse errors. Return `syn::Result<Template>`.

## Acceptance Criteria

- `cargo build -p zyn-derive` compiles
- `cargo clippy -p zyn-derive -- -D warnings` passes
- Parser correctly identifies nested `{{ }}` as interpolation vs regular brace groups
- Parser handles `@if`/`@else if`/`@else` chains
- Parser handles `@for (x of iter) { }`
- Parser handles `@match` with multiple arms
- Parser handles `@throw`
- Parser handles `@Element { prop: value }` with and without children
- Parser handles element paths like `@my_mod::Header { ... }`
