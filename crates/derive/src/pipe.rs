use zyn_core::__private::proc_macro2::TokenStream;
use zyn_core::__private::quote::quote;
use zyn_core::__private::syn;
use zyn_core::__private::syn::FnArg;
use zyn_core::__private::syn::ReturnType;
use zyn_core::__private::syn::spanned::Spanned;
use zyn_core::pascal;
use zyn_core::types::ItemFn;

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let custom_name: Option<zyn_core::__private::syn::LitStr> = if args.is_empty() {
        None
    } else {
        match zyn_core::__private::syn::parse2(args) {
            Ok(lit) => Some(lit),
            Err(e) => return e.to_compile_error(),
        }
    };

    match zyn_core::__private::syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_pipe(item, custom_name),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_pipe(item: ItemFn, custom_name: Option<zyn_core::__private::syn::LitStr>) -> TokenStream {
    let vis = &item.vis;
    let body = &item.block;

    if matches!(item.sig.output, ReturnType::Default) {
        return zyn_core::__private::syn::Error::new(
            item.sig.ident.span(),
            "pipe must have an explicit return type",
        )
        .to_compile_error();
    }

    if item.sig.inputs.is_empty() {
        return zyn_core::__private::syn::Error::new(
            item.sig.ident.span(),
            "pipe must have at least one input parameter",
        )
        .to_compile_error();
    }

    let struct_name = pascal!(item.sig.ident => ident);

    let first_arg = &item.sig.inputs[0];
    let (input_name, input_type) = match first_arg {
        FnArg::Typed(pat_type) => {
            let ident = match pat_type.pat.as_ref() {
                zyn_core::__private::syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    return zyn_core::__private::syn::Error::new(
                        pat_type.pat.span(),
                        "pipe parameters must be simple identifiers",
                    )
                    .to_compile_error();
                }
            };
            (ident.clone(), pat_type.ty.as_ref().clone())
        }
        FnArg::Receiver(r) => {
            return zyn_core::__private::syn::Error::new(r.span(), "pipe parameters must be typed")
                .to_compile_error();
        }
    };

    let ret_type = match &item.sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => unreachable!(),
    };

    let alias = custom_name.map(|lit| {
        let alias_name = zyn_core::__private::syn::Ident::new(&pascal!(&lit.value()), lit.span());
        quote! { use #struct_name as #alias_name; }
    });

    quote! {
        #vis struct #struct_name;

        impl ::zyn::Pipe for #struct_name {
            type Input = #input_type;
            type Output = #ret_type;

            fn pipe(&self, #input_name: #input_type) -> #ret_type
                #body
        }

        #alias
    }
}
