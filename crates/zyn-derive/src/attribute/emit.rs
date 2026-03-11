use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;
use zyn_core::syn;

use super::structs::FieldDefault;
use super::structs::FieldKey;
use super::structs::FieldMeta;
use super::structs::StructMeta;

pub fn from_args(
    name: &syn::Ident,
    fields: &[FieldMeta],
    impl_generics: &syn::ImplGenerics<'_>,
    ty_generics: &syn::TypeGenerics<'_>,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    let mut let_bindings: Vec<TokenStream> = Vec::new();
    let mut struct_inits: Vec<TokenStream> = Vec::new();
    let mut known_keys: Vec<String> = Vec::new();

    for f in fields {
        let ident = &f.ident;
        let ty = &f.ty;
        let field_ident = syn::Ident::new(&format!("__f_{ident}"), ident.span());

        if f.skip {
            struct_inits.push(quote! { #ident: ::std::default::Default::default() });
            continue;
        }

        match &f.key {
            FieldKey::Positional(idx) => {
                let_bindings.push(quote! {
                    let #field_ident: ::std::option::Option<#ty> = match args.get_index(#idx) {
                        ::std::option::Option::Some(arg) => {
                            match <#ty as ::zyn::FromArg>::from_arg(arg) {
                                ::std::result::Result::Ok(v) => ::std::option::Option::Some(v),
                                ::std::result::Result::Err(e) => {
                                    __diags = __diags.add(e);
                                    ::std::option::Option::None
                                }
                            }
                        }
                        ::std::option::Option::None => {
                            __diags = __diags.add(
                                ::zyn::mark::error(::std::format!("missing required positional argument [{}]", #idx))
                            );
                            ::std::option::Option::None
                        }
                    };
                });

                struct_inits.push(quote! { #ident: #field_ident.unwrap() });
            }
            FieldKey::Named(key) => {
                known_keys.push(key.clone());

                if f.is_bool() {
                    struct_inits.push(quote! { #ident: args.has(#key) });
                    continue;
                }

                if let Some(inner_ty) = f.option_inner() {
                    let_bindings.push(quote! {
                        let #field_ident: ::std::option::Option<::std::option::Option<#inner_ty>> = match args.get(#key) {
                            ::std::option::Option::Some(arg) => {
                                match <#inner_ty as ::zyn::FromArg>::from_arg(arg) {
                                    ::std::result::Result::Ok(v) => ::std::option::Option::Some(::std::option::Option::Some(v)),
                                    ::std::result::Result::Err(e) => {
                                        __diags = __diags.add(e);
                                        ::std::option::Option::None
                                    }
                                }
                            }
                            ::std::option::Option::None => ::std::option::Option::Some(::std::option::Option::None),
                        };
                    });

                    struct_inits.push(quote! { #ident: #field_ident.unwrap() });
                    continue;
                }

                let missing_msg = build_missing_msg(key, f);

                match &f.default {
                    FieldDefault::None => {
                        let_bindings.push(quote! {
                            let #field_ident: ::std::option::Option<#ty> = match args.get(#key) {
                                ::std::option::Option::Some(arg) => {
                                    match <#ty as ::zyn::FromArg>::from_arg(arg) {
                                        ::std::result::Result::Ok(v) => ::std::option::Option::Some(v),
                                        ::std::result::Result::Err(e) => {
                                            __diags = __diags.add(e);
                                            ::std::option::Option::None
                                        }
                                    }
                                }
                                ::std::option::Option::None => {
                                    __diags = __diags.add(
                                        ::zyn::mark::error(#missing_msg)
                                    );
                                    ::std::option::Option::None
                                }
                            };
                        });
                    }
                    FieldDefault::Unit => {
                        let_bindings.push(quote! {
                            let #field_ident: ::std::option::Option<#ty> = match args.get(#key) {
                                ::std::option::Option::Some(arg) => {
                                    match <#ty as ::zyn::FromArg>::from_arg(arg) {
                                        ::std::result::Result::Ok(v) => ::std::option::Option::Some(v),
                                        ::std::result::Result::Err(e) => {
                                            __diags = __diags.add(e);
                                            ::std::option::Option::None
                                        }
                                    }
                                }
                                ::std::option::Option::None => ::std::option::Option::Some(::std::default::Default::default()),
                            };
                        });
                    }
                    FieldDefault::Expr(expr) => {
                        let_bindings.push(quote! {
                            let #field_ident: ::std::option::Option<#ty> = match args.get(#key) {
                                ::std::option::Option::Some(arg) => {
                                    match <#ty as ::zyn::FromArg>::from_arg(arg) {
                                        ::std::result::Result::Ok(v) => ::std::option::Option::Some(v),
                                        ::std::result::Result::Err(e) => {
                                            __diags = __diags.add(e);
                                            ::std::option::Option::None
                                        }
                                    }
                                }
                                ::std::option::Option::None => ::std::option::Option::Some(::std::convert::Into::into(#expr)),
                            };
                        });
                    }
                }

                struct_inits.push(quote! { #ident: #field_ident.unwrap() });
            }
        }
    }

    let known_keys_slice: Vec<&str> = known_keys.iter().map(|s| s.as_str()).collect();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn from_args(args: &::zyn::Args) -> ::zyn::Result<Self> {
                let mut __diags = ::zyn::mark::new();

                #(#let_bindings)*

                let __known: &[&str] = &[#(#known_keys_slice),*];
                for __arg in args.iter() {
                    if let ::std::option::Option::Some(__ident) = __arg.name() {
                        let __key = __ident.to_string();
                        if !__known.contains(&__key.as_str()) {
                            let mut __d = ::zyn::mark::error(
                                ::std::format!("unknown argument `{}`", __key),
                            ).span(__ident.span());

                            if let ::std::option::Option::Some(__s) = ::zyn::closest_match(&__key, __known) {
                                __d = __d.add(
                                    ::zyn::mark::help(::std::format!("did you mean `{}`?", __s))
                                        .span(__ident.span())
                                );
                            }

                            __diags = __diags.add(__d);
                        }
                    }
                }

                let __diags = __diags.build();
                if __diags.is_error() {
                    return ::std::result::Result::Err(__diags);
                }

                ::std::result::Result::Ok(Self {
                    #(#struct_inits,)*
                })
            }
        }
    }
}

fn build_missing_msg(key: &str, field: &FieldMeta) -> String {
    let mut msg = format!("missing required field `{key}`");

    if let Some(about) = &field.about {
        let ty = &field.ty;
        let type_str = zyn_core::quote::quote!(#ty).to_string().replace(" ", "");
        msg = format!("{msg}\n{key}: {type_str} (required) — {about}");
    }

    msg
}

pub fn from_arg(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics<'_>,
    ty_generics: &syn::TypeGenerics<'_>,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    quote! {
        impl #impl_generics ::zyn::FromArg for #name #ty_generics #where_clause {
            fn from_arg(arg: &::zyn::Arg) -> ::zyn::Result<Self> {
                match arg {
                    ::zyn::Arg::List(_, args) => Self::from_args(args),
                    _ => ::std::result::Result::Err(
                        ::zyn::mark::error("expected list argument")
                            .span(::zyn::syn::spanned::Spanned::span(arg))
                            .build()
                    ),
                }
            }
        }
    }
}

pub fn from_input(
    name: &syn::Ident,
    meta: &StructMeta,
    impl_generics: &syn::ImplGenerics<'_>,
    ty_generics: &syn::TypeGenerics<'_>,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    let attr_name = meta.attr_name.as_deref().unwrap();

    let unique_check = if meta.unique {
        let msg = format!("only one #[{attr_name}(...)] allowed");
        quote! {
            if matches.len() > 1 {
                return ::std::result::Result::Err(
                    ::zyn::mark::error(#msg).build()
                );
            }
        }
    } else {
        quote! {}
    };

    let merge_or_first = if meta.unique {
        quote! {
            match matches.first() {
                ::std::option::Option::Some(attr) => {
                    let args: ::zyn::Args = attr.parse_args().map_err(::zyn::Diagnostic::from)?;
                    Self::from_args(&args)
                }
                ::std::option::Option::None => Self::from_args(&::zyn::Args::new()),
            }
        }
    } else {
        quote! {
            let mut merged = ::zyn::Args::new();
            for attr in &matches {
                let args: ::zyn::Args = attr.parse_args().map_err(::zyn::Diagnostic::from)?;
                merged.extend(args);
            }
            Self::from_args(&merged)
        }
    };

    quote! {
        impl #impl_generics ::zyn::FromInput for #name #ty_generics #where_clause {
            fn from_input(input: &::zyn::Input) -> ::zyn::Result<Self> {
                let matches: ::std::vec::Vec<_> = input.attrs().iter()
                    .filter(|a| a.path().is_ident(#attr_name))
                    .collect();

                #unique_check
                #merge_or_first
            }
        }
    }
}

pub fn about(
    name: &syn::Ident,
    meta: &StructMeta,
    fields: &[FieldMeta],
    impl_generics: &syn::ImplGenerics<'_>,
    ty_generics: &syn::TypeGenerics<'_>,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    let attr_name = meta.attr_name.as_deref().unwrap();

    let header = match &meta.about {
        Some(about) => format!("#[{attr_name}(...)]: {about}"),
        None => format!("#[{attr_name}(...)]"),
    };

    let mut lines = vec![header, String::new(), "Arguments:".to_string()];

    for f in fields {
        if f.skip {
            continue;
        }

        let type_str = {
            let ty = &f.ty;
            zyn_core::quote::quote!(#ty).to_string().replace(" ", "")
        };

        let status = match (&f.default, f.is_bool(), f.option_inner().is_some()) {
            (_, true, _) | (_, _, true) => String::new(),
            (FieldDefault::None, false, false) => " (required)".to_string(),
            (FieldDefault::Unit, false, false) => " (default: Default)".to_string(),
            (FieldDefault::Expr(expr), false, false) => {
                format!(" (default: {})", expr.to_string().replace(" ", ""))
            }
        };

        let about_suffix = match &f.about {
            Some(a) => format!(" — {a}"),
            None => String::new(),
        };

        let field_name = f.ident.to_string();
        let line = match &f.key {
            FieldKey::Positional(idx) => {
                format!("[{idx}] {field_name}: {type_str}{status}{about_suffix}")
            }
            FieldKey::Named(key) => {
                format!("{key}: {type_str}{status}{about_suffix}")
            }
        };

        lines.push(line);
    }

    let about_str = lines.join("\n");

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn about() -> &'static str {
                #about_str
            }
        }
    }
}
