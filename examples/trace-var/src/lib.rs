mod folder;

use std::collections::HashSet;

use proc_macro::TokenStream;
use zyn::ext::Args;
use zyn::proc_macro2::Ident;
use zyn::proc_macro2::TokenStream as TokenStream2;
use zyn::syn::Expr;
use zyn::syn::Pat;
use zyn::ItemFn;
use zyn::Render;
use zyn::syn::parse_macro_input;

use folder::TraceVarFolder;

#[zyn::element]
fn assign_trace(left: Expr, op: TokenStream2, right: Expr) -> zyn::syn::Result<TokenStream2> {
    Ok(zyn::zyn!({
        {
            { left }
        }
        {
            { op }
        }
        {
            { right }
        };
        ::std::println!(
            ::std::concat!(::std::stringify!({ { left } }), " = {:?}"),
            { { left } },
        );
    }))
}

#[zyn::element]
fn let_trace(pat: Pat, init: Expr, ident: Ident) -> zyn::syn::Result<TokenStream2> {
    Ok(zyn::zyn!(
        let {{ pat }} = {
            #[allow(unused_mut)]
            let {{ pat }} = {{ init }};
            ::std::println!(
                ::std::concat!(::std::stringify!({{ ident }}), " = {:?}"),
                {{ ident }},
            );
            {{ ident }}
        };
    ))
}


#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    let ext_args = parse_macro_input!(args as Args);
    let vars: HashSet<Ident> = ext_args.iter().filter_map(|a| a.name().cloned()).collect();
    let input = parse_macro_input!(input as ItemFn);
    TraceVarFolder { input, vars }.render().unwrap().into()
}
