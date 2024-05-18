use serde::Serialize;
use serde_xml_rs::ser::to_string;
use std::{error::Error, vec};

const WEBSITE_DOMAIN_NAME: &str = "https://wiki.saihex.com";

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "url")]
pub struct Url {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: f32,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "urlset")]
pub struct UrlSet {
    #[serde(rename = "url")]
    pub urls: Vec<Url>,
}

impl UrlSet {
    pub fn generate_sitemap(
        urls: Vec<String>,
        path_before_it: &str,
    ) -> Result<String, Box<dyn Error>> {
        // Construct URLs for each franchise
        // let mut url_objects = Vec::new();
        // for url in urls {
        //     url_objects.push(Url {
        //         loc: format!("{}/{}/{}", WEBSITE_DOMAIN_NAME, path_before_it, url),
        //         lastmod: "2024-05-17".to_string(),
        //         changefreq: "monthly".to_string(),
        //         priority: 0.8,
        //     });
        // }

        // // Create the UrlSet
        // let urlset = UrlSet {
        //     urls: url_objects,
        // };

        let url1 = Url {
            loc: "https://example.com/page1".to_string(),
            lastmod: "2024-05-19".to_string(),
            changefreq: "weekly".to_string(),
            priority: 0.8,
        };
    
        let url2 = Url {
            loc: "https://example.com/page2".to_string(),
            lastmod: "2024-05-18".to_string(),
            changefreq: "monthly".to_string(),
            priority: 0.6,
        };
    
        let urlset = UrlSet {
            urls: vec![url1, url2],
        };

        // Serialize UrlSet to XML string
        let xml_string = match to_string(&urlset) {
            Ok(s) => s,
            Err(err) => {
                println!("{}", err);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to serialize UrlSet to XML",
                )));
            }
        };

        Ok(xml_string)
    }
}
