use zyn::quote::quote;

#[zyn::element(debug(pretty))]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}

#[test]
fn setter_generates_method() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("host"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("String").unwrap(),
        )
    );

    zyn::assert_tokens_contain!(output, "fn host");
    zyn::assert_tokens_contain!(output, "self . host = Some (value)");
}

#[test]
fn setter_generates_expected_signature() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("port"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("u16").unwrap(),
        )
    );
    let expected = quote! {
        fn port(mut self, value: u16) -> Self {
            self.port = Some(value);
            self
        }
    };

    zyn::assert_tokens!(output, expected);
}

#[test]
fn setter_pretty_output() {
    let input: zyn::Input = zyn::parse!("struct Foo;").unwrap();
    let output = zyn::zyn!(
        @setter(
            name = zyn::format_ident!("host"),
            ty = zyn::syn::parse_str::<zyn::syn::Type>("String").unwrap(),
        )
    );

    zyn::assert_tokens_contain_pretty!(output, "fn host(mut self, value: String) -> Self");
}
