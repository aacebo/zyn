use zyn_core::proc_macro2::TokenStream;

pub fn prettify_raw(tokens: &TokenStream) -> String {
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

    zyn_core::debug::format_tokens(&cleaned)
}

pub fn prettify_ast(element: &zyn_core::ast::Element) -> String {
    let mut lines = Vec::new();
    lines.push("Element [".to_string());

    for node in &element.nodes {
        let desc = match node {
            zyn_core::ast::Node::Tokens(t) => format!("  Tokens({:?})", t.stream.to_string()),
            zyn_core::ast::Node::Interp(_) => "  Interp { ... }".to_string(),
            zyn_core::ast::Node::At(at) => {
                let kind = if at.is_if() {
                    "If"
                } else if at.is_for() {
                    "For"
                } else if at.is_match() {
                    "Match"
                } else if at.is_throw() {
                    "Throw"
                } else if at.is_warn() {
                    "Warn"
                } else if at.is_note() {
                    "Note"
                } else if at.is_help() {
                    "Help"
                } else if at.is_element() {
                    "Element"
                } else {
                    "Unknown"
                };
                format!("  At({})", kind)
            }
            zyn_core::ast::Node::Group(_) => "  Group { ... }".to_string(),
        };
        lines.push(desc);
    }

    lines.push("]".to_string());
    lines.join("\n")
}
