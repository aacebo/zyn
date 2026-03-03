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
use crate::ident;

pub struct UpperPipe {
    pub span: Span,
}

pub struct LowerPipe {
    pub span: Span,
}

pub struct SnakePipe {
    pub span: Span,
}

pub struct CamelPipe {
    pub span: Span,
}

pub struct PascalPipe {
    pub span: Span,
}

pub struct ScreamingPipe {
    pub span: Span,
}

pub struct CustomPipe {
    pub span: Span,
    pub name: syn::Ident,
    pub args: Vec<TokenStream>,
}

pub enum PipeNode {
    Upper(UpperPipe),
    Lower(LowerPipe),
    Snake(SnakePipe),
    Camel(CamelPipe),
    Pascal(PascalPipe),
    Screaming(ScreamingPipe),
    Custom(CustomPipe),
}

impl PipeNode {
    pub fn is_upper(&self) -> bool {
        matches!(self, Self::Upper(_))
    }

    pub fn is_lower(&self) -> bool {
        matches!(self, Self::Lower(_))
    }

    pub fn is_snake(&self) -> bool {
        matches!(self, Self::Snake(_))
    }

    pub fn is_camel(&self) -> bool {
        matches!(self, Self::Camel(_))
    }

    pub fn is_pascal(&self) -> bool {
        matches!(self, Self::Pascal(_))
    }

    pub fn is_screaming(&self) -> bool {
        matches!(self, Self::Screaming(_))
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl PipeNode {
    pub fn as_upper(&self) -> &UpperPipe {
        match self {
            Self::Upper(v) => v,
            _ => panic!("called as_upper on non-Upper pipe"),
        }
    }

    pub fn as_lower(&self) -> &LowerPipe {
        match self {
            Self::Lower(v) => v,
            _ => panic!("called as_lower on non-Lower pipe"),
        }
    }

    pub fn as_snake(&self) -> &SnakePipe {
        match self {
            Self::Snake(v) => v,
            _ => panic!("called as_snake on non-Snake pipe"),
        }
    }

    pub fn as_camel(&self) -> &CamelPipe {
        match self {
            Self::Camel(v) => v,
            _ => panic!("called as_camel on non-Camel pipe"),
        }
    }

    pub fn as_pascal(&self) -> &PascalPipe {
        match self {
            Self::Pascal(v) => v,
            _ => panic!("called as_pascal on non-Pascal pipe"),
        }
    }

    pub fn as_screaming(&self) -> &ScreamingPipe {
        match self {
            Self::Screaming(v) => v,
            _ => panic!("called as_screaming on non-Screaming pipe"),
        }
    }

    pub fn as_custom(&self) -> &CustomPipe {
        match self {
            Self::Custom(v) => v,
            _ => panic!("called as_custom on non-Custom pipe"),
        }
    }
}

impl PipeNode {
    pub fn span(&self) -> Span {
        match self {
            Self::Upper(v) => v.span,
            Self::Lower(v) => v.span,
            Self::Snake(v) => v.span,
            Self::Camel(v) => v.span,
            Self::Pascal(v) => v.span,
            Self::Screaming(v) => v.span,
            Self::Custom(v) => v.span,
        }
    }
}

impl From<UpperPipe> for PipeNode {
    fn from(v: UpperPipe) -> Self {
        Self::Upper(v)
    }
}

impl From<LowerPipe> for PipeNode {
    fn from(v: LowerPipe) -> Self {
        Self::Lower(v)
    }
}

impl From<SnakePipe> for PipeNode {
    fn from(v: SnakePipe) -> Self {
        Self::Snake(v)
    }
}

impl From<CamelPipe> for PipeNode {
    fn from(v: CamelPipe) -> Self {
        Self::Camel(v)
    }
}

impl From<PascalPipe> for PipeNode {
    fn from(v: PascalPipe) -> Self {
        Self::Pascal(v)
    }
}

impl From<ScreamingPipe> for PipeNode {
    fn from(v: ScreamingPipe) -> Self {
        Self::Screaming(v)
    }
}

impl From<CustomPipe> for PipeNode {
    fn from(v: CustomPipe) -> Self {
        Self::Custom(v)
    }
}

impl Parse for PipeNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let span = name.span();

        match name.to_string().as_str() {
            "upper" => Ok(UpperPipe { span }.into()),
            "lower" => Ok(LowerPipe { span }.into()),
            "snake" => Ok(SnakePipe { span }.into()),
            "camel" => Ok(CamelPipe { span }.into()),
            "pascal" => Ok(PascalPipe { span }.into()),
            "screaming" => Ok(ScreamingPipe { span }.into()),
            _ => {
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

                Ok(CustomPipe { span, name, args }.into())
            }
        }
    }
}

impl Expand for PipeNode {
    fn expand(&self, output: &Ident, idents: &mut ident::Iter) -> TokenStream {
        match self {
            Self::Upper(v) => v.expand(output, idents),
            Self::Lower(v) => v.expand(output, idents),
            Self::Snake(v) => v.expand(output, idents),
            Self::Camel(v) => v.expand(output, idents),
            Self::Pascal(v) => v.expand(output, idents),
            Self::Screaming(v) => v.expand(output, idents),
            Self::Custom(v) => v.expand(output, idents),
        }
    }
}

impl Expand for UpperPipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        quote! { let __zyn_val = __zyn_val.to_uppercase(); }
    }
}

impl Expand for LowerPipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        quote! { let __zyn_val = __zyn_val.to_lowercase(); }
    }
}

impl Expand for SnakePipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        quote! { let __zyn_val = ::zyn::to_snake_case(&__zyn_val); }
    }
}

impl Expand for CamelPipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        quote! { let __zyn_val = ::zyn::to_camel_case(&__zyn_val); }
    }
}

impl Expand for PascalPipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        quote! { let __zyn_val = ::zyn::to_pascal_case(&__zyn_val); }
    }
}

impl Expand for ScreamingPipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        quote! { let __zyn_val = ::zyn::to_screaming_case(&__zyn_val); }
    }
}

impl Expand for CustomPipe {
    fn expand(&self, _output: &Ident, _idents: &mut ident::Iter) -> TokenStream {
        let name = &self.name;
        quote! { let __zyn_val = ::zyn::Pipe::pipe(&(#name), __zyn_val); }
    }
}
