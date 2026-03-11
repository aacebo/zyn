/// Asserts that a token stream or [`Output`](crate::Output) produces no tokens.
///
/// Accepts any type implementing [`ToTokens`](crate::ToTokens). On failure,
/// prints the token content.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::Output::from(proc_macro2::TokenStream::new());
/// zyn::assert_tokens_empty!(output);
/// ```
#[macro_export]
macro_rules! assert_tokens_empty {
    ($value:expr) => {{
        let __tokens = $crate::ToTokens::to_token_stream(&$value);
        assert!(
            __tokens.is_empty(),
            "expected empty token stream, got: {}",
            __tokens,
        );
    }};
}

#[cfg(test)]
mod tests {
    use crate::Output;
    use crate::mark;
    use crate::proc_macro2::TokenStream;
    use crate::quote::quote;

    #[test]
    fn empty_stream() {
        let ts = TokenStream::new();
        assert_tokens_empty!(ts);
    }

    #[test]
    fn empty_output() {
        let output = Output::from(TokenStream::new());
        assert_tokens_empty!(output);
    }

    #[test]
    #[should_panic(expected = "expected empty token stream")]
    fn output_with_diagnostics_emits_compile_error() {
        let output = Output::new()
            .diagnostic(mark::error("something went wrong"))
            .build();
        assert_tokens_empty!(output);
    }

    #[test]
    #[should_panic(expected = "expected empty token stream")]
    fn non_empty_stream() {
        let ts = quote!(
            impl Foo {
                fn validate(&self) -> bool { true }
            }
        );
        assert_tokens_empty!(ts);
    }

    #[test]
    #[should_panic(expected = "expected empty token stream")]
    fn output_with_tokens() {
        let output = Output::new()
            .tokens(quote!(
                struct Foo;
            ))
            .build();
        assert_tokens_empty!(output);
    }
}
