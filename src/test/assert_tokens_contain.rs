/// Asserts that the token output contains the given substring.
///
/// The token stream is formatted via [`DebugExt::raw()`](crate::debug::DebugExt::raw)
/// before checking for the substring. On failure, prints the full output.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::zyn!(struct Foo { x: u32 });
/// zyn::assert_tokens_contain!(output, "struct Foo");
/// ```
#[macro_export]
macro_rules! assert_tokens_contain {
    ($value:expr, $needle:expr) => {{
        use $crate::debug::DebugExt as _;
        let __tokens = $crate::ToTokens::to_token_stream(&$value);
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
/// Same as [`assert_tokens_contain!`] but uses [`DebugExt::pretty()`](crate::debug::DebugExt::pretty).
///
/// Requires the `pretty` feature.
#[cfg(feature = "pretty")]
#[macro_export]
macro_rules! assert_tokens_contain_pretty {
    ($value:expr, $needle:expr) => {{
        use $crate::debug::DebugExt as _;
        let __tokens = $crate::ToTokens::to_token_stream(&$value);
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
    use crate::quote::quote;

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
