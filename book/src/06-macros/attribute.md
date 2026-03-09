# `#[zyn::attribute]`

Replaces `#[proc_macro_attribute]`. The annotated item is parsed as `syn::Item` and wrapped in `Input::Item`. Non-extractor parameters (at most one) receive the attribute arguments.

## With Arguments and Extractors

The most common pattern — extract data from the annotated item and receive attribute arguments. The args parameter can be any type that implements `syn::parse::Parse`:

```rust,zyn
use zyn::syn::punctuated::Punctuated;

#[zyn::attribute]
fn trace_var(
    #[zyn(input)] item: zyn::syn::ItemFn,
    args: Punctuated<zyn::syn::Ident, zyn::syn::Token![,]>,
) -> zyn::TokenStream {
    let vars: std::collections::HashSet<zyn::syn::Ident> = args.into_iter().collect();
    // transform the function using vars...
    zyn::zyn!({{ item }})
}
```

Usage — `item` is the annotated function, `args` contains `x` and `y`:

```rust
#[trace_var(x, y)]
fn factorial(mut n: u64) -> u64 {
    let mut p = 1u64;
    while n > 1 {
        p *= n;
        n -= 1;
    }
    p
}

// At runtime, assignments to x and y print their values
```

## Without Arguments

When your attribute doesn't accept arguments, use only extractors:

```rust,zyn
#[zyn::attribute]
fn instrument(
    #[zyn(input)] item: zyn::syn::ItemFn,
) -> zyn::TokenStream {
    let name = &item.sig.ident;
    zyn::zyn!(
        fn {{ name }}() {
            ::std::println!("entering {}", ::std::stringify!({{ name }}));
            let __result = (|| {{ item.block.clone() }})();
            ::std::println!("exiting {}", ::std::stringify!({{ name }}));
            __result
        }
    )
}
```

Usage — no arguments needed:

```rust
#[instrument]
fn compute() {
    // ...
}

// At runtime:
// entering compute
// exiting compute
```

## Without Extractors

When you only need the arguments, not the input item — pass the item through unchanged:

```rust,zyn
#[zyn::attribute]
fn deprecated_alias(args: zyn::syn::LitStr) -> zyn::TokenStream {
    let _alias = args.value();
    zyn::zyn!()
}
```

Usage:

```rust
#[deprecated_alias(name = "old_name")]
fn new_name() { /* ... */ }
```

No `#[zyn(input)]` parameters means the item is still parsed into `input` but nothing extracts from it.

## With Typed Arguments

Any type that implements `syn::parse::Parse` works as the args parameter:

```rust,zyn
#[zyn::attribute]
fn rename(
    #[zyn(input)] item: zyn::syn::ItemFn,
    args: zyn::syn::LitStr,
) -> zyn::TokenStream {
    let new_name = zyn::format_ident!("{}", args.value());
    zyn::zyn!(
        fn {{ new_name }}() {{ item.block.clone() }}
    )
}
```

Usage — the args are parsed as a `LitStr` directly:

```rust
#[rename("calculate")]
fn compute() -> i32 { 42 }

// Generated:
// fn calculate() -> i32 { 42 }
```

## With Struct Extractors

Extract specific item types — the `FromInput` impl handles validation:

```rust,zyn
#[zyn::attribute]
fn add_debug(
    #[zyn(input)] item: zyn::syn::ItemStruct,
) -> zyn::TokenStream {
    let name = &item.ident;
    zyn::zyn!(
        {{ item }}

        impl ::std::fmt::Debug for {{ name }} {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(::std::stringify!({{ name }})).finish()
            }
        }
    )
}
```

Usage:

```rust
#[add_debug]
struct Point {
    x: f32,
    y: f32,
}

// Generated:
// struct Point { x: f32, y: f32 }
// impl Debug for Point {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Point").finish()
//     }
// }
```

If `#[add_debug]` is applied to a non-struct (e.g., a function), the `ItemStruct` extractor produces a compile error automatically.

## Using Elements

Elements work inside attribute bodies because `input` is in scope:

```rust,zyn
#[zyn::element]
fn debug_field(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(
        .field(::std::stringify!({{ name }}), &self.{{ name }})
    )
}

#[zyn::attribute]
fn auto_debug(
    #[zyn(input)] item: zyn::syn::ItemStruct,
    #[zyn(input)] fields: zyn::Fields,
) -> zyn::TokenStream {
    let name = &item.ident;
    zyn::zyn!(
        {{ item }}

        impl ::std::fmt::Debug for {{ name }} {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(::std::stringify!({{ name }}))
                    @for (field in fields.iter()) {
                        @debug_field(name = field.ident.clone().unwrap())
                    }
                    .finish()
            }
        }
    )
}
```

Usage:

```rust
#[auto_debug]
struct User {
    name: String,
    age: u32,
}

// Generated:
// struct User { name: String, age: u32 }
// impl Debug for User {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.debug_struct("User")
//             .field("name", &self.name)
//             .field("age", &self.age)
//             .finish()
//     }
// }
```

## Diagnostics

All diagnostic macros are available — `error!`, `warn!`, `note!`, `help!`, `bail!`:

```rust,zyn
#[zyn::attribute]
fn must_be_pub(
    #[zyn(input)] item: zyn::syn::ItemFn,
) -> zyn::TokenStream {
    if !matches!(item.vis, zyn::syn::Visibility::Public(_)) {
        bail!("function must be public"; span = item.sig.ident.span());
    }

    warn!("this attribute is experimental");

    zyn::zyn!({{ item }})
}
```

Usage — applying to a non-public function produces a compile error:

```rust
#[must_be_pub]
fn private_fn() {}
// error: function must be public
//  --> src/lib.rs:2:4

#[must_be_pub]
pub fn public_fn() {}
// compiles fine (with a warning: "this attribute is experimental")
```

See [Diagnostics](../03-elements/diagnostics.md) for full details on `error!`, `warn!`, `note!`, `help!`, and `bail!`.
