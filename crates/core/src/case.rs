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
        let __tokens: Vec<proc_macro2::TokenTree> = $ts.clone().into_iter().collect();
        let mut __out = proc_macro2::TokenStream::new();

        for (i, __tt) in __tokens.iter().enumerate() {
            match __tt {
                proc_macro2::TokenTree::Ident(__ident) => {
                    let __is_last_ident = !__tokens[i + 1..]
                        .iter()
                        .any(|t| matches!(t, proc_macro2::TokenTree::Ident(_)));

                    if __is_last_ident {
                        quote::ToTokens::to_tokens(
                            &$crate::pascal!(__ident => ident),
                            &mut __out,
                        );
                    } else {
                        quote::ToTokens::to_tokens(__ident, &mut __out);
                    }
                }
                __other => {
                    quote::ToTokens::to_tokens(__other, &mut __out);
                }
            }
        }

        __out
    }};
    ($s:expr) => {
        $crate::case::to_pascal($s)
    };
}
