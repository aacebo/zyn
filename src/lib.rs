pub mod ast;
pub mod ident;

pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}

pub fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_uppercase();

            if prev_lower || (next_lower && prev_upper) {
                out.push('_');
            }

            out.extend(c.to_lowercase());
        } else if c == '_' {
            if !out.is_empty() && !out.ends_with('_') {
                out.push('_');
            }
        } else {
            out.push(c);
        }
    }

    out
}

pub fn to_pascal_case(s: &str) -> String {
    let mut out = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if c.is_uppercase() {
            if !out.is_empty()
                && !out
                    .chars()
                    .last()
                    .map(|p| p.is_uppercase())
                    .unwrap_or(false)
            {
                capitalize = true;
            }

            if capitalize {
                out.extend(c.to_uppercase());
                capitalize = false;
            } else {
                out.push(c);
            }
        } else if capitalize {
            out.extend(c.to_uppercase());
            capitalize = false;
        } else {
            out.push(c);
        }
    }

    out
}

pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_screaming_case(s: &str) -> String {
    to_snake_case(s).to_uppercase()
}
