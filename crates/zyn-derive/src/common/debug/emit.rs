use zyn_core::Template;
use zyn_core::mark;
use zyn_core::proc_macro2;
use zyn_core::proc_macro2::Spacing;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::proc_macro2::TokenTree;
use zyn_core::syn;

use super::config::DebugConfig;
use super::config::DebugFormat;

pub fn emit(
    config: &DebugConfig,
    label: &str,
    tokens: &TokenStream,
    injections: &[(String, TokenStream)],
) {
    let expanded = expand_macros(tokens, injections);
    let cleaned = strip_noise(&expanded);

    let text = match config.format {
        DebugFormat::Raw => normalize_placeholders(cleaned.to_string()),
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

pub fn emit_debug(
    config: &DebugConfig,
    label: &str,
    name: &str,
    output: &TokenStream,
    body: &syn::Block,
) {
    if !super::pattern::is_enabled(name) {
        return;
    }

    let tokens = if config.full {
        output.clone()
    } else {
        let stmts = &body.stmts;
        zyn_core::quote::quote! { #(#stmts)* }
    };

    let coerced: Vec<(String, TokenStream)> = config
        .injections
        .iter()
        .filter_map(|(key, expr)| {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = expr
            {
                let ts: TokenStream = syn::parse_str(&s.value()).ok()?;
                Some((key.clone(), ts))
            } else {
                None
            }
        })
        .collect();

    emit(config, &format!("{label} ─── {name}"), &tokens, &coerced);
}

fn expand_macros(tokens: &TokenStream, injections: &[(String, TokenStream)]) -> TokenStream {
    expand_macros_inner(tokens, 0, injections)
}

fn expand_macros_inner(
    tokens: &TokenStream,
    depth: u32,
    injections: &[(String, TokenStream)],
) -> TokenStream {
    const MAX_DEPTH: u32 = 16;

    if depth >= MAX_DEPTH {
        return tokens.clone();
    }

    let mut result = Vec::<TokenTree>::new();
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Ident(id) if id == "zyn" => {
                expand_zyn(tt, &mut iter, &mut result, depth, injections);
            }
            TokenTree::Ident(id) if id == "quote" => {
                expand_quote(tt, &mut iter, &mut result, depth, injections);
            }
            TokenTree::Group(g) => {
                let inner = expand_macros_inner(&g.stream(), depth + 1, injections);
                let mut new_g = proc_macro2::Group::new(g.delimiter(), inner);
                new_g.set_span(g.span());
                result.push(TokenTree::Group(new_g));
            }
            _ => result.push(tt),
        }
    }

    result.into_iter().collect()
}

type TokenIter = std::iter::Peekable<proc_macro2::token_stream::IntoIter>;

fn expand_zyn(
    first: TokenTree,
    iter: &mut TokenIter,
    result: &mut Vec<TokenTree>,
    depth: u32,
    injections: &[(String, TokenStream)],
) {
    let mut candidate = vec![first];

    // Try: zyn ! (...)
    if let Some(tt) = iter.peek()
        && is_bang(tt)
    {
        candidate.push(iter.next().unwrap());

        if let Some(TokenTree::Group(g)) = iter.peek()
            && is_zyn_delimiter(g.delimiter())
        {
            let g_tt = iter.next().unwrap();
            let inner = group_stream(&g_tt);

            if let Ok(tmpl) = syn::parse2::<Template>(inner) {
                let display =
                    expand_macros_inner(&tmpl.to_display_stream(injections), depth + 1, injections);
                result.extend(display);
                return;
            }

            candidate.push(g_tt);
        }

        result.extend(candidate);
        return;
    }

    // Try: zyn :: zyn ! (...)
    if let Some(tt) = iter.peek()
        && is_colon_joint(tt)
    {
        candidate.push(iter.next().unwrap()); // first ':'

        if let Some(tt) = iter.peek()
            && is_colon_alone(tt)
        {
            candidate.push(iter.next().unwrap()); // second ':'

            if let Some(TokenTree::Ident(id)) = iter.peek()
                && id == "zyn"
            {
                candidate.push(iter.next().unwrap()); // second "zyn"

                if let Some(tt) = iter.peek()
                    && is_bang(tt)
                {
                    candidate.push(iter.next().unwrap()); // "!"

                    if let Some(TokenTree::Group(g)) = iter.peek()
                        && is_zyn_delimiter(g.delimiter())
                    {
                        let g_tt = iter.next().unwrap();
                        let inner = group_stream(&g_tt);

                        if let Ok(tmpl) = syn::parse2::<Template>(inner) {
                            let display = expand_macros_inner(
                                &tmpl.to_display_stream(injections),
                                depth + 1,
                                injections,
                            );
                            result.extend(display);
                            return;
                        }

                        candidate.push(g_tt);
                    }
                }
            }
        }
    }

    result.extend(candidate);
}

fn expand_quote(
    first: TokenTree,
    iter: &mut TokenIter,
    result: &mut Vec<TokenTree>,
    depth: u32,
    injections: &[(String, TokenStream)],
) {
    let mut candidate = vec![first];

    if let Some(tt) = iter.peek()
        && is_bang(tt)
    {
        candidate.push(iter.next().unwrap());

        if let Some(TokenTree::Group(_)) = iter.peek() {
            let g_tt = iter.next().unwrap();
            let inner = group_stream(&g_tt);
            let expanded = expand_macros_inner(&inner, depth + 1, injections);
            result.extend(expanded);
            return;
        }
    }

    result.extend(candidate);
}

fn is_zyn_delimiter(d: proc_macro2::Delimiter) -> bool {
    matches!(
        d,
        proc_macro2::Delimiter::Parenthesis | proc_macro2::Delimiter::Brace
    )
}

fn is_bang(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if p.as_char() == '!')
}

fn is_colon_joint(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if p.as_char() == ':' && p.spacing() == Spacing::Joint)
}

fn is_colon_alone(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if p.as_char() == ':' && p.spacing() == Spacing::Alone)
}

fn group_stream(tt: &TokenTree) -> TokenStream {
    match tt {
        TokenTree::Group(g) => g.stream(),
        _ => unreachable!(),
    }
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

/// Normalize `TokenStream::to_string()` spacing for human-readable debug output.
///
/// `proc_macro2` emits spaces between every token pair. This collapses the most
/// common cases: placeholder braces, generics, references, and macro invocations.
fn normalize_placeholders(s: String) -> String {
    // Placeholder braces: `{ { expr } }` → `{{ expr }}`
    let s = s.replace("{ {", "{{ ");
    let s = s.replace("} }", " }}");
    // Generics: `Vec < u8 >` → `Vec<u8>`
    let s = s.replace(" < ", "<");
    let s = s.replace(" >", ">");
    // References: `& self`, `& str`, `& mut` → `&self`, `&str`, `&mut`
    let s = s.replace("& ", "&");
    // Macro calls: `todo! ()` → `todo!()`
    s.replace("! (", "!(")
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
