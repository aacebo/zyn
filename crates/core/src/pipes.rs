use crate::Pipe;
use crate::case;

/// Converts the input to UPPERCASE.
///
/// Template usage: `{{ name | upper }}`
pub struct Upper;

impl Pipe for Upper {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&input.to_uppercase(), proc_macro2::Span::call_site())
    }
}

/// Converts the input to lowercase.
///
/// Template usage: `{{ name | lower }}`
pub struct Lower;

impl Pipe for Lower {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&input.to_lowercase(), proc_macro2::Span::call_site())
    }
}

/// Converts the input to snake_case.
///
/// Template usage: `{{ name | snake }}`
pub struct Snake;

impl Pipe for Snake {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&case::to_snake(&input), proc_macro2::Span::call_site())
    }
}

/// Converts the input to camelCase.
///
/// Template usage: `{{ name | camel }}`
pub struct Camel;

impl Pipe for Camel {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&case::to_camel(&input), proc_macro2::Span::call_site())
    }
}

/// Converts the input to PascalCase.
///
/// Template usage: `{{ name | pascal }}`
pub struct Pascal;

impl Pipe for Pascal {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&case::to_pascal(&input), proc_macro2::Span::call_site())
    }
}

/// Converts the input to kebab-case as a string literal.
///
/// Unlike other pipes that return `Ident`, this returns a `LitStr`
/// because hyphens are not valid in Rust identifiers.
///
/// Template usage: `{{ name | kebab }}`
pub struct Kebab;

impl Pipe for Kebab {
    type Input = String;
    type Output = syn::LitStr;

    fn pipe(&self, input: String) -> syn::LitStr {
        syn::LitStr::new(&case::to_kebab(&input), proc_macro2::Span::call_site())
    }
}

/// Converts the input to SCREAMING_SNAKE_CASE.
///
/// Template usage: `{{ name | screaming }}`
pub struct Screaming;

impl Pipe for Screaming {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&case::to_screaming(&input), proc_macro2::Span::call_site())
    }
}

/// Formats the input using a pattern string, producing an `Ident`.
///
/// The `{}` placeholder in the pattern is replaced with the input value.
///
/// Template usage: `{{ name | ident:"get_{}" }}`
pub struct Ident(pub &'static str);

impl Pipe for Ident {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let formatted = self.0.replace("{}", &input);
        proc_macro2::Ident::new(&formatted, proc_macro2::Span::call_site())
    }
}

/// Formats the input using a pattern string, producing a string literal.
///
/// The `{}` placeholder in the pattern is replaced with the input value.
///
/// Template usage: `{{ name | fmt:"hello {}" }}`
pub struct Fmt(pub &'static str);

impl Pipe for Fmt {
    type Input = String;
    type Output = syn::LitStr;

    fn pipe(&self, input: String) -> syn::LitStr {
        let formatted = self.0.replace("{}", &input);
        syn::LitStr::new(&formatted, proc_macro2::Span::call_site())
    }
}

pub struct Str;

impl Pipe for Str {
    type Input = String;
    type Output = syn::LitStr;

    fn pipe(&self, input: String) -> syn::LitStr {
        syn::LitStr::new(&input, proc_macro2::Span::call_site())
    }
}

pub struct Trim;

impl Pipe for Trim {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let trimmed = input.trim_matches('_');
        proc_macro2::Ident::new(trimmed, proc_macro2::Span::call_site())
    }
}

pub struct Plural;

impl Pipe for Plural {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let result = if input.ends_with('s') {
            format!("{}es", input)
        } else {
            format!("{}s", input)
        };
        proc_macro2::Ident::new(&result, proc_macro2::Span::call_site())
    }
}

pub struct Singular;

impl Pipe for Singular {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let result = if input.ends_with('s') && !input.ends_with("ss") {
            &input[..input.len() - 1]
        } else {
            &input
        };
        proc_macro2::Ident::new(result, proc_macro2::Span::call_site())
    }
}
