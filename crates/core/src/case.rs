/// Converts a string to PascalCase.
///
/// Handles snake_case, camelCase, PascalCase, and SCREAMING_SNAKE_CASE inputs.
/// First normalizes to snake_case to detect word boundaries, then capitalizes each word.
pub fn to_pascal(s: &str) -> String {
    let snake = to_snake(s);
    let mut out = String::new();
    let mut capitalize = true;

    for c in snake.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(c.to_uppercase());
            capitalize = false;
        } else {
            out.push(c);
        }
    }

    out
}

/// Converts a string to snake_case.
pub fn to_snake(s: &str) -> String {
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

/// Converts a string to camelCase.
pub fn to_camel(s: &str) -> String {
    let pascal = to_pascal(s);
    let mut chars = pascal.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

/// Converts a string to SCREAMING_SNAKE_CASE.
pub fn to_screaming(s: &str) -> String {
    to_snake(s).to_uppercase()
}

/// Converts a string to kebab-case.
pub fn to_kebab(s: &str) -> String {
    to_snake(s).replace('_', "-")
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
