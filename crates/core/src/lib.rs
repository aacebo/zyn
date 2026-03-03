pub mod ast;
pub mod case;
pub mod ident;

/// Internal trait for expanding AST nodes into generated code.
///
/// Each AST node implements this to produce a `TokenStream` that builds
/// the output incrementally using unique identifier variables.
pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

/// Trait for renderable elements invoked via `@element_name { props }` in templates.
///
/// Implement this on a struct to make it usable as a template element.
/// The struct fields become the element's props, and `render()` produces
/// the output token stream.
///
/// # Example
///
/// ```ignore
/// struct FieldDecl {
///     vis: syn::Visibility,
///     name: syn::Ident,
///     ty: syn::Type,
/// }
///
/// impl zyn::Render for FieldDecl {
///     fn render(&self) -> syn::Result<proc_macro2::TokenStream> {
///         let vis = &self.vis;
///         let name = &self.name;
///         let ty = &self.ty;
///         Ok(zyn::zyn! { {{ vis }} {{ name }}: {{ ty }}, })
///     }
/// }
/// ```
///
/// Or use the `#[zyn::element]` attribute macro to generate this automatically.
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

/// Trait for pipe transforms used via `{{ expr | pipe_name }}` in templates.
///
/// Pipes transform interpolated values before they are emitted as tokens.
/// The `Input` type is what the pipe receives (typically `String` from `.to_string()`),
/// and `Output` is what gets emitted to the token stream.
///
/// # Example
///
/// ```ignore
/// struct Prefix;
///
/// impl zyn::Pipe for Prefix {
///     type Input = String;
///     type Output = proc_macro2::Ident;
///
///     fn pipe(&self, input: String) -> proc_macro2::Ident {
///         proc_macro2::Ident::new(
///             &format!("pfx_{}", input),
///             proc_macro2::Span::call_site(),
///         )
///     }
/// }
/// ```
///
/// Or use the `#[zyn::pipe]` attribute macro to generate this automatically.
pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}

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
        let mut out = String::new();
        let chars: Vec<char> = input.chars().collect();

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

        proc_macro2::Ident::new(&out, proc_macro2::Span::call_site())
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
        let pascal = Pascal.pipe(input);
        let s = pascal.to_string();
        let mut chars = s.chars();

        let result = match chars.next() {
            None => String::new(),
            Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
        };

        proc_macro2::Ident::new(&result, proc_macro2::Span::call_site())
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
        let mut out = String::new();
        let mut capitalize = true;

        for c in input.chars() {
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

        proc_macro2::Ident::new(&out, proc_macro2::Span::call_site())
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
        let snake = Snake.pipe(input);
        proc_macro2::Ident::new(
            &snake.to_string().to_uppercase(),
            proc_macro2::Span::call_site(),
        )
    }
}
