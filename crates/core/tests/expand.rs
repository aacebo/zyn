use quote::quote;
use zyn_core::ast::Element;

mod tokens {
    use super::*;

    #[test]
    fn plain() {
        let element: Element = syn::parse_str("struct Foo ;").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                __zyn_ts_0.extend(::quote::quote!(struct Foo ;));
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod interp {
    use super::*;

    #[test]
    fn no_pipes() {
        let element: Element = syn::parse_str("{{ name }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                ::quote::ToTokens::to_tokens(&(name), &mut __zyn_ts_0);
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod throw {
    use super::*;

    #[test]
    fn generates_compile_error() {
        let element: Element = syn::parse_str("@throw \"bad input\"").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                ::core::compile_error!("bad input");
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}
