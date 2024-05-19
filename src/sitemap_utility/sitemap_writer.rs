pub struct Url {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: f32,
}
pub struct UrlSet {
    pub urls: Vec<Url>,
}

impl UrlSet {
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
        
        for url in &self.urls {
            xml.push_str("  <url>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", url.loc));
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", url.lastmod));
            xml.push_str(&format!("    <changefreq>{}</changefreq>\n", url.changefreq));
            xml.push_str(&format!("    <priority>{}</priority>\n", url.priority));
            xml.push_str("  </url>\n");
        }

        xml.push_str("</urlset>");
        xml
    }
}

pub struct Sitemaps {
    pub loc: String,
    pub lastmod: String,
}
pub struct SitemapSet {
    pub sitemaps: Vec<Sitemaps>,
}

impl SitemapSet {
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
        
        for sitemap in &self.sitemaps {
            xml.push_str("  <sitemap>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", sitemap.loc));
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", sitemap.lastmod));
            xml.push_str("  </sitemap>\n");
        }

        xml.push_str("</sitemapindex>");
        xml
    }
}