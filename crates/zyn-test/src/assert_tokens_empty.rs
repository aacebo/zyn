/// Asserts that a token stream or [`Output`](zyn_core::Output) produces no tokens.
///
/// Accepts any type implementing [`ToTokens`](zyn_core::ToTokens). On failure,
/// prints the token content.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::Output::from(proc_macro2::TokenStream::new());
/// assert_tokens_empty!(output);
/// ```
#[macro_export]
macro_rules! assert_tokens_empty {
    ($value:expr) => {{
        let __tokens = ::zyn_core::ToTokens::to_token_stream(&$value);
        assert!(
            __tokens.is_empty(),
            "expected empty token stream, got: {}",
            __tokens,
        );
    }};
}

#[cfg(test)]
mod tests {
    use zyn_core::proc_macro2::TokenStream;
    use zyn_core::quote::quote;

    #[test]
    fn empty_stream() {
        let ts = TokenStream::new();
        assert_tokens_empty!(ts);
    }

    #[test]
    fn empty_output() {
        let output = zyn_core::Output::from(TokenStream::new());
        assert_tokens_empty!(output);
    }

    #[test]
    #[should_panic(expected = "expected empty token stream")]
    fn non_empty_stream() {
        let ts = quote!(
            struct Foo;
        );
        assert_tokens_empty!(ts);
    }
}
