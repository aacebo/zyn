use zyn_core::syn;
use zyn_core::syn::parse::ParseStream;

#[derive(Clone, Copy)]
pub enum DebugFormat {
    Raw,
    #[cfg(feature = "pretty")]
    Pretty,
}

#[derive(Clone)]
pub struct DebugConfig {
    pub format: DebugFormat,
    /// When `true`, emit the full generated struct + impl output.
    /// When `false` (default), emit only the expanded body tokens.
    pub full: bool,
    /// Static injection values: `key = "value"` pairs parsed from the debug attribute.
    /// Each string value is parsed as a `TokenStream` and substituted for the matching
    /// interpolation expression in the debug output.
    pub injections: Vec<(String, syn::Expr)>,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            format: DebugFormat::Raw,
            full: false,
            injections: Vec::new(),
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

    input.parse::<syn::Ident>()?; // consume "debug"

    // debug with no options → defaults (body only, raw)
    if !input.peek(syn::token::Paren) {
        return Ok(Some(DebugConfig::default()));
    }

    // debug(opt1, opt2, ...)
    let content;
    syn::parenthesized!(content in input);

    #[allow(unused_mut)]
    let mut format = DebugFormat::Raw;
    let mut full = false;
    let mut injections: Vec<(String, syn::Expr)> = Vec::new();

    while !content.is_empty() {
        // peek ahead: `ident =` (not `pretty` or `full`) → injection pair
        if content.peek(syn::Ident) {
            let fork = content.fork();
            if let Ok(key) = fork.parse::<syn::Ident>() {
                if fork.peek(syn::Token![=]) && key != "pretty" && key != "full" {
                    let key: syn::Ident = content.parse()?;
                    content.parse::<syn::Token![=]>()?;
                    let expr: syn::Expr = content.parse()?;
                    injections.push((key.to_string(), expr));

                    if content.peek(syn::Token![,]) {
                        content.parse::<syn::Token![,]>()?;
                    }

                    continue;
                }
            }
        }

        let opt: syn::Ident = content.parse()?;

        match opt.to_string().as_str() {
            "pretty" => {
                #[cfg(feature = "pretty")]
                {
                    format = DebugFormat::Pretty;
                }
                #[cfg(not(feature = "pretty"))]
                return Err(syn::Error::new(
                    opt.span(),
                    "enable the `pretty` feature to use `debug(pretty)`",
                ));
            }
            "full" => {
                full = true;
            }
            other => {
                return Err(syn::Error::new(
                    opt.span(),
                    format!("unknown debug option `{other}`, expected `pretty` or `full`"),
                ));
            }
        }

        if content.peek(syn::Token![,]) {
            content.parse::<syn::Token![,]>()?;
        }
    }

    Ok(Some(DebugConfig {
        format,
        full,
        injections,
    }))
}
