mod derive;
pub(crate) mod item;

use quote::ToTokens;

pub use syn::{
    Attribute, BinOp, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Expr, ExprAssign,
    ExprBinary, ExprLit, Field, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Ident,
    Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemType, ItemUnion, ItemUse, Lit, LitBool, LitChar,
    LitFloat, LitInt, LitStr, Local, Pat, Path, Signature, Stmt, Token, Type, Variant, Visibility,
    WhereClause, WherePredicate,
};

pub use syn::fold;
pub use syn::parse;
pub use syn::parse::Parse;

pub enum Input {
    Derive(syn::DeriveInput),
    Item(syn::Item),
}

impl Input {
    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Derive(d) => &d.attrs,
            Self::Item(i) => item::attrs(i),
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Derive(d) => &d.ident,
            Self::Item(i) => item::ident(i),
        }
    }

    pub fn generics(&self) -> &syn::Generics {
        match self {
            Self::Derive(d) => &d.generics,
            Self::Item(i) => item::generics(i),
        }
    }

    pub fn vis(&self) -> &syn::Visibility {
        match self {
            Self::Derive(d) => &d.vis,
            Self::Item(i) => item::vis(i),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Derive(syn::parse_str("struct __ZynDefault;").unwrap())
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Derive(d) => d.to_tokens(tokens),
            Self::Item(i) => i.to_tokens(tokens),
        }
    }
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative;

        let fork = input.fork();
        if let Ok(d) = fork.parse::<syn::DeriveInput>() {
            input.advance_to(&fork);
            return Ok(Self::Derive(d));
        }

        Ok(Self::Item(input.parse()?))
    }
}

impl From<syn::DeriveInput> for Input {
    fn from(v: syn::DeriveInput) -> Self {
        Self::Derive(v)
    }
}

impl From<syn::Item> for Input {
    fn from(v: syn::Item) -> Self {
        Self::Item(v)
    }
}
