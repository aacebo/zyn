use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;
use quote::quote;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;
use crate::Pipe;
use crate::pascal;
use crate::pipes;

/// A single pipe stage in a `{{ expr | pipe }}` interpolation.
///
/// At expand time, the name is matched against the built-in pipe list. Unrecognised
/// names are assumed to be custom pipe structs and are PascalCase-converted.
///
/// ```text
/// {{ name | snake }}          → PipeNode { name: "snake", args: [] }
/// {{ name | ident:"get_{}" }} → PipeNode { name: "ident", args: ["get_{}"] }
/// ```
pub struct PipeNode {
    /// Source span of the pipe name.
    pub span: Span,
    /// The pipe name as written, e.g. `snake`, `ident`, `my_custom_pipe`.
    pub name: syn::Ident,
    /// Colon-separated arguments following the name.
    pub args: Vec<TokenStream>,
}

impl PipeNode {
    pub fn span(&self) -> Span {
        self.span
    }

    /// Apply this pipe to `input` at proc-macro time, returning the transformed string.
    /// Used for debug display with static injection. Custom pipes pass through unchanged.
    pub fn apply_display(&self, input: String) -> String {
        let name = self.name.to_string();

        let arg = |i: usize| -> String {
            self.args
                .get(i)
                .map(|a| a.to_string().trim_matches('"').to_string())
                .unwrap_or_default()
        };

        match name.as_str() {
            "upper" => pipes::Upper.pipe(input).to_string(),
            "lower" => pipes::Lower.pipe(input).to_string(),
            "snake" => pipes::Snake.pipe(input).to_string(),
            "camel" => pipes::Camel.pipe(input).to_string(),
            "pascal" => pipes::Pascal.pipe(input).to_string(),
            "kebab" => pipes::Kebab.pipe(input).value(),
            "screaming" => pipes::Screaming.pipe(input).to_string(),
            "str" => pipes::Str.pipe(input).value(),
            "plural" => pipes::Plural.pipe(input).to_string(),
            "singular" => pipes::Singular.pipe(input).to_string(),
            "ident" | "fmt" => arg(0).replace("{}", &input),
            "trim" => {
                let start = arg(0);
                let end = if self.args.len() > 1 {
                    arg(1)
                } else {
                    start.clone()
                };
                input
                    .trim_start_matches(|c: char| start.contains(c))
                    .trim_end_matches(|c: char| end.contains(c))
                    .to_string()
            }
            // Custom pipes are not available at proc-macro time — pass through unchanged
            _ => input,
        }
    }
}

impl Parse for PipeNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let span = name.span();

        let mut args = Vec::new();

        while input.peek(Token![:]) {
            input.parse::<Token![:]>()?;

            let mut arg = TokenStream::new();

            while !input.is_empty() && !input.peek(Token![:]) && !input.peek(Token![|]) {
                let tt: TokenTree = input.parse()?;
                tt.to_tokens(&mut arg);
            }

            args.push(arg);
        }

        Ok(Self { span, name, args })
    }
}

fn is_builtin(name: &str) -> bool {
    matches!(
        name,
        "upper"
            | "lower"
            | "snake"
            | "camel"
            | "pascal"
            | "kebab"
            | "screaming"
            | "ident"
            | "fmt"
            | "str"
            | "trim"
            | "plural"
            | "singular"
    )
}

impl Expand for PipeNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let pascal_name = pascal!(self.name => ident);
        let name_str = self.name.to_string();

        if is_builtin(&name_str) {
            if name_str == "trim" {
                match self.args.as_slice() {
                    [] => {
                        quote! { let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::pipes::Trim(" ", " ")), __zyn_val); }
                    }
                    [a] => {
                        quote! { let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::pipes::Trim(#a, #a)), __zyn_val); }
                    }
                    [a, b] => {
                        quote! { let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::pipes::Trim(#a, #b)), __zyn_val); }
                    }
                    _ => quote! { compile_error!("trim pipe accepts at most 2 arguments"); },
                }
            } else if self.args.is_empty() {
                quote! { let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::pipes::#pascal_name), __zyn_val); }
            } else {
                let args = &self.args;
                quote! { let __zyn_val = ::zyn::Pipe::pipe(&(::zyn::pipes::#pascal_name(#(#args),*)), __zyn_val); }
            }
        } else if self.args.is_empty() {
            quote! { let __zyn_val = ::zyn::Pipe::pipe(&(#pascal_name), __zyn_val); }
        } else {
            let args = &self.args;
            quote! { let __zyn_val = ::zyn::Pipe::pipe(&(#pascal_name(#(#args),*)), __zyn_val); }
        }
    }
}
