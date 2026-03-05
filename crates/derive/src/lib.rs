mod attribute;
mod element;
mod pipe;
mod prettify;

use zyn_core::__private::proc_macro2;
use zyn_core::__private::quote::quote;
use zyn_core::diagnostic::Diagnostic;
use zyn_core::diagnostic::Level;

#[proc_macro]
pub fn zyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_template(input.into()).into()
}

#[proc_macro]
pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_debug(input.into()).into()
}

#[proc_macro_attribute]
pub fn element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    element::expand(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn pipe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    pipe::expand(args.into(), input.into()).into()
}

#[proc_macro_derive(Attribute, attributes(zyn))]
pub fn derive_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute::expand(input.into()).into()
}

fn expand_template(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match zyn_core::parse!(input => zyn_core::ast::Element) {
        Ok(element) => {
            let expanded = element.to_token_stream();
            quote! {
                {
                    #[allow(unused_variables)]
                    let input: ::zyn::Input = ::std::default::Default::default();
                    #expanded
                }
            }
        }
        Err(e) => e.to_compile_error(),
    }
}

fn expand_debug(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut tokens = input.into_iter().peekable();

    let mode = parse_mode(&mut tokens);
    let body: proc_macro2::TokenStream = tokens.collect();

    let element = match zyn_core::parse!(body => zyn_core::ast::Element) {
        Ok(el) => el,
        Err(e) => return e.to_compile_error(),
    };

    match mode.as_str() {
        "raw" => {
            let expanded = element.to_token_stream();
            let pretty = prettify::prettify_raw(&expanded);

            let _ = Diagnostic::spanned(
                proc_macro2::Span::call_site(),
                Level::Note,
                format!("zyn::expand! ─── raw\n\n{}", pretty),
            )
            .emit_as_item_tokens();

            expanded
        }
        "ast" => {
            let ast_str = prettify::prettify_ast(&element);

            let _ = Diagnostic::spanned(
                proc_macro2::Span::call_site(),
                Level::Note,
                format!("zyn::expand! ─── ast\n\n{}", ast_str),
            )
            .emit_as_item_tokens();

            element.to_token_stream()
        }
        _ => {
            let expanded = element.to_token_stream();

            quote! {
                {
                    let __zyn_expand_result = #expanded;
                    ::zyn::debug::print_pretty(&__zyn_expand_result);
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
