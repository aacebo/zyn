use zyn_core::proc_macro2::TokenStream;
use zyn_core::syn;

pub enum VariantKind {
    Unit,
    Struct(Vec<(syn::Ident, syn::Type)>),
    Tuple(Vec<syn::Type>),
}

pub struct VariantMeta {
    pub ident: syn::Ident,
    pub snake_name: String,
    pub kind: VariantKind,
}

impl VariantMeta {
    pub fn parse(input: &syn::DeriveInput) -> syn::Result<Vec<Self>> {
        let variants = match &input.data {
            syn::Data::Enum(e) => &e.variants,
            _ => unreachable!(),
        };

        let mut result = Vec::new();

        for variant in variants {
            let ident = variant.ident.clone();
            let snake_name = zyn_core::snake!(&ident.to_string());
            let kind = match &variant.fields {
                syn::Fields::Unit => VariantKind::Unit,
                syn::Fields::Named(n) => {
                    let fields = n
                        .named
                        .iter()
                        .map(|f| (f.ident.clone().unwrap(), f.ty.clone()))
                        .collect();
                    VariantKind::Struct(fields)
                }
                syn::Fields::Unnamed(u) => {
                    let tys = u.unnamed.iter().map(|f| f.ty.clone()).collect();
                    VariantKind::Tuple(tys)
                }
            };

            result.push(Self {
                ident,
                snake_name,
                kind,
            });
        }

        Ok(result)
    }

    pub fn arm_from_flag(&self) -> Option<TokenStream> {
        match &self.kind {
            VariantKind::Unit => {
                let name = &self.snake_name;
                let ident = &self.ident;
                Some(zyn_core::quote::quote! { #name => ::std::result::Result::Ok(Self::#ident) })
            }
            _ => None,
        }
    }

    pub fn arm_from_list(&self) -> Option<TokenStream> {
        match &self.kind {
            VariantKind::Struct(fields) => {
                let name = &self.snake_name;
                let ident = &self.ident;
                let field_inits: Vec<TokenStream> = fields.iter().map(|(field_ident, field_ty)| {
                    let key = field_ident.to_string();
                    zyn_core::quote::quote! {
                        #field_ident: match args.get(#key) {
                            ::std::option::Option::Some(arg) => <#field_ty as ::zyn::FromArg>::from_arg(arg)?,
                            ::std::option::Option::None => return ::std::result::Result::Err(
                                ::zyn::syn::Error::new(
                                    ident.span(),
                                    ::std::concat!("missing required field `", #key, "`"),
                                )
                            ),
                        }
                    }
                }).collect();
                Some(zyn_core::quote::quote! {
                    #name => ::std::result::Result::Ok(Self::#ident {
                        #(#field_inits,)*
                    })
                })
            }
            VariantKind::Tuple(tys) if tys.len() > 1 => {
                let name = &self.snake_name;
                let ident = &self.ident;
                let field_inits: Vec<TokenStream> = tys
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| {
                        zyn_core::quote::quote! {
                            <#ty as ::zyn::FromArg>::from_arg(&args[#i])?
                        }
                    })
                    .collect();
                Some(zyn_core::quote::quote! {
                    #name => ::std::result::Result::Ok(Self::#ident(#(#field_inits,)*))
                })
            }
            _ => None,
        }
    }

    pub fn arm_from_expr(&self) -> Option<TokenStream> {
        match &self.kind {
            VariantKind::Tuple(tys) if tys.len() == 1 => {
                let name = &self.snake_name;
                let ident = &self.ident;
                let ty = &tys[0];
                Some(zyn_core::quote::quote! {
                    #name => ::std::result::Result::Ok(Self::#ident(
                        ::std::convert::Into::into(<#ty as ::zyn::FromArg>::from_arg(arg)?)
                    ))
                })
            }
            _ => None,
        }
    }
}
