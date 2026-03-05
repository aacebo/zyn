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

fn has_input_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path().is_ident("zyn")
            && a.parse_args::<syn::Ident>()
                .map(|i| i == "input")
                .unwrap_or(false)
    })
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

                if has_input_attr(&pat_type.attrs) {
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

    let alias = custom_name.map(|lit| {
        let alias_name = zyn_core::syn::Ident::new(&pascal!(&lit.value()), lit.span());
        quote! { use #struct_name as #alias_name; }
    });

    let extractor_bindings: Vec<TokenStream> = extractor_names
        .iter()
        .zip(extractor_types.iter())
        .map(|(name, ty)| {
            quote! {
                let #name = match <#ty as ::zyn::FromInput>::from_input(input) {
                    ::std::result::Result::Ok(v) => v,
                    ::std::result::Result::Err(e) => {
                        let __err: ::zyn::syn::Error = ::std::convert::Into::into(e);
                        return __err.to_compile_error();
                    }
                };
            }
        })
        .collect();

    let prop_bindings: Vec<TokenStream> = prop_names
        .iter()
        .map(|name| quote! { let #name = &self.#name; })
        .collect();

    quote! {
        #vis struct #struct_name {
            #(pub #prop_names: #prop_types,)*
        }

        impl ::zyn::Render for #struct_name {
            fn render(&self, input: &::zyn::Input) -> ::zyn::proc_macro2::TokenStream {
                #(#extractor_bindings)*
                #(#prop_bindings)*
                #body
            }
        }

        #alias
    }
}
