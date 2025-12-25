use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

/// Transform CSS selectors to include scope attribute
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut iter = css.chars().peekable();
    scope_css_recursive(&mut iter, &scope_attr)
}

fn scope_css_recursive(iter: &mut Peekable<Chars>, scope_attr: &str) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();
                buffer.clear();

                // Check if this is a grouping rule (recurse) or style rule (scope)
                if is_grouping_rule(&selector_raw) {
                    result.push_str(&selector_raw);
                    result.push_str(" {");
                    // Recurse into the block
                    // We need to pass the iterator which is now inside the block
                    // We need to call scope_css_recursive until we hit '}' matched to this level?
                    // No, scope_css_recursive consumes until end of stream.
                    // But we want to consume only ONE block.
                    // Actually, we can just recurse. The recursive call will return when it finds a closing brace?
                    // We need to architect this so the recursive function processes a sequence of rules.
                    // It stops when it hits `}` (if it was called for a block) or EOF.

                    let inner_content = scope_css_level(iter, scope_attr, true); // true = stop at '}'
                    result.push_str(&inner_content);
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&selector_raw);
                    result.push_str(" {");
                    // Keyframes content (0% { ... }) should NOT be scoped
                    // Just copy balanced block
                    let content = extract_balanced_block(iter);
                    result.push_str(&content);
                    result.push('}');
                } else {
                    // Style Rule - Scope the selector
                    // But skip @font-face etc which are not grouping rules but also not style rules with selectors?
                    // @font-face { src: ... }
                    // scope_selector handles @ check.

                    // Split by comma for multiple selectors
                    let selectors: Vec<&str> = selector_raw.split(',').collect();
                    let scoped: Vec<String> = selectors
                        .iter()
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| scope_selector(s.trim(), scope_attr))
                        .collect();

                    if !scoped.is_empty() {
                        result.push_str(&scoped.join(", "));
                    } else {
                        // e.g. @font-face
                        result.push_str(&selector_raw);
                    }

                    result.push_str(" {");
                    // Content is properties, just copy balanced block
                    let content = extract_balanced_block(iter);
                    result.push_str(&content);
                    result.push('}');
                }
            }
            '}' => {
                // Determine if this closes the current level
                // In scope_css_level, we check this.
                // But here we are iterating.
                // If we hit '}', it means the block we were processing has ended.
                // We should put it back? Or return?
                // We need a helper that consumes.
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
    // Append remaining buffer (whitespace etc)
    result.push_str(&buffer);
    result
}

// Helper that processes rules until it sees a closing brace (if finding_close=true) or EOF
fn scope_css_level(iter: &mut Peekable<Chars>, scope_attr: &str, finding_close: bool) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();

                if is_grouping_rule(&selector_raw) {
                    result.push_str(&buffer); // Keep original whitespace/selector
                    result.push('{');
                    buffer.clear();
                    // Recurse
                    result.push_str(&scope_css_level(iter, scope_attr, true));
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                } else {
                    // Scope selectors
                    // We need to preserve the whitespace in buffer before the selector?
                    // buffer contains the selector.
                    let scoped_selector_str = if selector_raw.starts_with('@') {
                        selector_raw.to_string()
                    } else {
                        let selectors: Vec<&str> = selector_raw.split(',').collect();
                        selectors
                            .iter()
                            .filter(|s| !s.trim().is_empty())
                            .map(|s| scope_selector(s.trim(), scope_attr))
                            .collect::<Vec<_>>()
                            .join(", ")
                    };

                    // We replace buffer content with scoped selector
                    // But try to keep formatting? Naive replacement is fine for minified CSS.
                    result.push_str(&scoped_selector_str);
                    result.push('{');
                    buffer.clear();

                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                }
            }
            '}' => {
                if finding_close {
                    // We found the closing brace for this level
                    // Return everything accumulated so far (excluding the })
                    // The caller will append '}'
                    result.push_str(&buffer);
                    return result;
                } else {
                    // Stray } or logic error, just append
                    buffer.push(ch);
                }
            }
            ';' => {
                buffer.push(ch);
                result.push_str(&buffer);
                buffer.clear();
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
    result.push_str(&buffer);
    result
}

fn is_grouping_rule(s: &str) -> bool {
    s.starts_with("@media")
        || s.starts_with("@supports")
        || s.starts_with("@layer")
        || s.starts_with("@container")
}

fn is_keyframes(s: &str) -> bool {
    s.starts_with("@keyframes") || s.starts_with("@-webkit-keyframes")
}

fn extract_balanced_block(iter: &mut Peekable<Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1; // We already passed the opening '{'

    for ch in iter.by_ref() {
        match ch {
            '{' => {
                depth += 1;
                content.push(ch);
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return content;
                }
                content.push(ch);
            }
            _ => content.push(ch),
        }
    }
    content
}

/// Transform CSS selectors by renaming classes with a suffix (CSS Modules style)
/// e.g. .container -> .container-xyz
#[allow(dead_code)]
pub fn rename_css_selectors(css: &str, suffix: &str) -> String {
    // Keep original implementation or upgrade? Use scope_css logic ideally but focused on renaming.
    // For now, not touched as not primarily used (Azumi uses attribute scoping).
    let mut result = String::new();
    let mut in_rule = false;
    let mut selector_buffer = String::new();

    let mut chars = css.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' if !in_rule => {
                let selectors: Vec<&str> = selector_buffer.split(',').collect();
                let scoped: Vec<String> = selectors
                    .iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| rename_selector(s.trim(), suffix))
                    .collect();

                result.push_str(&scoped.join(", "));
                result.push_str(" {");
                selector_buffer.clear();
                in_rule = true;
            }
            '}' if in_rule => {
                result.push('}');
                in_rule = false;
            }
            '/' if !in_rule => {
                if let Some(&'*') = chars.peek() {
                    result.push('/');
                    result.push(chars.next().unwrap());
                    while let Some(c) = chars.next() {
                        result.push(c);
                        if c == '*' {
                            if let Some(&'/') = chars.peek() {
                                result.push(chars.next().unwrap());
                                break;
                            }
                        }
                    }
                } else {
                    selector_buffer.push(ch);
                }
            }
            _ => {
                if in_rule {
                    result.push(ch);
                } else {
                    selector_buffer.push(ch);
                }
            }
        }
    }

    if !selector_buffer.trim().is_empty() {
        result.push_str(&selector_buffer);
    }

    result
}

fn rename_selector(selector: &str, suffix: &str) -> String {
    if selector.starts_with('@') {
        return selector.to_string();
    }
    let mut result = String::new();
    let mut chars = selector.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '.' {
            result.push('.');
            let mut class_name = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    class_name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !class_name.is_empty() {
                result.push_str(&class_name);
                result.push('-');
                result.push_str(suffix);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn scope_selector(selector: &str, scope_attr: &str) -> String {
    if selector.starts_with('@') || selector.starts_with("/*") {
        return selector.to_string();
    }
    if let Some(pseudo_pos) = selector.find("::") {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }
    if let Some(pseudo_pos) = selector.find(':') {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }
    format!("{}{}", selector, scope_attr)
}

/// Extract all defined class names and IDs from CSS content
pub fn extract_selectors(css: &str) -> (HashSet<String>, HashSet<String>) {
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    // Use recursive extractor
    let mut iter = css.chars().peekable();
    extract_selectors_recursive(&mut iter, &mut classes, &mut ids, false);
    (classes, ids)
}

fn extract_selectors_recursive(
    iter: &mut Peekable<Chars>,
    classes: &mut HashSet<String>,
    ids: &mut HashSet<String>,
    finding_close: bool,
) {
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();
                if is_grouping_rule(&selector_raw) {
                    buffer.clear();
                    extract_selectors_recursive(iter, classes, ids, true);
                } else if is_keyframes(&selector_raw) {
                    buffer.clear();
                    // Consume balanced block without extracting
                    let _ = extract_balanced_block(iter);
                } else {
                    // Extract from selectors
                    process_selectors(&selector_raw, classes, ids);
                    buffer.clear();
                    // Consume balanced block (properties)
                    let _ = extract_balanced_block(iter);
                }
            }
            '}' => {
                if finding_close {
                    return;
                }
                // Ignore stray
            }
            '/' => {
                // Skip comments
                if let Some(&'*') = iter.peek() {
                    iter.next();
                    while let Some(c) = iter.next() {
                        if c == '*' {
                            if let Some(&'/') = iter.peek() {
                                iter.next();
                                break;
                            }
                        }
                    }
                } else {
                    buffer.push(ch);
                }
            }
            '"' | '\'' => {
                // Skip strings
                let quote = ch;
                for c in iter.by_ref() {
                    if c == quote {
                        break;
                    }
                }
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
}

fn process_selectors(buffer: &str, classes: &mut HashSet<String>, ids: &mut HashSet<String>) {
    for selector in buffer.split(',') {
        let selector = selector.trim();
        if selector.is_empty() || selector.starts_with('@') || selector.starts_with("/*") {
            continue;
        }

        let mut chars = selector.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '.' {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !name.is_empty() {
                    classes.insert(name);
                }
            } else if ch == '#' {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !name.is_empty() {
                    ids.insert(name);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_css_nested_media() {
        let css = "@media (max-width: 768px) { .center_zone { display: none !important; } }";
        let scope_id = "s123";
        let scoped = scope_css(css, scope_id);

        // Expected: @media (max-width: 768px) { .center_zone[data-s123] { display: none !important; } }
        assert!(
            scoped.contains(".center_zone[data-s123]"),
            "Actual: {}",
            scoped
        );
        assert!(scoped.contains("@media (max-width: 768px)"));
    }

    #[test]
    fn test_scope_css_nested_media_complex() {
        let css = "@media (min-width: 1024px) { .foo { color: red; } .bar { color: blue; } }";
        let scope_id = "xyz";
        let scoped = scope_css(css, scope_id);

        assert!(scoped.contains(".foo[data-xyz]"));
        assert!(scoped.contains(".bar[data-xyz]"));
    }

    #[test]
    fn test_extract_selectors_nested() {
        let css = "@media screen { .foo { color: red; } } .bar { color: blue; }";
        let (classes, _ids) = extract_selectors(css);

        assert!(classes.contains("foo"));
        assert!(classes.contains("bar"));
    }
}
