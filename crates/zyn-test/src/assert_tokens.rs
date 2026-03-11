/// Asserts that two token streams produce identical output when cleaned.
///
/// Both arguments are evaluated, formatted via [`DebugExt::raw()`](zyn_core::debug::DebugExt::clean),
/// and compared with `assert_eq!`. On failure, the diff shows raw-formatted token output.
///
/// Accepts any type implementing [`ToTokens`](zyn_core::ToTokens), including
/// [`Output`](zyn_core::Output) and `proc_macro2::TokenStream`.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::zyn!(fn hello() {});
/// let expected = quote::quote!(fn hello() {});
/// assert_tokens!(output, expected);
/// ```
#[macro_export]
macro_rules! assert_tokens {
    ($left:expr, $right:expr) => {{
        use ::zyn_core::debug::DebugExt as _;
        let __left = ::zyn_core::ToTokens::to_token_stream(&$left);
        let __right = ::zyn_core::ToTokens::to_token_stream(&$right);
        let __left_s = __left.debug().raw();
        let __right_s = __right.debug().raw();
        assert_eq!(__left_s, __right_s);
    }};
}

/// Asserts that two token streams produce identical output when pretty-printed.
///
/// Same as [`assert_tokens!`] but uses [`DebugExt::pretty()`](zyn_core::debug::DebugExt::pretty)
/// for formatted Rust source output via `prettyplease`.
///
/// Requires the `pretty` feature.
#[cfg(feature = "pretty")]
#[macro_export]
macro_rules! assert_tokens_pretty {
    ($left:expr, $right:expr) => {{
        use ::zyn_core::debug::DebugExt as _;
        let __left = ::zyn_core::ToTokens::to_token_stream(&$left);
        let __right = ::zyn_core::ToTokens::to_token_stream(&$right);
        let __left_s = __left.debug().pretty();
        let __right_s = __right.debug().pretty();
        assert_eq!(__left_s, __right_s);
    }};
}

#[cfg(test)]
mod tests {
    use zyn_core::quote::quote;

    #[test]
    fn matching_tokens() {
        let a = quote!(
            fn hello() {}
        );
        let b = quote!(
            fn hello() {}
        );
        assert_tokens!(a, b);
    }

    #[test]
    #[should_panic]
    fn mismatched_tokens() {
        let a = quote!(
            fn hello() {}
        );
        let b = quote!(
            fn world() {}
        );
        assert_tokens!(a, b);
    }

    #[test]
    fn output_vs_token_stream() {
        let tokens = quote!(
            struct Foo;
        );
        let output = zyn_core::Output::from(tokens.clone());
        assert_tokens!(output, tokens);
    }
}
