mod folder;

use std::collections::HashSet;

use proc_macro::TokenStream;
use zyn::Args;
use zyn::Render;
use zyn::TokenStream as TokenStream2;
use zyn::types::Expr;
use zyn::types::Ident;
use zyn::types::ItemFn;
use zyn::types::Pat;

use folder::TraceVarFolder;

#[zyn::element]
fn assign_trace(left: Expr, op: TokenStream2, right: Expr) -> TokenStream2 {
    zyn::zyn!({
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
    })
}

#[zyn::element]
fn let_trace(pat: Pat, init: Expr, ident: Ident) -> TokenStream2 {
    zyn::zyn!(
        let {{ pat }} = {
            #[allow(unused_mut)]
            let {{ pat }} = {{ init }};
            ::std::println!(
                ::std::concat!(::std::stringify!({{ ident }}), " = {:?}"),
                {{ ident }},
            );
            {{ ident }}
        };
    )
}

#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    let ext_args = zyn::parse_input!(args as Args);
    let vars: HashSet<Ident> = ext_args.iter().filter_map(|a| a.name().cloned()).collect();
    let input = zyn::parse_input!(input as ItemFn);
    TraceVarFolder { input, vars }
        .render(&Default::default())
        .into()
}
