use zyn_core::syn;
use zyn_core::syn::parse::ParseStream;

#[derive(Clone, Copy)]
pub enum DebugFormat {
    Raw,
    #[cfg(feature = "pretty")]
    Pretty,
}

#[derive(Clone, Copy)]
pub struct DebugConfig {
    pub format: DebugFormat,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            format: DebugFormat::Raw,
        }
    }
}

pub fn parse_debug_arg(input: ParseStream) -> syn::Result<Option<DebugConfig>> {
    if !input.peek(syn::Ident) {
        return Ok(None);
    }

    let fork = input.fork();
    let ident: syn::Ident = fork.parse()?;

    if ident != "debug" {
        return Ok(None);
    }

    input.parse::<syn::Ident>()?;

    if input.peek(syn::Token![=]) {
        input.parse::<syn::Token![=]>()?;

        let lit: syn::LitStr = input.parse()?;

        match lit.value().as_str() {
            #[cfg(feature = "pretty")]
            "pretty" => Ok(Some(DebugConfig {
                format: DebugFormat::Pretty,
            })),
            #[cfg(not(feature = "pretty"))]
            "pretty" => Err(syn::Error::new(
                lit.span(),
                "enable the `pretty` feature to use `debug = \"pretty\"`",
            )),
            _ => Err(syn::Error::new(lit.span(), "expected \"pretty\"")),
        }
    } else {
        Ok(Some(DebugConfig::default()))
    }
}
