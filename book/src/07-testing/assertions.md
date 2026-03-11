# Assertions

The `zyn!` macro returns [`Output`], which carries both generated tokens and any diagnostics from element renders. zyn provides assertion macros for testing both.

## Setup

Element tests require an `input` variable in scope:

```rust
let input: zyn::Input = zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
    .unwrap()
    .into();
```

## Comparing tokens

Use `assert_tokens!` to compare the full output against an expected `quote!` expression:

```rust
#[test]
fn generates_getter() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let name = zyn::format_ident!("get_name");
    let output = zyn::zyn!(
        fn {{ name }}(&self) -> &str {
            &self.name
        }
    );
    let expected = zyn::quote::quote!(
        fn get_name(&self) -> &str {
            &self.name
        }
    );
    zyn::assert_tokens!(output, expected);
}
```

For partial matching when you only care about a fragment:

```rust
let output = zyn::zyn!(
    struct Foo {
        x: u32,
        y: u32,
    }
);
zyn::assert_tokens_contain!(output, "struct Foo");
// ✓ passes — "struct Foo" appears in the raw token output
```

To verify that an element produced no output (e.g., after `bail!`):

```rust
zyn::assert_tokens_empty!(output);
```

## Asserting diagnostics

When an element emits errors via `error!` or `bail!`, the diagnostics are carried in the `Output`. Assert on them by level and message substring:

```rust
#[zyn::element]
fn validated(name: zyn::syn::Ident) -> zyn::TokenStream {
    if name == "forbidden" {
        bail!("reserved identifier `{}`", name);
    }
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn rejects_forbidden_name() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@validated(name = zyn::format_ident!("forbidden")));
    zyn::assert_diagnostic_error!(output, "reserved identifier");
    zyn::assert_tokens_empty!(output);
    // ✓ error diagnostic with "reserved identifier" message
    // ✓ no tokens produced (bail! stopped execution)
}

#[test]
fn accepts_valid_name() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@validated(name = zyn::format_ident!("hello")));
    zyn::assert_tokens_contain!(output, "fn hello");
    // ✓ tokens contain "fn hello"
}
```

## Warnings don't block output

Non-error diagnostics (`warn!`, `note!`, `help!`) are accumulated but don't suppress token generation. Both tokens and diagnostics are present in the `Output`:

```rust
#[zyn::element]
fn deprecated_getter(name: zyn::syn::Ident) -> zyn::TokenStream {
    warn!("this getter pattern is deprecated");
    zyn::zyn!(fn {{ name }}(&self) -> &str { &self.name })
}

#[test]
fn warning_preserved_alongside_output() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@deprecated_getter(name = zyn::format_ident!("get_name")));
    zyn::assert_tokens_contain!(output, "fn get_name");
    zyn::assert_diagnostic_warning!(output, "deprecated");
    // ✓ tokens produced despite warning
    // ✓ warning diagnostic preserved in output
}
```

## Multiple diagnostics

Elements can accumulate multiple diagnostics across different levels:

```rust
#[zyn::element]
fn multi_diag() -> zyn::TokenStream {
    error!("missing field `x`");
    error!("missing field `y`");
    warn!("deprecated usage");
    bail!();
    zyn::TokenStream::new()
}

#[test]
fn accumulates_all_diagnostics() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@multi_diag());
    zyn::assert_diagnostic_error!(output, "missing field `x`");
    zyn::assert_diagnostic_error!(output, "missing field `y`");
    zyn::assert_diagnostic_warning!(output, "deprecated usage");
    // ✓ all three diagnostics present
    // ✓ both errors and the warning accumulated before bail!
}
```

## Pretty assertions

With the `pretty` feature enabled, use `assert_tokens_pretty!` and `assert_tokens_contain_pretty!` to compare against `prettyplease`-formatted output:

```rust
#[zyn::element]
fn impl_block(name: zyn::syn::Ident) -> zyn::TokenStream {
    zyn::zyn!(
        impl {{ name }} {
            fn validate(&self) -> bool { true }
        }
    )
}

#[test]
fn pretty_token_comparison() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@impl_block(name = zyn::format_ident!("Foo")));
    let expected = zyn::quote::quote!(
        impl Foo {
            fn validate(&self) -> bool { true }
        }
    );
    zyn::assert_tokens_pretty!(output, expected);
    // ✓ compared using prettyplease-formatted output
}

#[test]
fn pretty_substring_check() {
    let input: zyn::Input = zyn::parse!("struct Foo;" => zyn::syn::DeriveInput)
        .unwrap()
        .into();
    let output = zyn::zyn!(@impl_block(name = zyn::format_ident!("Foo")));
    zyn::assert_tokens_contain_pretty!(output, "fn validate");
    // ✓ substring found in pretty-printed output
}
```

## Macro reference

| Macro | Purpose |
|-------|---------|
| `assert_tokens!(actual, expected)` | Compare two token streams (raw-formatted diff on failure) |
| `assert_tokens_empty!(output)` | Assert no tokens produced |
| `assert_tokens_contain!(output, "substr")` | Check for substring in raw token output |
| `assert_diagnostic!(output, Level, "msg")` | Assert diagnostic at a specific level with message |
| `assert_diagnostic_error!(output, "msg")` | Assert error diagnostic |
| `assert_diagnostic_warning!(output, "msg")` | Assert warning diagnostic |
| `assert_diagnostic_note!(output, "msg")` | Assert note diagnostic |
| `assert_diagnostic_help!(output, "msg")` | Assert help diagnostic |
| `assert_compile_error!(output, "msg")` | Alias for `assert_diagnostic_error!` |

With the `pretty` feature enabled:

| Macro | Purpose |
|-------|---------|
| `assert_tokens_pretty!(actual, expected)` | Compare using `prettyplease`-formatted output |
| `assert_tokens_contain_pretty!(output, "substr")` | Substring check on pretty-printed output |
