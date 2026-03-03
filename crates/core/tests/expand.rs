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

mod syntax_errors {
    use zyn_core::ast::Element;

    fn parse_err(input: &str) -> String {
        match syn::parse_str::<Element>(input) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("expected parse error for: {input}"),
        }
    }

    #[test]
    fn empty_interpolation() {
        let msg = parse_err("{{ }}");
        assert!(
            msg.contains("empty interpolation"),
            "expected 'empty interpolation', got: {msg}"
        );
    }

    #[test]
    fn throw_missing_message() {
        let msg = parse_err("@throw");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }

    #[test]
    fn else_without_if() {
        let msg = parse_err("@else { foo }");
        assert!(
            msg.contains("unexpected @else without @if"),
            "expected 'unexpected @else without @if', got: {msg}"
        );
    }

    #[test]
    fn for_in_keyword_hint() {
        let msg = parse_err("@for (item in items) { }");
        assert!(
            msg.contains("use `of` instead"),
            "expected hint about `of`, got: {msg}"
        );
    }

    #[test]
    fn for_wrong_of_keyword() {
        // "from" is a valid ident but not "of"
        let msg = parse_err("@for (item from items) { }");
        assert!(
            msg.contains("expected `of`"),
            "expected 'expected `of`', got: {msg}"
        );
    }

    #[test]
    fn element_no_parens() {
        let result = syn::parse_str::<Element>("@my_element");
        assert!(result.is_ok(), "@element without parens should succeed");
    }

    #[test]
    fn element_empty_parens() {
        let result = syn::parse_str::<Element>("@my_element()");
        assert!(result.is_ok(), "@element with empty parens should succeed");
    }

    #[test]
    fn throw_non_string_message() {
        let msg = parse_err("@throw 42");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }
}
