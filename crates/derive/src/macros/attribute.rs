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

struct AttributeArgs {
    debug: Option<DebugConfig>,
}

impl Parse for AttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { debug: None });
        }

        let debug = crate::common::debug::parse_debug_arg(input)?;
        Ok(Self { debug })
    }
}

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match syn::parse2::<AttributeArgs>(args) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_attribute(item, attr_args),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_attribute(item: ItemFn, args: AttributeArgs) -> TokenStream {
    let fn_name = &item.sig.ident;
    let body = &item.block;

    if matches!(item.sig.output, ReturnType::Default) {
        return syn::Error::new(
            item.sig.ident.span(),
            "attribute macro must return proc_macro2::TokenStream",
        )
        .to_compile_error();
    }

    let mut extractor_names: Vec<syn::Ident> = Vec::new();
    let mut extractor_types: Vec<syn::Type> = Vec::new();
    let mut args_binding: Option<TokenStream> = None;

    for arg in &item.sig.inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let ident = match pat_type.pat.as_ref() {
                    syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                    _ => {
                        return syn::Error::new(
                            pat_type.pat.span(),
                            "attribute parameters must be simple identifiers",
                        )
                        .to_compile_error();
                    }
                };

                let ty = pat_type.ty.as_ref().clone();

                if crate::common::attrs::exists(&pat_type.attrs) {
                    extractor_names.push(ident.clone());
                    extractor_types.push(ty);
                } else {
                    if args_binding.is_some() {
                        return syn::Error::new(
                            ident.span(),
                            "attribute macro can have at most one args parameter",
                        )
                        .to_compile_error();
                    }

                    args_binding = Some(quote! {
                        let #ident: #ty = ::zyn::parse_input!(__zyn_args as #ty);
                    });
                }
            }
            FnArg::Receiver(r) => {
                return syn::Error::new(r.span(), "attribute parameters must be typed")
                    .to_compile_error();
            }
        }
    }

    let diagnostic_macros = crate::common::diagnostics::macros();
    let input_expr = quote! { &::zyn::Input::from(input.clone()) };
    let extractor_bindings =
        crate::common::extractors::bindings(&extractor_names, &extractor_types, &input_expr);

    let output = quote! {
        #[proc_macro_attribute]
        pub fn #fn_name(
            __zyn_args: proc_macro::TokenStream,
            __zyn_input: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            let input = ::zyn::parse_input!(__zyn_input as ::zyn::syn::Item);
            #args_binding

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
        let ident = fn_name.to_string();

        if crate::common::debug::is_enabled(&ident) {
            crate::common::debug::emit(config, &format!("zyn::attribute ─── {ident}"), &output);
        }
    }

    output
}
