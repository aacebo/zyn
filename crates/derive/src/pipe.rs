use proc_macro2::TokenStream;
use quote::quote;
use syn::FnArg;
use syn::ItemFn;
use syn::ReturnType;
use syn::spanned::Spanned;
use zyn_core::pascal;

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let custom_name: Option<syn::LitStr> = if args.is_empty() {
        None
    } else {
        match syn::parse2(args) {
            Ok(lit) => Some(lit),
            Err(e) => return e.to_compile_error(),
        }
    };

    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_pipe(item, custom_name),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_pipe(item: ItemFn, custom_name: Option<syn::LitStr>) -> TokenStream {
    let vis = &item.vis;
    let body = &item.block;

    // Validate return type exists
    if matches!(item.sig.output, ReturnType::Default) {
        return syn::Error::new(
            item.sig.ident.span(),
            "pipe must have an explicit return type",
        )
        .to_compile_error();
    }

    // Validate at least one parameter
    if item.sig.inputs.is_empty() {
        return syn::Error::new(
            item.sig.ident.span(),
            "pipe must have at least one input parameter",
        )
        .to_compile_error();
    }

    // Convert snake_case function name to PascalCase struct name
    let struct_name = pascal!(item.sig.ident => ident);

    // Extract first parameter as pipe input
    let first_arg = &item.sig.inputs[0];
    let (input_name, input_type) = match first_arg {
        FnArg::Typed(pat_type) => {
            let ident = match pat_type.pat.as_ref() {
                syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    return syn::Error::new(
                        pat_type.pat.span(),
                        "pipe parameters must be simple identifiers",
                    )
                    .to_compile_error();
                }
            };
            (ident.clone(), pat_type.ty.as_ref().clone())
        }
        FnArg::Receiver(r) => {
            return syn::Error::new(r.span(), "pipe parameters must be typed").to_compile_error();
        }
    };

    let ret_type = match &item.sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => unreachable!(),
    };

    // Generate alias if custom name provided
    let alias = custom_name.map(|lit| {
        let alias_name = syn::Ident::new(&pascal!(&lit.value()), lit.span());
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
