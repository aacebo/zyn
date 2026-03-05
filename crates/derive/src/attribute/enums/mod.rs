mod variant_meta;

use variant_meta::VariantMeta;

use zyn_core::diagnostic::ToDiagnostics;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let variants = match VariantMeta::parse(&input) {
        Ok(v) => v,
        Err(e) => return e.to_diagnostics().emit(),
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let all_names: Vec<&str> = variants.iter().map(|v| v.snake_name.as_str()).collect();
    let expected = all_names.join(", ");

    let flag_arms: Vec<TokenStream> = variants.iter().filter_map(|v| v.arm_from_flag()).collect();
    let list_arms: Vec<TokenStream> = variants.iter().filter_map(|v| v.arm_from_list()).collect();
    let expr_arms: Vec<TokenStream> = variants.iter().filter_map(|v| v.arm_from_expr()).collect();

    let flag_block = if flag_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            ::zyn::Arg::Flag(ident) => match ident.to_string().as_str() {
                #(#flag_arms,)*
                other => ::std::result::Result::Err(::zyn::Diagnostics::error(
                    ident.span(),
                    ::std::format!("unknown variant `{}`, expected one of: {}", other, #expected),
                )),
            },
        }
    };

    let list_block = if list_arms.is_empty() {
        quote! {
            ::zyn::Arg::List(_, args) if args.len() == 1 => Self::from_arg(&args[0]),
        }
    } else {
        quote! {
            ::zyn::Arg::List(ident, args) => match ident.to_string().as_str() {
                #(#list_arms,)*
                _ if args.len() == 1 => Self::from_arg(&args[0]),
                other => ::std::result::Result::Err(::zyn::Diagnostics::error(
                    ident.span(),
                    ::std::format!("unknown variant `{}`, expected one of: {}", other, #expected),
                )),
            },
        }
    };

    let expr_block = if expr_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            ::zyn::Arg::Expr(ident, _) => match ident.to_string().as_str() {
                #(#expr_arms,)*
                other => ::std::result::Result::Err(::zyn::Diagnostics::error(
                    ident.span(),
                    ::std::format!("unknown variant `{}`, expected one of: {}", other, #expected),
                )),
            },
        }
    };

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn from_arg(arg: &::zyn::Arg) -> ::zyn::Result<Self> {
                match arg {
                    #flag_block
                    #list_block
                    #expr_block
                    _ => ::std::result::Result::Err(::zyn::Diagnostics::error(
                        ::zyn::proc_macro2::Span::call_site(),
                        ::std::format!("expected one of: {}", #expected),
                    )),
                }
            }
        }

        impl #impl_generics ::zyn::FromArg for #name #ty_generics #where_clause {
            fn from_arg(arg: &::zyn::Arg) -> ::zyn::Result<Self> {
                Self::from_arg(arg)
            }
        }
    }
}
