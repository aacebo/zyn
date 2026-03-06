use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn;
use zyn_core::syn::FnArg;
use zyn_core::syn::ItemFn;
use zyn_core::syn::ReturnType;
use zyn_core::syn::spanned::Spanned;

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_attribute(item),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_attribute(item: ItemFn) -> TokenStream {
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
    let extractor_bindings =
        crate::common::extractors::bindings(&extractor_names, &extractor_types);

    quote! {
        #[proc_macro_attribute]
        pub fn #fn_name(
            __zyn_args: proc_macro::TokenStream,
            __zyn_input: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            let input: ::zyn::Input = ::zyn::Input::Item(
                ::zyn::parse_input!(__zyn_input as ::zyn::syn::Item)
            );
            let mut diagnostics = ::zyn::Diagnostics::new();

            #diagnostic_macros

            #(#extractor_bindings)*
            #args_binding

            let __body = #body;
            if diagnostics.has_errors() {
                return diagnostics.emit().into();
            }
            __body.into()
        }
    }
}
