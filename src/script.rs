use crate::Component;

/// The Azumi client library (embedded at compile time)
/// This includes Idiomorph (DOM morphing) and the Azumi coordinator
pub const AZUMI_JS: &str = include_str!("client.min.js");

/// A zero-cost component that injects the Azumi runtime script.
///
/// Usage:
/// ```rust,ignore
/// use azumi::html;
/// html! {
///     <head>
///         @AzumiScript
///     </head>
/// }
/// ```
pub struct AzumiScript;

impl Component for AzumiScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<script>{}</script>", AZUMI_JS)
    }
}
