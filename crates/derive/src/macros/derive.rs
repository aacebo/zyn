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

struct DeriveArgs {
    name: Option<syn::LitStr>,
    helpers: Vec<syn::Ident>,
    debug: Option<DebugConfig>,
}

impl Parse for DeriveArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                name: None,
                helpers: Vec::new(),
                debug: None,
            });
        }

        let name: syn::LitStr = input.parse()?;
        let mut helpers = Vec::new();
        let mut debug = None;

        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;

            if !input.is_empty() {
                if input.peek(syn::Ident) {
                    let fork = input.fork();
                    let kw: syn::Ident = fork.parse()?;

                    if kw == "attributes" {
                        input.parse::<syn::Ident>()?;

                        let content;
                        syn::parenthesized!(content in input);

                        while !content.is_empty() {
                            helpers.push(content.parse()?);

                            if content.peek(syn::Token![,]) {
                                content.parse::<syn::Token![,]>()?;
                            }
                        }

                        if input.peek(syn::Token![,]) {
                            input.parse::<syn::Token![,]>()?;
                        }
                    }
                }

                if !input.is_empty() {
                    debug = crate::common::debug::parse_debug_arg(input)?;
                }
            }
        }

        Ok(Self {
            name: Some(name),
            helpers,
            debug,
        })
    }
}

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_args = match syn::parse2::<DeriveArgs>(args) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_derive(item, derive_args),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_derive(item: ItemFn, args: DeriveArgs) -> TokenStream {
    let fn_name = &item.sig.ident;
    let body = &item.block;

    if matches!(item.sig.output, ReturnType::Default) {
        return syn::Error::new(
            item.sig.ident.span(),
            "derive macro must return proc_macro2::TokenStream",
        )
        .to_compile_error();
    }

    let derive_name = match &args.name {
        Some(lit) => syn::Ident::new(&lit.value(), lit.span()),
        None => pascal!(item.sig.ident => ident),
    };

    let helpers = &args.helpers;
    let derive_attr = if helpers.is_empty() {
        quote! { #[proc_macro_derive(#derive_name)] }
    } else {
        quote! { #[proc_macro_derive(#derive_name, attributes(#(#helpers),*))] }
    };

    let mut extractor_names: Vec<syn::Ident> = Vec::new();
    let mut extractor_types: Vec<syn::Type> = Vec::new();

    for arg in &item.sig.inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let ident = match pat_type.pat.as_ref() {
                    syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                    _ => {
                        return syn::Error::new(
                            pat_type.pat.span(),
                            "derive parameters must be simple identifiers",
                        )
                        .to_compile_error();
                    }
                };

                if !crate::common::attrs::exists(&pat_type.attrs) {
                    return syn::Error::new(
                        ident.span(),
                        "derive parameters must be marked with #[zyn(input)]",
                    )
                    .to_compile_error();
                }

                extractor_names.push(ident.clone());
                extractor_types.push(pat_type.ty.as_ref().clone());
            }
            FnArg::Receiver(r) => {
                return syn::Error::new(r.span(), "derive parameters must be typed")
                    .to_compile_error();
            }
        }
    }

    let diagnostic_macros = crate::common::diagnostics::macros();
    let input_expr = quote! { &::zyn::Input::from(input.clone()) };
    let extractor_bindings =
        crate::common::extractors::bindings(&extractor_names, &extractor_types, &input_expr);

    let output = quote! {
        #derive_attr
        pub fn #fn_name(__zyn_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let input = ::zyn::parse_input!(__zyn_input as ::zyn::syn::DeriveInput);

            let __zyn_result: ::zyn::proc_macro2::TokenStream = (|| {
                let mut diagnostics = ::zyn::Diagnostics::new();

                #diagnostic_macros

                #(#extractor_bindings)*

                let __body = #body;

                if diagnostics.has_errors() {
                    return diagnostics.emit();
                }

                __body
            })();

            __zyn_result.into()
        }
    };

    if let Some(ref config) = args.debug {
        let ident = derive_name.to_string();

        if crate::common::debug::is_enabled(&ident) {
            crate::common::debug::emit(config, &format!("zyn::derive ─── {ident}"), &output);
        }
    }

    output
}
