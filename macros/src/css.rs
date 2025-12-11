use std::collections::HashSet;

/// Transform CSS selectors to include scope attribute
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut result = String::new();
    let mut in_rule = false;
    let mut selector_buffer = String::new();

    for ch in css.chars() {
        match ch {
            '{' if !in_rule => {
                // Found opening brace - scope the selector buffer
                let selectors: Vec<&str> = selector_buffer.split(',').collect();
                let scoped: Vec<String> = selectors
                    .iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| scope_selector(s.trim(), &scope_attr))
                    .collect();

                result.push_str(&scoped.join(", "));
                result.push('{');
                selector_buffer.clear();
                in_rule = true;
            }
            '}' if in_rule => {
                result.push('}');
                in_rule = false;
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

    // Handle any remaining selector buffer (malformed CSS)
    if !selector_buffer.trim().is_empty() {
        result.push_str(&selector_buffer);
    }

    result
}

/// Transform CSS selectors by renaming classes with a suffix (CSS Modules style)
/// e.g. .container -> .container-xyz
#[allow(dead_code)]
pub fn rename_css_selectors(css: &str, suffix: &str) -> String {
    let mut result = String::new();
    let mut in_rule = false;
    let mut selector_buffer = String::new();

    // Simple tokenizer to handle strings and comments would be better,
    // but for now we reuse the char loop approach
    let mut chars = css.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' if !in_rule => {
                // Found opening brace - process the selector buffer
                let selectors: Vec<&str> = selector_buffer.split(',').collect();
                let scoped: Vec<String> = selectors
                    .iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| rename_selector(s.trim(), suffix))
                    .collect();

                result.push_str(&scoped.join(", "));
                result.push_str(" {"); // Add space before opening brace
                selector_buffer.clear();
                in_rule = true;
            }
            '}' if in_rule => {
                result.push('}');
                in_rule = false;
            }
            '/' if !in_rule => {
                // Check for comment
                if let Some(&'*') = chars.peek() {
                    result.push('/');
                    result.push(chars.next().unwrap()); // *
                                                        // Skip until */
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

    // Handle any remaining selector buffer (malformed CSS)
    if !selector_buffer.trim().is_empty() {
        result.push_str(&selector_buffer);
    }

    result
}

fn rename_selector(selector: &str, suffix: &str) -> String {
    // Skip @-rules
    if selector.starts_with('@') {
        return selector.to_string();
    }

    // We need to find all class names in the selector and append the suffix
    // e.g. "div.container > .item:hover" -> "div.container-suffix > .item-suffix:hover"

    let mut result = String::new();
    let mut chars = selector.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '.' {
            // Potential class start
            result.push('.');

            // Read class name
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
    // Skip @-rules and comments
    if selector.starts_with('@') || selector.starts_with("/*") {
        return selector.to_string();
    }

    // Handle pseudo-elements (::before, ::after, etc.)
    if let Some(pseudo_pos) = selector.find("::") {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }

    // Handle pseudo-classes (:hover, :focus, etc.)
    if let Some(pseudo_pos) = selector.find(':') {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }

    // Default: append scope attribute
    format!("{}{}", selector, scope_attr)
}

/// Extract all defined class names and IDs from CSS content
pub fn extract_selectors(css: &str) -> (HashSet<String>, HashSet<String>) {
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    let mut in_rule = false;
    let mut selector_buffer = String::new();

    // Strip comments and strings first
    let mut clean_css = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '*' {
                    // Start of comment, skip until end
                    chars.next(); // consume '*'
                    while let Some(c) = chars.next() {
                        if c == '*' {
                            if let Some(&end_ch) = chars.peek() {
                                if end_ch == '/' {
                                    chars.next(); // consume '/'
                                    break;
                                }
                            }
                        }
                    }
                    continue;
                }
            }
            clean_css.push(ch);
        } else if ch == '"' || ch == '\'' {
            // Start of string, skip until end quote
            let quote = ch;
            clean_css.push(quote); // Keep opening quote

            while let Some(c) = chars.next() {
                if c == quote {
                    clean_css.push(c); // Keep closing quote
                    break;
                }
                // Replace content with space to avoid parsing selectors inside strings
                clean_css.push(' ');
            }
        } else {
            clean_css.push(ch);
        }
    }

    // Simple parser to extract selectors
    // This is a basic implementation and might need refinement for complex CSS
    for ch in clean_css.chars() {
        match ch {
            '{' if !in_rule => {
                // Process selectors in buffer
                process_selectors(&selector_buffer, &mut classes, &mut ids);
                selector_buffer.clear();
                in_rule = true;
            }
            '}' if in_rule => {
                in_rule = false;
            }
            _ => {
                if !in_rule {
                    selector_buffer.push(ch);
                }
            }
        }
    }

    (classes, ids)
}

fn process_selectors(buffer: &str, classes: &mut HashSet<String>, ids: &mut HashSet<String>) {
    // Split by comma for multiple selectors
    for selector in buffer.split(',') {
        let selector = selector.trim();
        if selector.is_empty() || selector.starts_with('@') || selector.starts_with("/*") {
            continue;
        }

        // Split by whitespace and combinators to get individual parts
        // e.g. "div.container > .item" -> ["div.container", ">", ".item"]
        // We just want to find .class and #id in the string

        let mut chars = selector.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '.' {
                // Start of class
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
                // Start of ID
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
