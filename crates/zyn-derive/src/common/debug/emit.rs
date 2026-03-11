use zyn_core::mark;
use zyn_core::proc_macro2;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::proc_macro2::TokenTree;

use super::config::DebugConfig;
use super::config::DebugFormat;

pub fn emit(config: &DebugConfig, label: &str, tokens: &TokenStream) {
    let cleaned = strip_noise(tokens);

    let text = match config.format {
        DebugFormat::Raw => cleaned.to_string(),
        #[cfg(feature = "pretty")]
        DebugFormat::Pretty => {
            use zyn_core::debug::DebugExt;
            cleaned.debug().pretty()
        }
    };

    let _ = mark::note(format!("{label}\n\n{text}"))
        .span(proc_macro2::Span::call_site())
        .build()
        .emit_as_item_tokens();
}

fn strip_noise(tokens: &TokenStream) -> TokenStream {
    let mut result = Vec::new();
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                if let Some(TokenTree::Group(g)) = iter.peek()
                    && is_noise_attr(g)
                {
                    iter.next();
                    continue;
                }

                result.push(tt);
            }
            TokenTree::Ident(ident) if *ident == "macro_rules" => {
                if skip_macro_rules(&mut iter) {
                    continue;
                }

                result.push(tt);
            }
            TokenTree::Group(g) => {
                let cleaned = strip_noise(&g.stream());
                let mut new_group = proc_macro2::Group::new(g.delimiter(), cleaned);
                new_group.set_span(g.span());
                result.push(TokenTree::Group(new_group));
            }
            _ => result.push(tt),
        }
    }

    result.into_iter().collect()
}

fn is_noise_attr(group: &proc_macro2::Group) -> bool {
    if group.delimiter() != proc_macro2::Delimiter::Bracket {
        return false;
    }

    let s = group.stream().to_string();
    s.starts_with("doc =") || s.starts_with("allow")
}

fn skip_macro_rules(iter: &mut std::iter::Peekable<proc_macro2::token_stream::IntoIter>) -> bool {
    if let Some(bang) = iter.peek()
        && let TokenTree::Punct(p) = bang
        && p.as_char() == '!'
    {
        iter.next();

        if let Some(TokenTree::Ident(name)) = iter.peek() {
            let name_str = name.to_string();

            if matches!(
                name_str.as_str(),
                "error" | "warn" | "note" | "help" | "bail"
            ) {
                iter.next();

                if let Some(TokenTree::Group(_)) = iter.peek() {
                    iter.next();
                    return true;
                }
            }
        }
    }

    false
}
