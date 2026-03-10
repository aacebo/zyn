pub fn is_enabled(ident: &str) -> bool {
    let Ok(pattern) = std::env::var("ZYN_DEBUG") else {
        return false;
    };

    pattern
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .any(|pat| wildcard_match(pat, ident))
}

fn wildcard_match(pattern: &str, value: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();

    if parts.len() == 1 {
        return pattern == value;
    }

    let mut pos = 0;

    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }

        match value[pos..].find(part) {
            Some(found) => {
                if i == 0 && found != 0 {
                    return false;
                }

                pos += found + part.len();
            }
            None => return false,
        }
    }

    if let Some(last) = parts.last()
        && !last.is_empty()
    {
        return value.ends_with(last) && pos == value.len();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    mod wildcard_match {
        use super::*;

        #[test]
        fn exact_match() {
            assert!(wildcard_match("Foo", "Foo"));
            assert!(!wildcard_match("Foo", "Bar"));
        }

        #[test]
        fn star_matches_all() {
            assert!(wildcard_match("*", "anything"));
            assert!(wildcard_match("*", ""));
        }

        #[test]
        fn prefix_wildcard() {
            assert!(wildcard_match("My*", "MyElement"));
            assert!(wildcard_match("My*", "My"));
            assert!(!wildcard_match("My*", "NotMy"));
        }

        #[test]
        fn suffix_wildcard() {
            assert!(wildcard_match("*Element", "MyElement"));
            assert!(!wildcard_match("*Element", "MyPipe"));
        }

        #[test]
        fn middle_wildcard() {
            assert!(wildcard_match("My*Pipe", "MyCustomPipe"));
            assert!(!wildcard_match("My*Pipe", "MyCustomElement"));
        }
    }
}
