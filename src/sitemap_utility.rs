use std::fs;
mod directory_utility;
mod sitemap_writer;
use chrono::{DateTime, Utc};
const WEBSITE_DOMAIN_NAME: &str = "https://wiki.saihex.com";

pub fn generate_urls_dir(path: &str, franchise: &str) -> Result<String, String> {
    let directories = match directory_utility::get_directories_to_children(path) {
        Ok(all_the_franchise) => all_the_franchise,
        Err(_) => return Err("Failed to read directory".to_string()),
    };

    let mut urls: Vec<sitemap_writer::Url> = vec![];

    for ele in directories {
        let formatted = if ele.ends_with(".md") {
            format!("{}/{}", path, &ele)
        } else {
            format!("{}/{}/index.md", path, &ele)
        };

        let metadata = match fs::metadata(formatted) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let last_modified = {
            if let Ok(modified_time) = metadata.modified() {
                let modified_time: DateTime<Utc> = modified_time.into();
                let formatted_time = modified_time.format("%Y-%m-%d").to_string();
                formatted_time
            } else {
                continue;
            }
        };

        let mut priority = 0.8;
        
        if ele.ends_with("index.md") {
            priority =  0.5;
        }

        let url = sitemap_writer::Url {
            loc: format!("{}/wiki{}/{}", WEBSITE_DOMAIN_NAME, franchise, ele.trim_end_matches(".md")),
            lastmod: last_modified,
            changefreq: "monthly".to_string(),
            priority: priority,
        };

        urls.push(url);
    }

    let url_set = sitemap_writer::UrlSet { urls: urls };

    Ok(url_set.to_xml())
}

pub fn generate_sitemap_dir(path: &str, franchise: &str, no_twice: bool) -> Result<String, String> {
    let directories = match directory_utility::get_directories(path) {
        Ok(all_the_franchise) => all_the_franchise,
        Err(_) => return Err("Failed to read directory".to_string()),
    };

    let mut sitemaps: Vec<sitemap_writer::Sitemaps> = vec![];

    for ele in directories {
        let metadata = match fs::metadata(format!("{}/{}/index.md", path, &ele)) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let last_modified = {
            if let Ok(modified_time) = metadata.modified() {
                let modified_time: DateTime<Utc> = modified_time.into();
                let formatted_time = modified_time.format("%Y-%m-%d").to_string();
                formatted_time
            } else {
                continue;
            }
        };

        let sitemap = sitemap_writer::Sitemaps {
            loc: format!(
                "{}/api/sitemap_xml/{}{}",
                WEBSITE_DOMAIN_NAME, franchise, ele
            ),
            lastmod: last_modified.to_owned(),
        };

        let sitemap2 = sitemap_writer::Sitemaps {
            loc: format!("{}/api/sitemap/{}{}", WEBSITE_DOMAIN_NAME, franchise, ele),
            lastmod: last_modified.to_owned(),
        };

        if !no_twice {
            sitemaps.push(sitemap);
        }
        sitemaps.push(sitemap2);
    }

    let sitemap_set = sitemap_writer::SitemapSet { sitemaps: sitemaps };

    Ok(sitemap_set.to_xml())
}
