use quote::quote;
use zyn::ast::Element;

mod tokens {
    use super::*;

    #[test]
    fn plain() {
        let element: Element = syn::parse_str("struct Foo ;").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::proc_macro2::TokenStream::new();
                ::quote::quote!(struct Foo ;).to_tokens(&mut __zyn_ts_0);
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
