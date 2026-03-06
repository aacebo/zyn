use zyn_core::diagnostic::Diagnostic;
use zyn_core::diagnostic::Level;
use zyn_core::proc_macro2;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;

pub fn expand(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    let mode = parse_mode(&mut tokens);
    let body: TokenStream = tokens.collect();

    let element = match zyn_core::parse!(body => zyn_core::Template) {
        Ok(el) => el,
        Err(e) => return e.to_compile_error(),
    };

    match mode.as_str() {
        "raw" => {
            let expanded = element.to_token_stream();
            let pretty = zyn_core::debug::raw(&expanded);

            let _ = Diagnostic::spanned(
                proc_macro2::Span::call_site(),
                Level::Note,
                format!("zyn::debug! ─── raw\n\n{}", pretty),
            )
            .emit_as_item_tokens();

            expanded
        }
        "ast" => {
            let ast_str = zyn_core::debug::ast(&element);

            let _ = Diagnostic::spanned(
                proc_macro2::Span::call_site(),
                Level::Note,
                format!("zyn::debug! ─── ast\n\n{}", ast_str),
            )
            .emit_as_item_tokens();

            element.to_token_stream()
        }
        _ => {
            let expanded = element.to_token_stream();

            quote! {
                {
                    let __zyn_expand_result = #expanded;
                    ::zyn::debug::print(&__zyn_expand_result);
                    __zyn_expand_result
                }
            }
        }
    }
}

fn parse_mode(tokens: &mut std::iter::Peekable<proc_macro2::token_stream::IntoIter>) -> String {
    let mut fork = tokens.clone();

    if let Some(proc_macro2::TokenTree::Ident(ident)) = fork.next() {
        let mode = ident.to_string();

        if matches!(mode.as_str(), "pretty" | "raw" | "ast")
            && let Some(proc_macro2::TokenTree::Punct(p)) = fork.peek()
            && p.as_char() == '='
        {
            fork.next();

            if let Some(proc_macro2::TokenTree::Punct(p2)) = fork.peek()
                && p2.as_char() == '>'
            {
                fork.next();
                *tokens = fork;
                return mode;
            }
        }
    }

    "pretty".to_string()
}
