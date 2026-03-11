use zyn_core::proc_macro2::Span;
use zyn_core::proc_macro2::TokenStream;
use zyn_core::syn;

use zyn_core::meta::Arg;
use zyn_core::meta::Args;

pub enum FieldKey {
    Positional(usize),
    Named(String),
}

pub enum FieldDefault {
    None,
    Unit,
    Expr(TokenStream),
}

pub struct FieldMeta {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub key: FieldKey,
    pub default: FieldDefault,
    pub skip: bool,
    pub about: Option<String>,
}

impl FieldMeta {
    pub fn parse(input: &syn::DeriveInput) -> syn::Result<Vec<Self>> {
        let fields = match &input.data {
            syn::Data::Struct(s) => &s.fields,
            _ => unreachable!(),
        };

        let named = match fields {
            syn::Fields::Named(n) => &n.named,
            _ => {
                return Err(syn::Error::new(
                    input.ident.span(),
                    "#[derive(Attribute)] only supports named-field structs",
                ));
            }
        };

        let mut result = Vec::new();

        for field in named {
            let ident = field.ident.clone().unwrap();
            let ty = field.ty.clone();
            let (key, default, skip, about) = Self::parse_attrs(&ident, &field.attrs)?;

            result.push(Self {
                ident,
                ty,
                key,
                default,
                skip,
                about,
            });
        }

        Ok(result)
    }

    pub fn is_bool(&self) -> bool {
        matches!(&self.ty, syn::Type::Path(p) if p.qself.is_none() && p.path.is_ident("bool"))
    }

    pub fn option_inner(&self) -> Option<&syn::Type> {
        if let syn::Type::Path(p) = &self.ty
            && let Some(seg) = p.path.segments.last()
            && seg.ident == "Option"
            && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
            && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
        {
            return Some(inner);
        }
        None
    }

    fn parse_attrs(
        ident: &syn::Ident,
        attrs: &[syn::Attribute],
    ) -> syn::Result<(FieldKey, FieldDefault, bool, Option<String>)> {
        let mut key = FieldKey::Named(ident.to_string());
        let mut default = FieldDefault::None;
        let mut skip = false;
        let mut about = None;

        for attr in attrs {
            if !attr.path().is_ident("zyn") {
                continue;
            }

            let args: Args = attr.parse_args()?;

            for arg in &args {
                match arg {
                    Arg::Lit(syn::Lit::Int(i)) => {
                        let idx: usize = i.base10_parse()?;
                        key = FieldKey::Positional(idx);
                    }
                    Arg::Lit(syn::Lit::Str(s)) => {
                        key = FieldKey::Named(s.value());
                    }
                    Arg::Flag(f) if f == "skip" => {
                        skip = true;
                    }
                    Arg::Flag(f) if f == "default" => {
                        default = FieldDefault::Unit;
                    }
                    Arg::Expr(k, expr) if k == "default" => {
                        default = FieldDefault::Expr(zyn_core::quote::quote!(#expr));
                    }
                    Arg::Expr(
                        k,
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }),
                    ) if k == "about" => {
                        about = Some(s.value());
                    }
                    _ => {
                        return Err(syn::Error::new(
                            arg.name().map(|i| i.span()).unwrap_or(Span::call_site()),
                            "unexpected field-level zyn annotation",
                        ));
                    }
                }
            }
        }

        Ok((key, default, skip, about))
    }
}
