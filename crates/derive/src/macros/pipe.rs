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

struct PipeArgs {
    name: Option<syn::LitStr>,
    debug: Option<DebugConfig>,
}

impl Parse for PipeArgs {
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
    let pipe_args = match syn::parse2::<PipeArgs>(args) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_pipe(item, pipe_args),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_pipe(item: ItemFn, args: PipeArgs) -> TokenStream {
    let vis = &item.vis;
    let body = &item.block;

    if matches!(item.sig.output, ReturnType::Default) {
        return zyn_core::syn::Error::new(
            item.sig.ident.span(),
            "pipe must have an explicit return type",
        )
        .to_compile_error();
    }

    if item.sig.inputs.is_empty() {
        return zyn_core::syn::Error::new(
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
                zyn_core::syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    return zyn_core::syn::Error::new(
                        pat_type.pat.span(),
                        "pipe parameters must be simple identifiers",
                    )
                    .to_compile_error();
                }
            };
            (ident.clone(), pat_type.ty.as_ref().clone())
        }
        FnArg::Receiver(r) => {
            return zyn_core::syn::Error::new(r.span(), "pipe parameters must be typed")
                .to_compile_error();
        }
    };

    let ret_type = match &item.sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => unreachable!(),
    };

    let alias = args.name.map(|lit| {
        let alias_name = zyn_core::syn::Ident::new(&pascal!(&lit.value()), lit.span());
        quote! { use #struct_name as #alias_name; }
    });

    let output = quote! {
        #vis struct #struct_name;

        impl ::zyn::Pipe for #struct_name {
            type Input = #input_type;
            type Output = #ret_type;

            fn pipe(&self, #input_name: #input_type) -> #ret_type
                #body
        }

        #alias
    };

    if let Some(ref config) = args.debug {
        let ident = struct_name.to_string();

        if crate::common::debug::is_enabled(&ident) {
            crate::common::debug::emit(config, &format!("zyn::pipe ─── {ident}"), &output);
        }
    }

    output
}
