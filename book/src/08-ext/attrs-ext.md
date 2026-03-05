# AttrsExt

Extension methods on a slice of `syn::Attribute`. Most derive macros work with `&[syn::Attribute]` from `DeriveInput::attrs` or field attrs.

```rust
use zyn::ext::AttrsExt;

let attrs: &[syn::Attribute] = &input.attrs;

attrs.has_attr("skip")                // true if any attr is named "skip"
attrs.find_attr("rename")             // Option<&Attribute>
let args = attrs.find_args("serde")?; // Option<Args> for the first matching attr
let args = attrs.merge_args("zyn")?;  // merge args from all #[zyn(...)] attrs
```

## Checking for a Flag Attribute

Test whether a `#[skip]` or similar marker is present:

```rust
// Check if a field should be skipped
for field in &input.fields {
    if field.attrs.has_attr("skip") {
        continue;
    }
    // process field...
}
```

## Finding a Single Attribute

`find_attr` returns the first attribute with the given name:

```rust
// Find #[rename(...)] if present
if let Some(attr) = attrs.find_attr("rename") {
    let args = attr.args()?;
    // ...
}
```

## Parsing Arguments from One Attribute

`find_args` combines `find_attr` + `args()` in one call:

```rust
// Given: #[serde(rename_all = "camelCase")]
if let Some(args) = attrs.find_args("serde")? {
    if let Some(arg) = args.get("rename_all") {
        // arg is Arg::Expr with value "camelCase"
    }
}
```

## Merging Repeated Attributes

`merge_args` collects arguments from all attributes with the same name and merges them. Later attributes override earlier ones on key conflicts:

```rust
// Given:
//   #[zyn(skip)]
//   #[zyn(rename = "foo")]
let args = attrs.merge_args("zyn")?;
// args contains both "skip" and "rename = foo"
```
