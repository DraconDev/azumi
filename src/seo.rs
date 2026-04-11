use std::fmt::Write;
use std::sync::OnceLock;

/// Escape a string for safe inclusion in an HTML attribute value (double-quoted).
/// Prevents XSS by escaping `"`, `<`, `>`, `&`, and `'`.
fn html_attr_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape a string for safe inclusion as HTML text content.
/// Escapes `<`, `>`, and `&`.
fn html_text_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
    out
}

static SITE_CONFIG: OnceLock<SeoConfig> = OnceLock::new();

#[derive(Clone, Default, Debug)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
    pub type_: Option<String>,
}

#[derive(Clone, Default, Debug)]
pub struct TwitterCard {
    pub card: String,            // "summary", "summary_large_image"
    pub site: Option<String>,    // @username
    pub creator: Option<String>, // @username
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Default, Debug)]
pub struct SeoConfig {
    pub title: String,
    pub description: Option<String>,
    pub canonical_url: Option<String>,
    pub base_url: Option<String>, // Global base URL for canonical/og generation
    pub open_graph: Option<OpenGraph>,
    pub twitter: Option<TwitterCard>,
}

impl SeoConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        let img = image.into();
        // Update both OG and Twitter if they exist or create them
        let mut og = self.open_graph.unwrap_or_default();
        og.image = Some(img.clone());
        self.open_graph = Some(og);

        let mut tw = self.twitter.unwrap_or_default();
        tw.image = Some(img);
        self.twitter = Some(tw);
        self
    }
}

/// Initialize the global SEO configuration.
/// This should be called once at application startup.
pub fn init_seo(config: SeoConfig) {
    if SITE_CONFIG.set(config).is_err() {
        eprintln!("WARNING: init_seo() called multiple times - first initialization preserved");
    }
}

/// Generates the full HTML string for <head> meta tags.
/// automatically inferring missing fields from Global Defaults and Request Context.
pub fn generate_head(
    title: &str,
    description: Option<&str>,
    image: Option<&str>,
    url: Option<&str>,
    type_: Option<&str>,
) -> crate::Raw<String> {
    let global = SITE_CONFIG.get();

    // 0. Resolve Context (ThreadLocal override from #[azumi::page])
    let context_meta = crate::context::get_page_meta();

    // Resolve effective title/desc/image
    // Priority: Explicit Arg > Context > Global Default
    let effective_title = if !title.is_empty() {
        title.to_string()
    } else {
        context_meta.title.unwrap_or_default()
    };

    let effective_desc = description
        .map(|s| s.to_string())
        .or(context_meta.description)
        .or(global.and_then(|g| g.description.clone()));

    let effective_image = image
        .map(|s| s.to_string())
        .or(context_meta.image)
        .or(global.and_then(|g| g.open_graph.as_ref().and_then(|og| og.image.clone())));

    // 1. Merge Title with Site Name
    let full_title = if let Some(g) = global {
        if let Some(og) = &g.open_graph {
            if let Some(site_name) = &og.site_name {
                if !effective_title.is_empty() {
                    format!("{} | {}", effective_title, site_name)
                } else {
                    site_name.clone()
                }
            } else {
                effective_title.clone()
            }
        } else {
            effective_title.clone()
        }
    } else {
        effective_title.clone()
    };

    // 4. Infer Canonical / Current URL from Context
    let current_path = crate::context::get_current_path();
    let base_url = global.and_then(|g| g.base_url.as_deref());

    // Resolve effective URL
    // Priority: Explicit Arg > Context (inferred from Base + Path) > Global Default (Base)
    let full_url = if let Some(u) = url {
        Some(u.to_string())
    } else {
        match (base_url, &current_path) {
            (Some(base), Some(path)) => {
                let base_clean = base.trim_end_matches('/');
                let path_clean = if let Some(stripped) = path.strip_prefix('/') {
                    stripped
                } else {
                    path
                };
                Some(format!("{}/{}", base_clean, path_clean))
            }
            (Some(base), None) => Some(base.to_string()),
            _ => None,
        }
    };

    // Resolve effective Type
    let effective_type = type_.unwrap_or("website");

    // 5. Build Output (all values escaped to prevent XSS)
    let mut html = String::new();

    let safe_title = html_text_escape(&full_title);
    let safe_desc = effective_desc.as_deref().map(html_attr_escape);
    let safe_url = full_url.as_deref().map(html_attr_escape);
    let safe_image = effective_image.as_deref().map(html_attr_escape);

    // Basic Tags
    let _ = write!(html, "<title>{}</title>", safe_title);
    if let Some(d) = &safe_desc {
        let _ = write!(html, r#"<meta name="description" content="{}">"#, d);
    }
    if let Some(url) = &safe_url {
        let _ = write!(html, r#"<link rel="canonical" href="{}">"#, url);
    }

    // OpenGraph
    // Use global defaults for site_name, etc.
    if let Some(g) = global {
        if let Some(og) = &g.open_graph {
            // Title
            let _ = write!(
                html,
                r#"<meta property="og:title" content="{}">"#,
                safe_title
            );

            // Description
            if let Some(d) = &safe_desc {
                let _ = write!(html, r#"<meta property="og:description" content="{}">"#, d);
            }

            // URL
            if let Some(u) = &safe_url {
                let _ = write!(html, r#"<meta property="og:url" content="{}">"#, u);
            }

            // Image
            if let Some(i) = &safe_image {
                let _ = write!(html, r#"<meta property="og:image" content="{}">"#, i);
            }

            // Site Name (Always from global)
            if let Some(s) = &og.site_name {
                let safe_s = html_attr_escape(s);
                let _ = write!(
                    html,
                    r#"<meta property="og:site_name" content="{}">"#,
                    safe_s
                );
            }

            // Type
            let safe_type = html_attr_escape(effective_type);
            let _ = write!(html, r#"<meta property="og:type" content="{}">"#, safe_type);
        }
    }

    // Twitter
    if let Some(g) = global {
        if let Some(tw) = &g.twitter {
            let safe_card = html_attr_escape(&tw.card);
            let _ = write!(
                html,
                r#"<meta name="twitter:card" content="{}">"#,
                safe_card
            );
            if let Some(s) = &tw.site {
                let safe_s = html_attr_escape(s);
                let _ = write!(html, r#"<meta name="twitter:site" content="{}">"#, safe_s);
            }
            if let Some(c) = &tw.creator {
                let safe_c = html_attr_escape(c);
                let _ = write!(
                    html,
                    r#"<meta name="twitter:creator" content="{}">"#,
                    safe_c
                );
            }
            let _ = write!(
                html,
                r#"<meta name="twitter:title" content="{}">"#,
                safe_title
            );
            if let Some(d) = &safe_desc {
                let _ = write!(html, r#"<meta name="twitter:description" content="{}">"#, d);
            }
            if let Some(i) = &safe_image {
                let _ = write!(html, r#"<meta name="twitter:image" content="{}">"#, i);
            }
        }
    }

    crate::Raw(html)
}

/// Helper function to automatically render SEO tags based on current context.
/// Call this inside your Layout's <head>.
pub fn render_automatic_seo() -> crate::Raw<String> {
    // Pass empty strings to force reading from Context/Global
    generate_head("", None, None, None, None)
}

/// Simple Sitemap Builder
pub struct SitemapBuilder {
    base_url: String,
    urls: Vec<String>,
}

impl SitemapBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            urls: Vec::new(),
        }
    }

    pub fn add_url(mut self, path: impl Into<String>) -> Self {
        self.urls.push(path.into());
        self
    }

    pub fn build(self) -> String {
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
        );

        let base = self.base_url.trim_end_matches('/');

        for path in self.urls {
            let url = if path.starts_with("http") {
                path
            } else {
                format!(
                    "{}{}{}",
                    base,
                    if path.starts_with('/') { "" } else { "/" },
                    path
                )
            };

            let _ = write!(
                xml,
                "  <url>\n    <loc>{}</loc>\n    <changefreq>weekly</changefreq>\n  </url>\n",
                url
            );
        }

        xml.push_str("</urlset>");
        xml
    }
}
