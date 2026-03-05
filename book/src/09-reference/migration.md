# Migration from Raw quote/syn

Common `quote`, `syn`, and `proc_macro2` patterns and their zyn equivalents. Use this table when converting existing proc macros to zyn templates.

## Inside Templates (`zyn!`)

| Raw Pattern | Zyn Alternative |
|---|---|
| `quote! { fn #name() {} }` | `zyn! { fn {{ name }}() {} }` |
| `format_ident!("get_{}", name)` | `{{ name \| ident:"get_{}" }}` |
| `format_ident!("{}_impl", name)` | `{{ name \| ident:"{}_impl" }}` |
| `name.to_string().to_uppercase()` | `{{ name \| upper }}` |
| `name.to_string().to_lowercase()` | `{{ name \| lower }}` |
| `to_snake_case(name)` | `{{ name \| snake }}` |
| `to_pascal_case(name)` | `{{ name \| pascal }}` |
| `to_camel_case(name)` | `{{ name \| camel }}` |
| `LitStr::new(&name, span)` | `{{ name \| str }}` |
| `if cond { quote!(...) } else { quote!() }` | `@if (cond) { ... } @else { ... }` |
| `items.iter().map(\|i\| quote!(...)).collect()` + `#(#tokens)*` | `@for (item in items) { ... }` |
| `match kind { A => quote!(...), _ => quote!() }` | `@match (kind) { A => { ... } _ => {} }` |
| `compile_error!("msg")` | `@throw "msg"` |
| Manual `#[deprecated]` trick for warnings | `@warn "msg"` |
| Reusable `fn render(...) -> TokenStream` | `#[zyn::element]` |
| Reusable `fn transform(input: String) -> Ident` | `#[zyn::pipe]` |

## No Zyn Alternative (use raw APIs)

These patterns have no template-level equivalent but are available through zyn's public API:

| Pattern | zyn equivalent |
|---|---|
| `syn::parse_macro_input!(input as DeriveInput)` | `zyn::parse_input!(input as DeriveInput)` |
| `syn::parse_str::<T>("...")` | `zyn::parse!("..." => T)` |
| `syn::parse2::<T>(tokens)` | `zyn::parse!(tokens => T)` |
| `quote::format_ident!("name")` | `zyn::format_ident!("name")` |
| `Span::call_site()` | `zyn::Span::call_site()` |
| `syn::fold::Fold` / `syn::visit::Visit` | `syn::fold::Fold` |

> In book examples and tests, `zyn::format_ident!()` appears frequently because it constructs illustrative input values. In real proc macros, these values typically come from parsed input (`input.ident`, `field.ty`, etc.) rather than being constructed manually.
