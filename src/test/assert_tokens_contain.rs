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
    use crate::Output;
    use crate::mark;
    use crate::quote::quote;

    #[test]
    fn finds_method_in_impl() {
        let ts = quote!(
            impl Foo {
                fn validate(&self) -> bool { self.x > 0 }
                fn name(&self) -> &str { &self.name }
            }
        );
        assert_tokens_contain!(ts, "fn validate");
        assert_tokens_contain!(ts, "fn name");
    }

    #[test]
    fn finds_return_type() {
        let ts = quote!(
            impl Foo {
                fn validate(&self) -> bool { true }
            }
        );
        assert_tokens_contain!(ts, "-> bool");
    }

    #[test]
    #[should_panic(expected = "token stream does not contain")]
    fn does_not_find_absent_method() {
        let ts = quote!(
            impl Foo {
                fn validate(&self) -> bool { true }
            }
        );
        assert_tokens_contain!(ts, "fn missing_method");
    }

    #[test]
    fn output_with_diagnostics_searches_tokens() {
        let tokens = quote!(
            impl Foo {
                fn validate(&self) -> bool { true }
            }
        );
        let output = Output::new()
            .tokens(tokens)
            .diagnostic(mark::error("something went wrong"))
            .build();
        assert_tokens_contain!(output, "fn validate");
    }

    #[cfg(feature = "pretty")]
    mod pretty {
        use crate::Output;
        use crate::mark;
        use crate::quote::quote;

        #[test]
        fn finds_indented_method() {
            let ts = quote!(
                impl Foo {
                    fn validate(&self) -> bool { self.x > 0 }
                    fn name(&self) -> &str { &self.name }
                }
            );
            assert_tokens_contain_pretty!(ts, "fn validate");
            assert_tokens_contain_pretty!(ts, "fn name");
        }

        #[test]
        #[should_panic(expected = "token stream does not contain")]
        fn does_not_find_absent_method() {
            let ts = quote!(
                impl Foo {
                    fn validate(&self) -> bool { true }
                }
            );
            assert_tokens_contain_pretty!(ts, "fn missing_method");
        }

        #[test]
        fn output_with_diagnostics_searches_tokens() {
            let tokens = quote!(
                impl Foo {
                    fn validate(&self) -> bool { true }
                }
            );
            let output = Output::new()
                .tokens(tokens)
                .diagnostic(mark::error("something went wrong"))
                .build();
            assert_tokens_contain_pretty!(output, "fn validate");
        }
    }
}
