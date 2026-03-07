//! Built-in pipe transforms for template value interpolation.
//!
//! Each pipe is a zero-sized struct implementing [`crate::Pipe`]. Pipes are applied
//! in `{{ expr | pipe }}` expressions inside `zyn!` templates and can be chained.
//!
//! # Template usage
//!
//! ```ignore
//! // Single pipe
//! zyn::zyn! { fn {{ name | snake }}() {} }
//! // name = "HelloWorld" → fn hello_world() {}
//!
//! // Chained pipes
//! zyn::zyn! { fn {{ name | snake | ident:"get_{}" }}() {} }
//! // name = "HelloWorld" → fn get_hello_world() {}
//!
//! // Pipe to string literal
//! zyn::zyn! { const PATH: &str = {{ name | kebab }}; }
//! // name = "MyComponent" → const PATH: &str = "my-component";
//! ```
//!
//! ## Built-in pipe reference
//!
//! All case pipes accept `String` input. Return type varies:
//!
//! | Pipe | Input | Output | Returns |
//! |------|-------|--------|---------|
//! | `snake` | `HelloWorld` | `hello_world` | `Ident` |
//! | `pascal` | `hello_world` | `HelloWorld` | `Ident` |
//! | `camel` | `hello_world` | `helloWorld` | `Ident` |
//! | `screaming` | `HelloWorld` | `HELLO_WORLD` | `Ident` |
//! | `kebab` | `HelloWorld` | `"hello-world"` | `LitStr` |
//! | `upper` | `hello` | `HELLO` | `Ident` |
//! | `lower` | `HELLO` | `hello` | `Ident` |
//! | `str` | `hello` | `"hello"` | `LitStr` |
//! | `plural` | `user` | `users` | `Ident` |
//! | `singular` | `users` | `user` | `Ident` |
//! | `ident:"pat_{}"` | `hello` | `pat_hello` | `Ident` |
//! | `fmt:"pat_{}"` | `hello` | `"pat_hello"` | `LitStr` |
//! | `trim` | `__foo__` | `foo` | `Ident` |
//!
//! `kebab`, `str`, and `fmt` return `LitStr` because their output may contain
//! characters invalid in Rust identifiers.
//!
//! # Using pipes outside templates
//!
//! ```ignore
//! use zyn_core::{Pipe, pipes};
//!
//! let ident = pipes::Snake.pipe("HelloWorld".to_string());
//! // → hello_world (proc_macro2::Ident)
//!
//! let lit = pipes::Fmt("error_{}.rs").pipe("not_found".to_string());
//! // → "error_not_found.rs" (syn::LitStr)
//! ```

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

/// Converts the input to a string literal.
///
/// Template usage: `{{ name | str }}`
pub struct Str;

impl Pipe for Str {
    type Input = String;
    type Output = syn::LitStr;

    fn pipe(&self, input: String) -> syn::LitStr {
        syn::LitStr::new(&input, proc_macro2::Span::call_site())
    }
}

/// Trims characters from the start and end of the input.
///
/// Template usage: `{{ name | trim:"_" }}`
pub struct Trim(pub &'static str, pub &'static str);

impl Pipe for Trim {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let trimmed = input
            .trim_start_matches(|c: char| self.0.contains(c))
            .trim_end_matches(|c: char| self.1.contains(c));
        proc_macro2::Ident::new(trimmed, proc_macro2::Span::call_site())
    }
}

/// Converts the input to its plural form.
///
/// Template usage: `{{ name | plural }}`
pub struct Plural;

impl Pipe for Plural {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let result = if input.ends_with('y')
            && input
                .chars()
                .rev()
                .nth(1)
                .is_some_and(|c| !"aeiou".contains(c))
        {
            format!("{}ies", &input[..input.len() - 1])
        } else if input.ends_with('s')
            || input.ends_with('x')
            || input.ends_with('z')
            || input.ends_with("ch")
            || input.ends_with("sh")
        {
            format!("{}es", input)
        } else {
            format!("{}s", input)
        };
        proc_macro2::Ident::new(&result, proc_macro2::Span::call_site())
    }
}

/// Converts the input to its singular form.
///
/// Template usage: `{{ name | singular }}`
pub struct Singular;

impl Pipe for Singular {
    type Input = String;
    type Output = proc_macro2::Ident;

    fn pipe(&self, input: String) -> proc_macro2::Ident {
        let result = if input.ends_with("ies") {
            format!("{}y", &input[..input.len() - 3])
        } else if input.ends_with("ses")
            || input.ends_with("xes")
            || input.ends_with("zes")
            || input.ends_with("ches")
            || input.ends_with("shes")
        {
            input[..input.len() - 2].to_string()
        } else if input.ends_with('s') && !input.ends_with("ss") {
            input[..input.len() - 1].to_string()
        } else {
            input
        };
        proc_macro2::Ident::new(&result, proc_macro2::Span::call_site())
    }
}
