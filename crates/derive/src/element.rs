use zyn_core::pascal;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn;
use zyn_core::syn::FnArg;
use zyn_core::syn::ItemFn;
use zyn_core::syn::ReturnType;
use zyn_core::syn::spanned::Spanned;

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let custom_name: Option<zyn_core::syn::LitStr> = if args.is_empty() {
        None
    } else {
        match zyn_core::syn::parse2(args) {
            Ok(lit) => Some(lit),
            Err(e) => return e.to_compile_error(),
        }
    };

    match zyn_core::syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_element(item, custom_name),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_element(item: ItemFn, custom_name: Option<zyn_core::syn::LitStr>) -> TokenStream {
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

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

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

                field_names.push(ident.clone());
                field_types.push(pat_type.ty.as_ref().clone());
            }
            FnArg::Receiver(r) => {
                return zyn_core::syn::Error::new(r.span(), "element parameters must be typed")
                    .to_compile_error();
            }
        }
    }

    let alias = custom_name.map(|lit| {
        let alias_name = zyn_core::syn::Ident::new(&pascal!(&lit.value()), lit.span());
        quote! { use #struct_name as #alias_name; }
    });

    quote! {
        #vis struct #struct_name {
            #(pub #field_names: #field_types,)*
        }

        impl ::zyn::Render for #struct_name {
            fn render(&self) -> ::zyn::proc_macro2::TokenStream {
                #(let #field_names = &self.#field_names;)*
                #body
            }
        }

        #alias
    }
}
