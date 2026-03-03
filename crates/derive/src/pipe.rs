use proc_macro2::TokenStream;
use quote::quote;
use syn::FnArg;
use syn::ItemFn;
use syn::ReturnType;
use syn::spanned::Spanned;

pub fn expand(input: TokenStream) -> TokenStream {
    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_pipe(item),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_pipe(item: ItemFn) -> TokenStream {
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
    let struct_name = to_pascal_case_ident(&item.sig.ident);

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

    quote! {
        #vis struct #struct_name;

        impl ::zyn::Pipe for #struct_name {
            type Input = #input_type;
            type Output = #ret_type;

            fn pipe(&self, #input_name: #input_type) -> #ret_type
                #body
        }
    }
}

fn to_pascal_case_ident(ident: &syn::Ident) -> syn::Ident {
    let s = ident.to_string();
    let mut out = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(c.to_uppercase());
            capitalize = false;
        } else {
            out.push(c);
        }
    }

    syn::Ident::new(&out, ident.span())
}
