use std::collections::HashSet;

/// Compute the Levenshtein edit distance between two strings.
/// Used for "did you mean?" suggestions in compile error messages.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    // Use a single row for space efficiency
    let mut prev_row: Vec<usize> = (0..=b_len).collect();
    let mut curr_row = vec![0usize; b_len + 1];

    for i in 1..=a_len {
        curr_row[0] = i;
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[b_len]
}

/// Find the closest match to `target` from a set of candidates.
/// Returns the best match if its edit distance is within `max_distance`.
/// Uses a dynamic max_distance based on target length for better suggestions.
pub fn closest_match<'a>(target: &str, candidates: &'a HashSet<String>) -> Option<&'a String> {
    if target.is_empty() || candidates.is_empty() {
        return None;
    }

    // Dynamic threshold: allow more edits for longer strings
    let max_distance = match target.len() {
        0..=3 => 1,
        4..=6 => 2,
        _ => 3,
    };

    candidates
        .iter()
        .map(|c| (c, levenshtein(target, c)))
        .filter(|(_, d)| *d <= max_distance && *d > 0)
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

/// Format an error message with a "did you mean?" suggestion if available.
pub fn suggest_fix(target: &str, candidates: &HashSet<String>) -> String {
    if let Some(suggestion) = closest_match(target, candidates) {
        format!("'{}' not found. Did you mean '{}'?", target, suggestion)
    } else {
        format!("'{}' not found.", target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein("foo", "foo"), 0);
    }

    #[test]
    fn test_levenshtein_one_edit() {
        assert_eq!(levenshtein("foo", "fo"), 1);
        assert_eq!(levenshtein("foo", "fooo"), 1);
        assert_eq!(levenshtein("foo", "boo"), 1);
    }

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
        assert_eq!(levenshtein("", ""), 0);
    }

    #[test]
    fn test_closest_match_finds_typo() {
        let mut candidates = HashSet::new();
        candidates.insert("my_button".to_string());
        candidates.insert("container".to_string());
        candidates.insert("header".to_string());

        let result = closest_match("my_buttn", &candidates);
        assert_eq!(result, Some(&"my_button".to_string()));
    }

    #[test]
    fn test_closest_match_no_match() {
        let mut candidates = HashSet::new();
        candidates.insert("container".to_string());

        let result = closest_match("xyz", &candidates);
        assert_eq!(result, None);
    }

    #[test]
    fn test_closest_match_exact_is_not_returned() {
        let mut candidates = HashSet::new();
        candidates.insert("foo".to_string());

        let result = closest_match("foo", &candidates);
        assert_eq!(result, None); // Exact match, no suggestion needed
    }

    #[test]
    fn test_suggest_fix_with_suggestion() {
        let mut candidates = HashSet::new();
        candidates.insert("my_button".to_string());
        let msg = suggest_fix("my_buttn", &candidates);
        assert!(msg.contains("Did you mean 'my_button'?"));
    }

    #[test]
    fn test_suggest_fix_no_suggestion() {
        let candidates = HashSet::new();
        let msg = suggest_fix("xyz", &candidates);
        assert!(msg.contains("not found"));
        assert!(!msg.contains("Did you mean"));
    }
}
