//! Case conversion functions and macros.
//!
//! zyn ships its own case conversion — no dependency on `heck`.
//!
//! # Functions
//!
//! ```ignore
//! use zyn_core::case;
//!
//! assert_eq!(case::to_snake("HelloWorld"),   "hello_world");
//! assert_eq!(case::to_pascal("hello_world"), "HelloWorld");
//! assert_eq!(case::to_camel("hello_world"),  "helloWorld");
//! assert_eq!(case::to_screaming("fooBar"),   "FOO_BAR");
//! assert_eq!(case::to_kebab("FooBar"),       "foo-bar");
//! ```
//!
//! # Macros
//!
//! ```ignore
//! // String form
//! let s: String = zyn_core::snake!("HelloWorld");
//! // → "hello_world"
//!
//! // Ident form (preserves span)
//! let id: syn::Ident = zyn_core::pascal!(my_ident => ident);
//! // my_ident = `foo_bar` → `FooBar`
//! ```

/// Converts a string to PascalCase.
///
/// Handles snake_case, camelCase, PascalCase, and SCREAMING_SNAKE_CASE inputs.
pub fn to_pascal(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut capitalize = true;

    for i in 0..bytes.len() {
        let c = bytes[i];

        if c == b'_' {
            if !out.is_empty() {
                capitalize = true;
            }
        } else if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && bytes[i - 1].is_ascii_lowercase();
            let next_lower = i + 1 < bytes.len() && bytes[i + 1].is_ascii_lowercase();
            let prev_upper = i > 0 && bytes[i - 1].is_ascii_uppercase();

            if capitalize || prev_lower || (next_lower && prev_upper) {
                out.push(c as char);
            } else {
                out.push((c + 32) as char);
            }

            capitalize = false;
        } else if capitalize {
            out.push((c - 32) as char);
            capitalize = false;
        } else {
            out.push(c as char);
        }
    }

    out
}

/// Converts a string to snake_case.
pub fn to_snake(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + 4);

    for i in 0..bytes.len() {
        let c = bytes[i];

        if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && bytes[i - 1].is_ascii_lowercase();
            let next_lower = i + 1 < bytes.len() && bytes[i + 1].is_ascii_lowercase();
            let prev_upper = i > 0 && bytes[i - 1].is_ascii_uppercase();

            if prev_lower || (next_lower && prev_upper) {
                out.push('_');
            }

            out.push((c + 32) as char);
        } else if c == b'_' {
            if !out.is_empty() && !out.ends_with('_') {
                out.push('_');
            }
        } else {
            out.push(c as char);
        }
    }

    out
}

/// Converts a string to camelCase.
pub fn to_camel(s: &str) -> String {
    let mut pascal = to_pascal(s);

    if let Some(first) = pascal.as_bytes().first()
        && first.is_ascii_uppercase()
    {
        unsafe {
            pascal.as_bytes_mut()[0] = first + 32;
        }
    }

    pascal
}

/// Converts a string to SCREAMING_SNAKE_CASE.
pub fn to_screaming(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + 4);

    for i in 0..bytes.len() {
        let c = bytes[i];

        if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && bytes[i - 1].is_ascii_lowercase();
            let next_lower = i + 1 < bytes.len() && bytes[i + 1].is_ascii_lowercase();
            let prev_upper = i > 0 && bytes[i - 1].is_ascii_uppercase();

            if prev_lower || (next_lower && prev_upper) {
                out.push('_');
            }

            out.push(c as char);
        } else if c == b'_' {
            if !out.is_empty() && !out.ends_with('_') {
                out.push('_');
            }
        } else {
            out.push((c - 32) as char);
        }
    }

    out
}

/// Converts a string to kebab-case.
pub fn to_kebab(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + 4);

    for i in 0..bytes.len() {
        let c = bytes[i];

        if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && bytes[i - 1].is_ascii_lowercase();
            let next_lower = i + 1 < bytes.len() && bytes[i + 1].is_ascii_lowercase();
            let prev_upper = i > 0 && bytes[i - 1].is_ascii_uppercase();

            if prev_lower || (next_lower && prev_upper) {
                out.push('-');
            }

            out.push((c + 32) as char);
        } else if c == b'_' {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
        } else {
            out.push(c as char);
        }
    }

    out
}

/// Converts a string or ident to PascalCase.
///
/// # Usage
///
/// - `pascal!("hello_world")` → `"HelloWorld"` (`String`)
/// - `pascal!(ident => ident)` → PascalCase `syn::Ident`
/// - `pascal!(token_stream => token_stream)` → PascalCase last ident in path
#[macro_export]
macro_rules! pascal {
    ($ident:expr => ident) => {
        syn::Ident::new(
            &$crate::case::to_pascal(&$ident.to_string()),
            $ident.span(),
        )
    };
    ($ts:expr => token_stream) => {{
        let __tokens: Vec<proc_macro2::TokenTree> = $ts.clone().into_iter().collect();
        let mut __out = proc_macro2::TokenStream::new();

        for (i, __tt) in __tokens.iter().enumerate() {
            match __tt {
                proc_macro2::TokenTree::Ident(__ident) => {
                    let __is_last_ident = !__tokens[i + 1..]
                        .iter()
                        .any(|t| matches!(t, proc_macro2::TokenTree::Ident(_)));

                    if __is_last_ident {
                        quote::ToTokens::to_tokens(
                            &$crate::pascal!(__ident => ident),
                            &mut __out,
                        );
                    } else {
                        quote::ToTokens::to_tokens(__ident, &mut __out);
                    }
                }
                __other => {
                    quote::ToTokens::to_tokens(__other, &mut __out);
                }
            }
        }

        __out
    }};
    ($s:expr) => {
        $crate::case::to_pascal($s)
    };
}

/// Converts a string or ident to snake_case.
///
/// - `snake!("HelloWorld")` → `"hello_world"` (`String`)
/// - `snake!(ident => ident)` → snake_case `syn::Ident`
#[macro_export]
macro_rules! snake {
    ($ident:expr => ident) => {
        syn::Ident::new(&$crate::case::to_snake(&$ident.to_string()), $ident.span())
    };
    ($s:expr) => {
        $crate::case::to_snake($s)
    };
}

/// Converts a string or ident to camelCase.
///
/// - `camel!("hello_world")` → `"helloWorld"` (`String`)
/// - `camel!(ident => ident)` → camelCase `syn::Ident`
#[macro_export]
macro_rules! camel {
    ($ident:expr => ident) => {
        syn::Ident::new(&$crate::case::to_camel(&$ident.to_string()), $ident.span())
    };
    ($s:expr) => {
        $crate::case::to_camel($s)
    };
}

/// Converts a string or ident to SCREAMING_SNAKE_CASE.
///
/// - `screaming!("HelloWorld")` → `"HELLO_WORLD"` (`String`)
/// - `screaming!(ident => ident)` → SCREAMING_SNAKE_CASE `syn::Ident`
#[macro_export]
macro_rules! screaming {
    ($ident:expr => ident) => {
        syn::Ident::new(
            &$crate::case::to_screaming(&$ident.to_string()),
            $ident.span(),
        )
    };
    ($s:expr) => {
        $crate::case::to_screaming($s)
    };
}

/// Converts a string or ident to kebab-case.
///
/// - `kebab!("HelloWorld")` → `"hello-world"` (`String`)
#[macro_export]
macro_rules! kebab {
    ($s:expr) => {
        $crate::case::to_kebab($s)
    };
}
