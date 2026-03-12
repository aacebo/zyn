use zyn::ext::AttrExt;

#[derive(zyn::Attribute)]
#[zyn("builder")]
struct BuilderConfig {
    #[zyn(default)]
    skip: bool,
    #[zyn(default)]
    default: bool,
    default_value: Option<String>,
}

impl BuilderConfig {
    fn from_field(field: &zyn::syn::Field) -> Self {
        let attr = field.attrs.iter().find(|a| a.is("builder"));

        match attr {
            Some(attr) => {
                let args = attr.args().unwrap();
                Self::from_args(&args).unwrap()
            }
            None => Self {
                skip: false,
                default: false,
                default_value: None,
            },
        }
    }
}

#[zyn::element]
fn setter(name: zyn::syn::Ident, ty: zyn::syn::Type) -> zyn::TokenStream {
    zyn::zyn! {
        fn {{ name }}(mut self, value: {{ ty }}) -> Self {
            self.{{ name }} = Some(value);
            self
        }
    }
}

#[zyn::element]
fn build_field(name: zyn::syn::Ident, config: BuilderConfig) -> zyn::TokenStream {
    let name_str = name.to_string();

    if config.default {
        zyn::zyn!({{ name }}: self.{{ name }}.unwrap_or_default())
    } else if let Some(ref expr) = config.default_value {
        let default_expr: zyn::syn::Expr = zyn::syn::parse_str(expr).unwrap();
        zyn::zyn!({{ name }}: self.{{ name }}.unwrap_or_else(|| {{ default_expr }}))
    } else {
        zyn::zyn!({{ name }}: self.{{ name }}.expect(
            ::std::concat!("field `", {{ name_str | str }}, "` is required")
        ))
    }
}

#[zyn::derive("Builder", attributes(builder))]
fn builder(
    #[zyn(input)] ident: zyn::syn::Ident,
    #[zyn(input)] fields: zyn::Fields<zyn::syn::FieldsNamed>,
) -> zyn::TokenStream {
    for field in fields.named.iter() {
        let config = BuilderConfig::from_field(field);

        if config.skip && config.default {
            error!(
                "`skip` and `default` are mutually exclusive on field `{}`",
                field.ident.as_ref().unwrap();
                span = field.ident.as_ref().unwrap().span()
            );
        }

        if config.skip && config.default_value.is_some() {
            warn!(
                "`default_value` is ignored when `skip` is set";
                span = field.ident.as_ref().unwrap().span()
            );
        }
    }

    // stop here if any errors accumulated, otherwise continue to codegen
    bail!();

    zyn::zyn! {
        struct {{ ident | ident:"{}Builder" }} {
            @for (field in fields.named.iter()) {
                {{ field.ident }}: Option<{{ field.ty }}>,
            }
        }

        impl {{ ident | ident:"{}Builder" }} {
            @for (field in fields.named.iter()) {
                @setter(
                    name = field.ident.clone().unwrap(),
                    ty = field.ty.clone(),
                )
            }

            fn build(self) -> {{ ident }} {
                {{ ident }} {
                    @for (field in fields.named.iter()) {
                        @build_field(
                            name = field.ident.clone().unwrap(),
                            config = BuilderConfig::from_field(field),
                        ),
                    }
                }
            }
        }

        impl {{ ident }} {
            fn builder() -> {{ ident | ident:"{}Builder" }} {
                {{ ident | ident:"{}Builder" }} {
                    @for (field in fields.named.iter()) {
                        {{ field.ident }}: None,
                    }
                }
            }
        }
    }
}
