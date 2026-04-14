#[cfg(test)]
mod tests {
    #[test]
    fn debug_sitemap() {
        let sitemap = azumi::seo::SitemapBuilder::new("https://example.com")
            .add_url("/a/b/c/../../d")
            .build();
        println!("SITEMAP: {}", sitemap);
        assert!(false, "SITEMAP: {}", sitemap);
    }
}
