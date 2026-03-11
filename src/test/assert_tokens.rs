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
    use crate::Output;
    use crate::mark;
    use crate::quote::quote;

    #[test]
    fn impl_block() {
        let a = quote!(
            impl Foo {
                fn validate(&self) -> bool {
                    self.x > 0
                }
                fn name(&self) -> &str {
                    &self.name
                }
            }
        );
        let b = quote!(
            impl Foo {
                fn validate(&self) -> bool {
                    self.x > 0
                }
                fn name(&self) -> &str {
                    &self.name
                }
            }
        );
        assert_tokens!(a, b);
    }

    #[test]
    fn struct_with_generics() {
        let a = quote!(
            struct Builder<T: Clone + Default>
            where
                T: Send,
            {
                value: T,
                ready: bool,
            }
        );
        let b = quote!(
            struct Builder<T: Clone + Default>
            where
                T: Send,
            {
                value: T,
                ready: bool,
            }
        );
        assert_tokens!(a, b);
    }

    #[test]
    fn output_includes_diagnostic_tokens() {
        let tokens = quote!(
            impl Foo {
                fn validate(&self) -> bool { true }
            }
        );
        let output = Output::new()
            .tokens(tokens)
            .diagnostic(mark::warning("deprecated"))
            .build();
        crate::assert_tokens_contain!(output, "fn validate");
        crate::assert_tokens_contain!(output, "deprecated");
    }

    #[test]
    #[should_panic]
    fn mismatched_method_body() {
        let a = quote!(
            impl Foo {
                fn validate(&self) -> bool { self.x > 0 }
            }
        );
        let b = quote!(
            impl Foo {
                fn validate(&self) -> bool { self.y > 0 }
            }
        );
        assert_tokens!(a, b);
    }

    #[cfg(feature = "pretty")]
    mod pretty {
        use crate::Output;
        use crate::mark;
        use crate::quote::quote;

        #[test]
        fn impl_block() {
            let a = quote!(
                impl Foo {
                    fn validate(&self) -> bool {
                        self.x > 0
                    }
                    fn name(&self) -> &str {
                        &self.name
                    }
                }
            );
            let b = quote!(
                impl Foo {
                    fn validate(&self) -> bool {
                        self.x > 0
                    }
                    fn name(&self) -> &str {
                        &self.name
                    }
                }
            );
            assert_tokens_pretty!(a, b);
        }

        #[test]
        fn output_includes_diagnostic_tokens() {
            let tokens = quote!(
                impl Foo {
                    fn validate(&self) -> bool { true }
                }
            );
            let output = Output::new()
                .tokens(tokens)
                .diagnostic(mark::warning("deprecated"))
                .build();
            crate::assert_tokens_contain_pretty!(output, "fn validate");
        }

        #[test]
        #[should_panic]
        fn mismatched_method_body() {
            let a = quote!(
                impl Foo {
                    fn validate(&self) -> bool { self.x > 0 }
                }
            );
            let b = quote!(
                impl Foo {
                    fn validate(&self) -> bool { self.y > 0 }
                }
            );
            assert_tokens_pretty!(a, b);
        }
    }
}
