pub fn to_pascal(s: &str) -> String {
    let mut out = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(c.to_uppercase());
            capitalize = false;
        } else {
            out.push(c);
        }
    }

    out
}

#[macro_export]
macro_rules! pascal {
    ($ident:expr => ident) => {
        syn::Ident::new(
            &$crate::case::to_pascal(&$ident.to_string()),
            $ident.span(),
        )
    };
    ($ts:expr => token_stream) => {{
        let mut __out = proc_macro2::TokenStream::new();

        for __tt in $ts.clone() {
            match __tt {
                proc_macro2::TokenTree::Ident(__ident) => {
                    quote::ToTokens::to_tokens(
                        &$crate::pascal!(__ident => ident),
                        &mut __out,
                    );
                }
                __other => {
                    quote::ToTokens::to_tokens(&__other, &mut __out);
                }
            }
        }

        __out
    }};
    ($s:expr) => {
        $crate::case::to_pascal($s)
    };
}
