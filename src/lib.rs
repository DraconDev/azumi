pub mod prelude {
    pub use crate::action::Action;
    pub use crate::{
        action, azumi_script, component, head, html, live, live_impl, predict, AzumiScript,
        Component,
    };
}

pub use azumi_macros::{action, component, head, html, live, live_impl, page, predict};
pub mod action;
pub mod context;
pub mod hot_reload;
pub mod script;
pub mod security;
pub use inventory;
pub mod devtools;

pub mod seo;

pub use script::AzumiScript;

#[cfg(feature = "test-utils")]
pub mod test;

pub trait Component {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

/// Marker trait for live state structs
/// Auto-implemented by #[azumi::live]
pub trait LiveState:
    serde::Serialize + for<'de> serde::de::Deserialize<'de> + Send + Sync + 'static
{
    fn to_scope(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        crate::security::sign_state(&json)
    }
    /// Returns predictions for optimistic UI (method_name -> dsl)
    fn predictions() -> &'static [(&'static str, &'static str)];

    /// Returns the struct name for namespacing actions
    fn struct_name() -> &'static str;
}

#[derive(Clone)]
pub struct FnComponent<F>(F);

impl<F> Component for FnComponent<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

impl<T: Component + ?Sized> Component for &T {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl<T: Component + ?Sized> Component for Box<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl<T: Component + ?Sized> Component for std::rc::Rc<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl<T: Component + ?Sized> Component for std::sync::Arc<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

pub fn from_fn<F>(f: F) -> FnComponent<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    FnComponent(f)
}

pub fn render_to_string<C: Component + ?Sized>(component: &C) -> String {
    struct DisplayWrapper<'a, C: Component + ?Sized>(&'a C);
    impl<'a, C: Component + ?Sized> std::fmt::Display for DisplayWrapper<'a, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.render(f)
        }
    }
    format!("{}", DisplayWrapper(component))
}

pub struct Escaped<T: std::fmt::Display>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.to_string();
        for c in s.chars() {
            match c {
                '<' => write!(f, "&lt;")?,
                '>' => write!(f, "&gt;")?,
                '&' => write!(f, "&amp;")?,
                '"' => write!(f, "&quot;")?,
                '\'' => write!(f, "&#x27;")?,
                _ => write!(f, "{}", c)?,
            }
        }
        Ok(())
    }
}

// Smart Interpolation Machinery
// Allows {} to handle both Components (render) and Display types (escape)

pub struct RenderWrapper<T>(pub T);

impl<T: Component> RenderWrapper<T> {
    // Priority 1: Component (Render directly)
    // This inherent method takes precedence over the trait implementation below
    pub fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.render(f)
    }
}

pub trait FallbackRender {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

// Priority 2: Display (Escape HTML)
impl<T: std::fmt::Display> FallbackRender for RenderWrapper<T> {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Escaped(&self.0))
    }
}

/// A wrapper to inject raw HTML/JS content without escaping.
///
/// Usage:
/// ```rust,ignore
/// use azumi::html;
/// html! {
///     <script>
///         {azumi::Raw("console.log('Hello');")}
///     </script>
/// }
/// ```
pub struct Raw<T: std::fmt::Display>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for Raw<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: std::fmt::Display> Component for Raw<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn js<T: std::fmt::Debug>(v: T) -> String {
    format!("{:?}", v)
}

/// Generate a unique scope ID for CSS scoping
pub fn generate_scope_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("s{:x}", id)
}

/// Transform CSS selectors to include scope attribute
/// All CSS is automatically scoped - no escape hatches!
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut iter = css.chars().peekable();
    scope_css_level(&mut iter, &scope_attr, false)
}

fn scope_css_level(
    iter: &mut std::iter::Peekable<std::str::Chars>,
    scope_attr: &str,
    finding_close: bool,
) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();

                if is_grouping_rule(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&scope_css_level(iter, scope_attr, true));
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                } else {
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

                    result.push_str(&scoped_selector_str);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                }
            }
            '}' => {
                if finding_close {
                    result.push_str(&buffer);
                    return result;
                } else {
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

fn extract_balanced_block(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1;
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

// ============================================================================
// Schema.org JSON-LD Support (Optional Feature)
// ============================================================================

#[cfg(feature = "schema")]
pub use azumi_macros::Schema;

#[cfg(feature = "schema")]
pub trait Schema {
    /// Generate a complete <script type="application/ld+json"> tag
    fn to_schema_script(&self) -> String;

    /// Generate just the JSON value (for recursive nesting)
    fn to_schema_json_value(&self) -> serde_json::Value;
}

#[cfg(test)]
mod tests;

// ============================================================================
// Embedded Client Runtime
// ============================================================================

/// The Azumi client library (embedded at compile time)
/// This includes Idiomorph (DOM morphing) and the Azumi coordinator
pub const AZUMI_JS: &str = include_str!("client.min.js");

/// Helper to generate the <script> tag for the client runtime
/// Usage: html! { <head> { azumi::azumi_script() } ... </head> }
pub fn azumi_script() -> String {
    format!(r#"<script>{}</script>"#, AZUMI_JS)
}

pub struct HotReloadClosure<'a>(pub &'a dyn Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result);

impl<'a> FallbackRender for HotReloadClosure<'a> {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}
