# AttrExt

Extension methods on a single `syn::Attribute`.

```rust
use zyn::ext::AttrExt;

let attr: &syn::Attribute = /* ... */;

attr.is("serde")          // true if the attribute path is "serde"
let args = attr.args()?;  // parse the attribute's argument list as Args
```

## Checking the Attribute Name

`is` matches the last path segment, so `#[serde(...)]`, `#[::serde(...)]`, and `#[mymod::serde(...)]` all match `attr.is("serde")`:

```rust
#[proc_macro_derive(MyDerive, attributes(my_attr))]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    for attr in &input.attrs {
        if attr.is("my_attr") {
            let args = attr.args().unwrap();
            // process args...
        }
    }
    // ...
}
```

## Parsing Arguments

`args()` returns an `Args` container. It parses the token list inside the attribute's parentheses:

```rust
// Given: #[my_attr(skip, rename = "foo")]
let args = attr.args()?;

args.has("skip")          // true
args.get("rename")        // Some(Arg::Expr(...))
```

Returns `None` / an empty `Args` for attributes without parentheses (e.g., `#[derive]`, `#[inline]`).
