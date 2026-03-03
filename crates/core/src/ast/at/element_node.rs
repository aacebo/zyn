use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;
use quote::quote;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::super::Element;

use crate::Expand;

pub struct ElementNode {
    pub span: Span,
    pub name: TokenStream,
    pub props: Vec<(syn::Ident, TokenStream)>,
    pub children: Option<Box<Element>>,
}

impl ElementNode {
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn parse_with_ident(input: ParseStream, first_ident: syn::Ident) -> syn::Result<Self> {
        let span = first_ident.span();

        let mut name = TokenStream::new();
        first_ident.to_tokens(&mut name);

        while input.peek(Token![::]) {
            let colons: Token![::] = input.parse()?;
            colons.to_tokens(&mut name);
            let segment: syn::Ident = input.parse()?;
            segment.to_tokens(&mut name);
        }

        let props_content;
        syn::braced!(props_content in input);

        let mut props = Vec::new();

        while !props_content.is_empty() {
            let prop_name: syn::Ident = props_content.parse()?;
            props_content.parse::<Token![:]>()?;

            let mut value = TokenStream::new();

            while !props_content.is_empty() && !props_content.peek(Token![,]) {
                let tt: TokenTree = props_content.parse()?;
                tt.to_tokens(&mut value);
            }

            props.push((prop_name, value));

            if props_content.peek(Token![,]) {
                props_content.parse::<Token![,]>()?;
            }
        }

        let children = if input.peek(syn::token::Brace) {
            let children_content;
            syn::braced!(children_content in input);
            Some(Box::new(children_content.parse::<Element>()?))
        } else {
            None
        };

        Ok(Self {
            span,
            name,
            props,
            children,
        })
    }
}

impl Expand for ElementNode {
    fn expand(&self, output: &Ident, idents: &mut crate::ident::Iter) -> TokenStream {
        let name = &self.name;
        let prop_names: Vec<&syn::Ident> = self.props.iter().map(|(n, _)| n).collect();
        let prop_values: Vec<&TokenStream> = self.props.iter().map(|(_, v)| v).collect();

        if let Some(children) = &self.children {
            let inner = idents.next().unwrap();
            let children_expanded = children.expand(&inner, idents);

            quote! {
                {
                    let mut #inner = ::proc_macro2::TokenStream::new();
                    #children_expanded
                    let __zyn_rendered = ::zyn::Render::render(&#name {
                        #(#prop_names: #prop_values,)*
                        children: #inner,
                    })?;
                    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut #output);
                }
            }
        } else {
            quote! {
                {
                    let __zyn_rendered = ::zyn::Render::render(&#name {
                        #(#prop_names: #prop_values,)*
                    })?;
                    ::quote::ToTokens::to_tokens(&__zyn_rendered, &mut #output);
                }
            }
        }
    }
}

impl Parse for ElementNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first_ident: syn::Ident = input.parse()?;
        let span = first_ident.span();

        let mut name = TokenStream::new();
        first_ident.to_tokens(&mut name);

        while input.peek(Token![::]) {
            let colons: Token![::] = input.parse()?;
            colons.to_tokens(&mut name);
            let segment: syn::Ident = input.parse()?;
            segment.to_tokens(&mut name);
        }

        let props_content;
        syn::braced!(props_content in input);

        let mut props = Vec::new();

        while !props_content.is_empty() {
            let prop_name: syn::Ident = props_content.parse()?;
            props_content.parse::<Token![:]>()?;

            let mut value = TokenStream::new();

            while !props_content.is_empty() && !props_content.peek(Token![,]) {
                let tt: TokenTree = props_content.parse()?;
                tt.to_tokens(&mut value);
            }

            props.push((prop_name, value));

            if props_content.peek(Token![,]) {
                props_content.parse::<Token![,]>()?;
            }
        }

        let children = if input.peek(syn::token::Brace) {
            let children_content;
            syn::braced!(children_content in input);
            Some(Box::new(children_content.parse::<Element>()?))
        } else {
            None
        };

        Ok(Self {
            span,
            name,
            props,
            children,
        })
    }
}
