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
                    .map(|s| {
                        let trimmed = s.trim();
                        // Skip @-rules and comments
                        if trimmed.starts_with('@') || trimmed.starts_with("/*") {
                            trimmed.to_string()
                        } else {
                            format!("{}{}", trimmed, scope_attr)
                        }
                    })
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
