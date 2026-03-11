/// Computes the Levenshtein edit distance between two strings.
///
/// Returns the minimum number of single-character insertions, deletions,
/// or substitutions needed to transform `a` into `b`.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();
    let mut matrix = vec![vec![0usize; b_len + 1]; a_len + 1];

    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i;
    }

    for (j, val) in matrix[0].iter_mut().enumerate() {
        *val = j;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}

/// Returns the closest matching string from `haystack` if the edit distance
/// is at most 3. Used for "did you mean?" suggestions in error messages.
pub fn closest_match<'a>(needle: &str, haystack: &[&'a str]) -> Option<&'a str> {
    haystack
        .iter()
        .map(|&s| (s, levenshtein(needle, s)))
        .filter(|&(_, d)| d <= 3)
        .min_by_key(|&(_, d)| d)
        .map(|(s, _)| s)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod levenshtein_tests {
        use super::*;

        #[test]
        fn identical_strings() {
            assert_eq!(levenshtein("hello", "hello"), 0);
        }

        #[test]
        fn single_insertion() {
            assert_eq!(levenshtein("cat", "cats"), 1);
        }

        #[test]
        fn single_deletion() {
            assert_eq!(levenshtein("cats", "cat"), 1);
        }

        #[test]
        fn single_substitution() {
            assert_eq!(levenshtein("cat", "car"), 1);
        }

        #[test]
        fn both_empty() {
            assert_eq!(levenshtein("", ""), 0);
        }

        #[test]
        fn one_empty() {
            assert_eq!(levenshtein("", "abc"), 3);
            assert_eq!(levenshtein("abc", ""), 3);
        }

        #[test]
        fn multiple_edits() {
            assert_eq!(levenshtein("kitten", "sitting"), 3);
        }
    }

    mod closest_match_tests {
        use super::*;

        #[test]
        fn finds_typo() {
            let candidates = &["name", "count", "enabled"];
            assert_eq!(closest_match("naem", candidates), Some("name"));
        }

        #[test]
        fn finds_closest_among_multiple() {
            let candidates = &["rename_all", "deny_unknown_fields", "format"];
            assert_eq!(closest_match("formt", candidates), Some("format"));
        }

        #[test]
        fn none_when_too_distant() {
            let candidates = &["name", "count"];
            assert_eq!(closest_match("zzzzzzzzz", candidates), None);
        }

        #[test]
        fn empty_haystack() {
            let candidates: &[&str] = &[];
            assert_eq!(closest_match("name", candidates), None);
        }

        #[test]
        fn exact_match() {
            let candidates = &["name", "count"];
            assert_eq!(closest_match("name", candidates), Some("name"));
        }
    }
}
