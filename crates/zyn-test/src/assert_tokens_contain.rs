/// Asserts that the cleaned token output contains the given substring.
///
/// The token stream is formatted via [`DebugExt::raw()`](zyn_core::debug::DebugExt::clean)
/// before checking for the substring. On failure, prints the full cleaned output.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::zyn!(struct Foo { x: u32 });
/// assert_tokens_contain!(output, "struct Foo");
/// ```
#[macro_export]
macro_rules! assert_tokens_contain {
    ($value:expr, $needle:expr) => {{
        use ::zyn_core::debug::DebugExt as _;
        let __tokens = ::zyn_core::ToTokens::to_token_stream(&$value);
        let __s = __tokens.debug().raw();
        let __needle = $needle;
        assert!(
            __s.contains(__needle),
            "token stream does not contain {:?}\n\ngot:\n{}",
            __needle,
            __s,
        );
    }};
}

/// Asserts that the pretty-printed token output contains the given substring.
///
/// Same as [`assert_tokens_contain!`] but uses [`DebugExt::pretty()`](zyn_core::debug::DebugExt::pretty).
///
/// Requires the `pretty` feature.
#[cfg(feature = "pretty")]
#[macro_export]
macro_rules! assert_tokens_contain_pretty {
    ($value:expr, $needle:expr) => {{
        use ::zyn_core::debug::DebugExt as _;
        let __tokens = ::zyn_core::ToTokens::to_token_stream(&$value);
        let __s = __tokens.debug().pretty();
        let __needle = $needle;
        assert!(
            __s.contains(__needle),
            "token stream does not contain {:?}\n\ngot:\n{}",
            __needle,
            __s,
        );
    }};
}

#[cfg(test)]
mod tests {
    use zyn_core::quote::quote;

    #[test]
    fn contains_substring() {
        let ts = quote!(
            struct Foo {
                x: u32,
            }
        );
        assert_tokens_contain!(ts, "struct Foo");
    }

    #[test]
    #[should_panic(expected = "token stream does not contain")]
    fn missing_substring() {
        let ts = quote!(
            struct Foo;
        );
        assert_tokens_contain!(ts, "struct Bar");
    }
}
