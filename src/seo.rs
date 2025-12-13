#[cfg(feature = "seo")]
use std::fmt::Write;

#[cfg(feature = "seo")]
#[derive(Clone, Default, Debug)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
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

    pub fn with_canonical(mut self, url: impl Into<String>) -> Self {
        self.canonical_url = Some(url.into());
        self
    }

    pub fn with_open_graph(mut self, og: OpenGraph) -> Self {
        self.open_graph = Some(og);
        self
    }

    pub fn with_twitter(mut self, twitter: TwitterCard) -> Self {
        self.twitter = Some(twitter);
        self
    }

    /// Renders all meta tags as a Safe HTML string
    pub fn render_tags(&self) -> crate::Raw<String> {
        let mut html = String::new();

        // Basic
        let _ = write!(html, "<title>{}</title>", self.title);
        if let Some(desc) = &self.description {
            let _ = write!(html, r#"<meta name="description" content="{}">"#, desc);
        }
        if let Some(canon) = &self.canonical_url {
            let _ = write!(html, r#"<link rel="canonical" href="{}">"#, canon);
        }

        // OpenGraph
        if let Some(og) = &self.open_graph {
            if let Some(t) = &og.title {
                let _ = write!(html, r#"<meta property="og:title" content="{}">"#, t);
            }
            if let Some(d) = &og.description {
                let _ = write!(html, r#"<meta property="og:description" content="{}">"#, d);
            }
            if let Some(u) = &og.url {
                let _ = write!(html, r#"<meta property="og:url" content="{}">"#, u);
            }
            if let Some(i) = &og.image {
                let _ = write!(html, r#"<meta property="og:image" content="{}">"#, i);
            }
            if let Some(s) = &og.site_name {
                let _ = write!(html, r#"<meta property="og:site_name" content="{}">"#, s);
            }
        }

        // Twitter
        if let Some(tw) = &self.twitter {
            let _ = write!(html, r#"<meta name="twitter:card" content="{}">"#, tw.card);
            if let Some(s) = &tw.site {
                let _ = write!(html, r#"<meta name="twitter:site" content="{}">"#, s);
            }
            // Fallback to OG/Basic if not explicit, but allow explicit overrides
            if let Some(t) = &tw.title {
                let _ = write!(html, r#"<meta name="twitter:title" content="{}">"#, t);
            }
            if let Some(d) = &tw.description {
                let _ = write!(html, r#"<meta name="twitter:description" content="{}">"#, d);
            }
            if let Some(i) = &tw.image {
                let _ = write!(html, r#"<meta name="twitter:image" content="{}">"#, i);
            }
        }

        crate::Raw(html)
    }
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
