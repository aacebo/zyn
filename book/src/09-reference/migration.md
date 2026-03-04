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

These patterns have no template-level equivalent and require direct use of `syn`, `quote`, or `proc_macro2` via `zyn::syn`, `zyn::quote`, `zyn::proc_macro2`:

| Pattern | Why |
|---|---|
| `syn::parse_quote!(pub)` | Constructs typed `syn` AST values — needed for element prop values |
| `quote::format_ident!("name")` outside a template | Creates `Ident` values to pass INTO templates as input |
| `syn::parse_macro_input!(input as DeriveInput)` | Proc macro entry point parsing |
| `syn::fold::Fold` / `syn::visit::Visit` | Full AST visitor traversal — too complex for template syntax |
| `syn::parse2::<T>(tokens)` | Converting template output back to typed AST for further manipulation |
| `Span::call_site()` / `Span::mixed_site()` | Explicit span control — templates use call-site spans by default |
| `TokenStream::extend()` / manual accumulation | Templates handle accumulation automatically via `@for` / `@if` |

> In book examples and tests, `quote::format_ident!()` and `syn::parse_quote!()` appear frequently because they construct illustrative input values. In real proc macros, these values typically come from parsed input (`input.ident`, `field.ty`, etc.) rather than being constructed manually.
