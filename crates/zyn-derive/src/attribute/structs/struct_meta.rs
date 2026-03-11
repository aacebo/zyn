use zyn_core::proc_macro2::Span;
use zyn_core::syn;

use zyn_core::meta::Arg;
use zyn_core::meta::Args;

pub struct StructMeta {
    pub attr_name: Option<String>,
    pub unique: bool,
    pub about: Option<String>,
}

impl StructMeta {
    pub fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut attr_name = None;
        let mut unique = false;
        let mut about = None;

        for attr in attrs {
            if !attr.path().is_ident("zyn") {
                continue;
            }

            let args: Args = attr.parse_args()?;

            for arg in &args {
                match arg {
                    Arg::Lit(syn::Lit::Str(s)) => {
                        attr_name = Some(s.value());
                    }
                    Arg::Flag(f) if f == "unique" => {
                        unique = true;
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
                            "unexpected struct-level zyn annotation",
                        ));
                    }
                }
            }
        }

        Ok(Self {
            attr_name,
            unique,
            about,
        })
    }
}
