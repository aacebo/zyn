use proc_macro2::TokenStream;

pub fn pretty(tokens: &TokenStream) -> String {
    // Normalize `TokenStream::to_string()` spacing before pretty-printing.
    let raw = tokens
        .to_string()
        .replace("{ {", "{{ ")
        .replace("} }", " }}")
        .replace(" < ", "<")
        .replace(" >", ">")
        .replace("& ", "&")
        .replace("! (", "!(");

    // `prettyplease` requires valid Rust. Replace `{{...}}` placeholders with
    // valid sentinel identifiers, format, then restore the originals.
    let (sanitized, placeholders) = extract_placeholders(&raw);

    let formatted = match syn::parse_str::<syn::File>(&sanitized) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => sanitized,
    };

    restore_placeholders(formatted, &placeholders)
}

/// Replace each `{{...}}` span with `__zyn_N__` and record the original text.
fn extract_placeholders(raw: &str) -> (String, Vec<String>) {
    let mut result = String::new();
    let mut placeholders = Vec::new();
    let mut rest = raw;

    while let Some(start) = rest.find("{{") {
        result.push_str(&rest[..start]);
        rest = &rest[start + 2..];

        if let Some(end) = rest.find("}}") {
            let inner = rest[..end].trim().to_string();
            let idx = placeholders.len();
            placeholders.push(format!("{{{{ {inner} }}}}"));
            result.push_str(&format!("__zyn_{idx}__"));
            rest = &rest[end + 2..];
        } else {
            result.push_str("{{");
        }
    }

    result.push_str(rest);
    (result, placeholders)
}

/// Restore sentinel identifiers back to their original `{{...}}` form.
fn restore_placeholders(mut s: String, placeholders: &[String]) -> String {
    for (i, original) in placeholders.iter().enumerate() {
        s = s.replace(&format!("__zyn_{i}__"), original);
    }
    s
}
