use quote::quote;
use zyn_core::ast::Element;

mod upper {
    use super::*;

    #[test]
    fn expands_correctly() {
        let element: Element = syn::parse_str("{{ name | upper }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Upper), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod lower {
    use super::*;

    #[test]
    fn expands_correctly() {
        let element: Element = syn::parse_str("{{ name | lower }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Lower), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod snake {
    use super::*;

    #[test]
    fn expands_correctly() {
        let element: Element = syn::parse_str("{{ name | snake }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Snake), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod camel {
    use super::*;

    #[test]
    fn expands_correctly() {
        let element: Element = syn::parse_str("{{ name | camel }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Camel), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod pascal {
    use super::*;

    #[test]
    fn expands_correctly() {
        let element: Element = syn::parse_str("{{ name | pascal }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Pascal), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod screaming {
    use super::*;

    #[test]
    fn expands_correctly() {
        let element: Element = syn::parse_str("{{ name | screaming }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Screaming), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod chained {
    use super::*;

    #[test]
    fn snake_then_upper() {
        let element: Element = syn::parse_str("{{ name | snake | upper }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Snake), __zyn_val);
                    let __zyn_val = __zyn_val.to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(Upper), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod custom {
    use super::*;

    #[test]
    fn dispatches_via_trait() {
        let element: Element = syn::parse_str("{{ name | my_pipe }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                {
                    let __zyn_val = (name).to_string();
                    let __zyn_val = ::zyn::Pipe::pipe(&(MyPipe), __zyn_val);
                    ::quote::ToTokens::to_tokens(&__zyn_val, &mut __zyn_ts_0);
                }
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}
