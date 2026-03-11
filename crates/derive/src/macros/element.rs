use zyn_core::pascal;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn;
use zyn_core::syn::FnArg;
use zyn_core::syn::ItemFn;
use zyn_core::syn::ReturnType;
use zyn_core::syn::parse::Parse;
use zyn_core::syn::parse::ParseStream;
use zyn_core::syn::spanned::Spanned;

use crate::common::debug::DebugConfig;

struct ElementArgs {
    name: Option<syn::LitStr>,
    debug: Option<DebugConfig>,
}

impl Parse for ElementArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                name: None,
                debug: None,
            });
        }

        let mut name = None;
        let mut debug = None;

        if input.peek(syn::LitStr) {
            name = Some(input.parse()?);

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        if !input.is_empty() {
            debug = crate::common::debug::parse_debug_arg(input)?;
        }

        Ok(Self { name, debug })
    }
}

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let element_args = match syn::parse2::<ElementArgs>(args) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_element(item, element_args),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_element(item: ItemFn, args: ElementArgs) -> TokenStream {
    let vis = &item.vis;
    let body = &item.block;

    if matches!(item.sig.output, ReturnType::Default) {
        return zyn_core::syn::Error::new(
            item.sig.ident.span(),
            "element must return proc_macro2::TokenStream",
        )
        .to_compile_error();
    }

    let struct_name = pascal!(item.sig.ident => ident);
    let mut prop_names: Vec<syn::Ident> = Vec::new();
    let mut prop_types: Vec<syn::Type> = Vec::new();
    let mut extractor_names: Vec<syn::Ident> = Vec::new();
    let mut extractor_types: Vec<syn::Type> = Vec::new();

    for arg in &item.sig.inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let ident = match pat_type.pat.as_ref() {
                    zyn_core::syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                    _ => {
                        return zyn_core::syn::Error::new(
                            pat_type.pat.span(),
                            "element parameters must be simple identifiers",
                        )
                        .to_compile_error();
                    }
                };

                let ty = pat_type.ty.as_ref().clone();

                if crate::common::attrs::exists(&pat_type.attrs) {
                    extractor_names.push(ident.clone());
                    extractor_types.push(ty);
                } else {
                    prop_names.push(ident.clone());
                    prop_types.push(ty);
                }
            }
            FnArg::Receiver(r) => {
                return zyn_core::syn::Error::new(r.span(), "element parameters must be typed")
                    .to_compile_error();
            }
        }
    }

    let alias = args.name.map(|lit| {
        let alias_name = zyn_core::syn::Ident::new(&pascal!(&lit.value()), lit.span());
        quote! { use #struct_name as #alias_name; }
    });

    let diagnostic_macros = crate::common::diagnostics::macros();
    let input_expr = quote! { input };
    let extractor_bindings =
        crate::common::extractors::bindings(&extractor_names, &extractor_types, &input_expr);

    let prop_bindings: Vec<TokenStream> = prop_names
        .iter()
        .map(|name| quote! { let #name = &self.#name; })
        .collect();

    let output = quote! {
        #vis struct #struct_name {
            #(pub #prop_names: #prop_types,)*
        }

        impl ::zyn::Render for #struct_name {
            fn render(&self, input: &::zyn::Input) -> ::zyn::proc_macro2::TokenStream {
                let mut diagnostics = ::zyn::mark::new();

                #diagnostic_macros

                #(#extractor_bindings)*
                #(#prop_bindings)*
                let __body = #body;

                let diagnostics = diagnostics.build();
                if diagnostics.is_error() {
                    return diagnostics.emit();
                }

                __body
            }
        }

        #alias
    };

    if let Some(ref config) = args.debug {
        let ident = struct_name.to_string();

        if crate::common::debug::is_enabled(&ident) {
            crate::common::debug::emit(config, &format!("zyn::element ─── {ident}"), &output);
        }
    }

    output
}
