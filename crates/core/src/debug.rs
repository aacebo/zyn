use proc_macro2::TokenStream;

use crate::ast;

pub fn raw(tokens: &TokenStream) -> String {
    let raw = tokens.to_string();
    let cleaned = raw
        .replace(
            ":: zyn :: proc_macro2 :: TokenStream :: new ()",
            "TokenStream::new()",
        )
        .replace(":: zyn :: proc_macro2 :: TokenStream", "TokenStream")
        .replace(":: zyn :: quote :: quote !", "quote!")
        .replace(
            ":: zyn :: quote :: ToTokens :: to_tokens",
            "ToTokens::to_tokens",
        )
        .replace(":: zyn :: Pipe :: pipe", "Pipe::pipe")
        .replace(":: zyn :: Render :: render", "Render::render")
        .replace(":: core :: compile_error !", "compile_error!")
        .replace(":: core :: result :: Result :: Ok", "Ok")
        .replace(":: core :: result :: Result :: Err", "Err")
        .replace("__zyn_ts_0", "output")
        .replace("__zyn_ts_1", "inner_1")
        .replace("__zyn_ts_2", "inner_2")
        .replace("__zyn_ts_3", "inner_3")
        .replace("__zyn_ts_4", "inner_4")
        .replace("__zyn_val", "value")
        .replace("__zyn_rendered", "rendered")
        .replace("__zyn_expand_result", "result");

    fmt(&cleaned)
}

pub fn ast(element: &crate::template::Template) -> String {
    let mut lines = Vec::new();
    lines.push("Element [".to_string());

    for node in &element.nodes {
        let desc = match node {
            ast::Node::Tokens(t) => format!("  Tokens({:?})", t.stream.to_string()),
            ast::Node::Interp(_) => "  Interp { ... }".to_string(),
            ast::Node::At(at) => {
                let kind = if at.is_if() {
                    "If"
                } else if at.is_for() {
                    "For"
                } else if at.is_match() {
                    "Match"
                } else if at.is_element() {
                    "Element"
                } else {
                    "Unknown"
                };
                format!("  At({})", kind)
            }
            ast::Node::Group(_) => "  Group { ... }".to_string(),
        };
        lines.push(desc);
    }

    lines.push("]".to_string());
    lines.join("\n")
}

pub fn print(tokens: &TokenStream) {
    let formatted = fmt(&tokens.to_string());
    eprintln!("zyn::debug! ─── pretty\n{}", formatted);
}

pub fn fmt(input: &str) -> String {
    let mut result = String::new();
    let mut depth: usize = 0;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                result.push('{');
                result.push('\n');
                depth += 1;
                push_indent(&mut result, depth);
            }
            '}' => {
                depth = depth.saturating_sub(1);
                if result.ends_with("    ") || result.ends_with('\n') {
                    result.truncate(result.trim_end().len());
                    result.push('\n');
                }
                push_indent(&mut result, depth);
                result.push('}');

                if chars
                    .peek()
                    .is_some_and(|c| *c != ',' && *c != ';' && *c != ')')
                {
                    result.push('\n');
                    push_indent(&mut result, depth);
                }
            }
            ';' => {
                result.push(';');
                result.push('\n');
                push_indent(&mut result, depth);
            }
            ' ' if result.ends_with('\n') || result.ends_with("    ") => {}
            _ => {
                result.push(ch);
            }
        }
    }

    result.trim().to_string()
}

fn push_indent(s: &mut String, depth: usize) {
    for _ in 0..depth {
        s.push_str("    ");
    }
}
