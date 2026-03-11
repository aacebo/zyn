/// Asserts that two token streams produce identical output.
///
/// Both arguments are evaluated, formatted via [`DebugExt::raw()`](crate::debug::DebugExt::raw),
/// and compared with `assert_eq!`. On failure, the diff shows raw-formatted token output.
///
/// Accepts any type implementing [`ToTokens`](crate::ToTokens), including
/// [`Output`](crate::Output) and `proc_macro2::TokenStream`.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::zyn!(fn hello() {});
/// let expected = quote::quote!(fn hello() {});
/// zyn::assert_tokens!(output, expected);
/// ```
#[macro_export]
macro_rules! assert_tokens {
    ($left:expr, $right:expr) => {{
        use $crate::debug::DebugExt as _;
        let __left = $crate::ToTokens::to_token_stream(&$left);
        let __right = $crate::ToTokens::to_token_stream(&$right);
        let __left_s = __left.debug().raw();
        let __right_s = __right.debug().raw();
        assert_eq!(__left_s, __right_s);
    }};
}

/// Asserts that two token streams produce identical output when pretty-printed.
///
/// Same as [`assert_tokens!`] but uses [`DebugExt::pretty()`](crate::debug::DebugExt::pretty)
/// for formatted Rust source output via `prettyplease`.
///
/// Requires the `pretty` feature.
#[cfg(feature = "pretty")]
#[macro_export]
macro_rules! assert_tokens_pretty {
    ($left:expr, $right:expr) => {{
        use $crate::debug::DebugExt as _;
        let __left = $crate::ToTokens::to_token_stream(&$left);
        let __right = $crate::ToTokens::to_token_stream(&$right);
        let __left_s = __left.debug().pretty();
        let __right_s = __right.debug().pretty();
        assert_eq!(__left_s, __right_s);
    }};
}

#[cfg(test)]
mod tests {
    use crate::quote::quote;

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
        let output = crate::Output::from(tokens.clone());
        assert_tokens!(output, tokens);
    }
}
