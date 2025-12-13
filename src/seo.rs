#[cfg(feature = "seo")]
use std::fmt::Write;
#[cfg(feature = "seo")]
use std::sync::OnceLock;

#[cfg(feature = "seo")]
static SITE_CONFIG: OnceLock<SeoConfig> = OnceLock::new();

#[cfg(feature = "seo")]
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

#[cfg(feature = "seo")]
#[derive(Clone, Default, Debug)]
pub struct TwitterCard {
    pub card: String,            // "summary", "summary_large_image"
    pub site: Option<String>,    // @username
    pub creator: Option<String>, // @username
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[cfg(feature = "seo")]
#[derive(Clone, Default, Debug)]
pub struct SeoConfig {
    pub title: String,
    pub description: Option<String>,
    pub canonical_url: Option<String>,
    pub base_url: Option<String>, // Global base URL for canonical/og generation
    pub open_graph: Option<OpenGraph>,
    pub twitter: Option<TwitterCard>,
}

#[cfg(feature = "seo")]
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
#[cfg(feature = "seo")]
pub fn init_seo(config: SeoConfig) {
    let _ = SITE_CONFIG.set(config);
}

/// Generates the full HTML string for <head> meta tags.
/// automatically inferring missing fields from Global Defaults and Request Context.
#[cfg(feature = "seo")]
pub fn generate_head(
    title: &str,
    description: Option<&str>,
    image: Option<&str>,
) -> crate::Raw<String> {
    let global = SITE_CONFIG.get();

    // 1. Merge Title
    // If global has a title template like "%s | MySite", we could use it here.
    // For now, we append site name if available.
    let full_title = if let Some(g) = global {
        if let Some(og) = &g.open_graph {
            if let Some(site_name) = &og.site_name {
                format!("{} | {}", title, site_name)
            } else {
                title.to_string()
            }
        } else {
            title.to_string()
        }
    } else {
        title.to_string()
    };

    // 2. Resolve Description
    let desc = description.or(global.and_then(|g| g.description.as_deref()));

    // 3. Resolve Image
    let img =
        image.or(global.and_then(|g| g.open_graph.as_ref().and_then(|og| og.image.as_deref())));

    // 4. Infer Canonical / Current URL from Context
    let current_path = crate::context::get_current_path();
    let base_url = global.and_then(|g| g.base_url.as_deref());

    let full_url = match (base_url, &current_path) {
        (Some(base), Some(path)) => {
            let base_clean = base.trim_end_matches('/');
            let path_clean = if path.starts_with('/') {
                &path[1..]
            } else {
                path
            };
            Some(format!("{}/{}", base_clean, path_clean))
        }
        (::std::option::Option::Some(base), ::std::option::Option::None) => Some(base.to_string()),
        _ => None,
    };

    // 5. Build Output
    let mut html = String::new();

    // Basic Tags
    let _ = write!(html, "<title>{}</title>", full_title);
    if let Some(d) = desc {
        let _ = write!(html, r#"<meta name="description" content="{}">"#, d);
    }
    if let Some(url) = &full_url {
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
                full_title
            );

            // Description
            if let Some(d) = desc {
                let _ = write!(html, r#"<meta property="og:description" content="{}">"#, d);
            }

            // URL
            if let Some(u) = &full_url {
                let _ = write!(html, r#"<meta property="og:url" content="{}">"#, u);
            }

            // Image
            if let Some(i) = img {
                let _ = write!(html, r#"<meta property="og:image" content="{}">"#, i);
            }

            // Site Name (Always from global)
            if let Some(s) = &og.site_name {
                let _ = write!(html, r#"<meta property="og:site_name" content="{}">"#, s);
            }

            // Type (Default to website)
            let type_ = og.type_.as_deref().unwrap_or("website");
            let _ = write!(html, r#"<meta property="og:type" content="{}">"#, type_);
        }
    }

    // Twitter
    if let Some(g) = global {
        if let Some(tw) = &g.twitter {
            let _ = write!(html, r#"<meta name="twitter:card" content="{}">"#, tw.card);
            if let Some(s) = &tw.site {
                let _ = write!(html, r#"<meta name="twitter:site" content="{}">"#, s);
            }
            if let Some(c) = &tw.creator {
                let _ = write!(html, r#"<meta name="twitter:creator" content="{}">"#, c);
            }
            // Fallbacks for title/desc/image if not explicitly overridden in twitter object
            let _ = write!(
                html,
                r#"<meta name="twitter:title" content="{}">"#,
                full_title
            ); // Simplified: Always use page title
            if let Some(d) = desc {
                let _ = write!(html, r#"<meta name="twitter:description" content="{}">"#, d);
            }
            if let Some(i) = img {
                let _ = write!(html, r#"<meta name="twitter:image" content="{}">"#, i);
            }
        }
    }

    crate::Raw(html)
}

/// Simple Sitemap Builder
#[cfg(feature = "seo")]
pub struct SitemapBuilder {
    base_url: String,
    urls: Vec<String>,
}

#[cfg(feature = "seo")]
impl SitemapBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            urls: Vec::new(),
        }
    }

    pub fn add(mut self, path: impl Into<String>) -> Self {
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
                    "{}{}",
                    base,
                    if path.starts_with('/') {
                        &path[..]
                    } else {
                        &format!("/{}", path)
                    }
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
