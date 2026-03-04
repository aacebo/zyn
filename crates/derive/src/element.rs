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
        Ok(item) => expand_element(item, custom_name),
        Err(e) => e.to_compile_error(),
    }
}

fn is_result(ret: &ReturnType) -> bool {
    let ReturnType::Type(_, ty) = ret else {
        return false;
    };
    let syn::Type::Path(path) = ty.as_ref() else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|s| s.ident == "Result")
}

fn expand_element(item: ItemFn, custom_name: Option<syn::LitStr>) -> TokenStream {
    let vis = &item.vis;
    let body = &item.block;

    if matches!(item.sig.output, ReturnType::Default) {
        return syn::Error::new(
            item.sig.ident.span(),
            "element must return TokenStream, diagnostic::Result, or syn::Result<TokenStream>",
        )
        .to_compile_error();
    }

    let struct_name = pascal!(item.sig.ident => ident);

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

    for arg in &item.sig.inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let ident = match pat_type.pat.as_ref() {
                    syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                    _ => {
                        return syn::Error::new(
                            pat_type.pat.span(),
                            "element parameters must be simple identifiers",
                        )
                        .to_compile_error();
                    }
                };

                field_names.push(ident.clone());
                field_types.push(pat_type.ty.as_ref().clone());
            }
            FnArg::Receiver(r) => {
                return syn::Error::new(r.span(), "element parameters must be typed")
                    .to_compile_error();
            }
        }
    }

    let alias = custom_name.map(|lit| {
        let alias_name = syn::Ident::new(&pascal!(&lit.value()), lit.span());
        quote! { use #struct_name as #alias_name; }
    });

    let render_body = if is_result(&item.sig.output) {
        quote! {
            #(let #field_names = &self.#field_names;)*
            (#body).map_err(::zyn::Diagnostic::from)
        }
    } else {
        quote! {
            #(let #field_names = &self.#field_names;)*
            ::core::result::Result::Ok(#body)
        }
    };

    quote! {
        #vis struct #struct_name {
            #(pub #field_names: #field_types,)*
        }

        impl ::zyn::Render for #struct_name {
            fn render(&self) -> ::zyn::Result {
                #render_body
            }
        }

        #alias
    }
}
