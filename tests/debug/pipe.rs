use zyn::quote::quote;

#[zyn::pipe(debug)]
fn shout_debug(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(
        &format!("{}_DEBUG", input.to_uppercase()),
        zyn::Span::call_site(),
    )
}

#[zyn::pipe("yell_debug", debug)]
fn make_loud_debug(input: String) -> zyn::syn::Ident {
    zyn::syn::Ident::new(
        &format!("{}__DEBUG", input.to_uppercase()),
        zyn::Span::call_site(),
    )
}

#[test]
fn pipe_with_debug() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | shout_debug } });
    let expected = quote!(HELLO_DEBUG);
    zyn::assert_tokens!(result, expected);
}

#[test]
fn pipe_with_debug_and_name() {
    let name = zyn::format_ident!("hello");
    let result = zyn::zyn!({ { name | yell_debug } });
    let expected = quote!(HELLO__DEBUG);
    zyn::assert_tokens!(result, expected);
}

#[cfg(feature = "pretty")]
mod pretty {
    use zyn::quote::quote;

    #[zyn::pipe(debug(pretty))]
    fn shout_pretty(input: String) -> zyn::syn::Ident {
        zyn::syn::Ident::new(
            &format!("{}_PRETTY", input.to_uppercase()),
            zyn::Span::call_site(),
        )
    }

    #[zyn::pipe("yell_pretty", debug(pretty))]
    fn make_loud_pretty(input: String) -> zyn::syn::Ident {
        zyn::syn::Ident::new(
            &format!("{}__PRETTY", input.to_uppercase()),
            zyn::Span::call_site(),
        )
    }

    #[test]
    fn pipe_with_pretty() {
        let name = zyn::format_ident!("hello");
        let result = zyn::zyn!({ { name | shout_pretty } });
        let expected = quote!(HELLO_PRETTY);
        zyn::assert_tokens!(result, expected);
    }

    #[test]
    fn pipe_with_pretty_and_name() {
        let name = zyn::format_ident!("hello");
        let result = zyn::zyn!({ { name | yell_pretty } });
        let expected = quote!(HELLO__PRETTY);
        zyn::assert_tokens!(result, expected);
    }
}

mod inject {
    use zyn::quote::quote;

    #[zyn::pipe(debug(input = "hello"))]
    fn shout_inject(input: String) -> zyn::syn::Ident {
        zyn::syn::Ident::new(&input.to_uppercase(), zyn::Span::call_site())
    }

    #[test]
    fn pipe_with_string_injection() {
        let name = zyn::format_ident!("hello");
        let result = zyn::zyn!({ { name | shout_inject } });
        let expected = quote!(HELLO);
        zyn::assert_tokens!(result, expected);
    }
}

mod full {
    use zyn::quote::quote;

    #[zyn::pipe(debug(full))]
    fn shout_full(input: String) -> zyn::syn::Ident {
        zyn::syn::Ident::new(
            &format!("{}_FULL", input.to_uppercase()),
            zyn::Span::call_site(),
        )
    }

    #[test]
    fn pipe_with_full() {
        let name = zyn::format_ident!("hello");
        let result = zyn::zyn!({ { name | shout_full } });
        let expected = quote!(HELLO_FULL);
        zyn::assert_tokens!(result, expected);
    }

    #[cfg(feature = "pretty")]
    mod pretty {
        use zyn::quote::quote;

        #[zyn::pipe(debug(pretty, full))]
        fn shout_full_pretty(input: String) -> zyn::syn::Ident {
            zyn::syn::Ident::new(
                &format!("{}_FULL_PRETTY", input.to_uppercase()),
                zyn::Span::call_site(),
            )
        }

        #[test]
        fn pipe_with_full_pretty() {
            let name = zyn::format_ident!("hello");
            let result = zyn::zyn!({ { name | shout_full_pretty } });
            let expected = quote!(HELLO_FULL_PRETTY);
            zyn::assert_tokens!(result, expected);
        }
    }
}
