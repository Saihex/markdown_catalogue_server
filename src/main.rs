extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};
use frontmatter_extractor::FrontMatter;
use rust_search::{similarity_sort, SearchBuilder};
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

mod data_types;
mod frontmatter_extractor;
mod sitemap_utility;

const GLOBAL_COLLECTION_DIRECTORY: &str = "./collection";
const SERVER_VERSION: &str = "v0.0.2-c";

///

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening...");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/sitemap").route(web::get().to(handle_sitemap)))
            .service(web::resource("/sitemap/{franchise}").route(web::get().to(handle_sitemap_category)))
            //
            .service(web::resource("/sitemap_xml").route(web::get().to(handle_sitemap_xml)))
            .service(web::resource("/sitemap_xml/{franchise}").route(web::get().to(handle_sitemap_category)))
            //
            .service(web::resource("/{filename:.*}").route(web::get().to(handle_request)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

///

// Making the process of adding server version header or/and cache header less verbose.
pub trait HeaderManipulator {
    fn server_version_header(&mut self) -> &mut Self;
}

impl HeaderManipulator for actix_web::HttpResponseBuilder {
    fn server_version_header(&mut self) -> &mut Self {
        self.append_header(("Server-Version", SERVER_VERSION))
    }
}

// Sitemap urls

async fn handle_sitemap() -> HttpResponse {
    let franchise_urls = match sitemap_utility::generate_urls_dir(GLOBAL_COLLECTION_DIRECTORY, "") {
        Ok(franchise_urls) => franchise_urls,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .server_version_header()
                .finish();
        }
    };

    HttpResponse::Ok()
        .server_version_header()
        .append_header(("Content-Type", "text/xml"))
        .body(franchise_urls)
}

async fn handle_sitemap_category(info: web::Path<(String,)>) -> HttpResponse {
    let filename = &info.0;

    let raw_path = &format!("{}/{}", GLOBAL_COLLECTION_DIRECTORY, filename);
    let path = PathBuf::from(raw_path);

    if !path.exists() {
        return HttpResponse::NotFound().server_version_header().finish();
    };

    let franchise_urls = match sitemap_utility::generate_urls_dir(raw_path, &format!("/{}/category", filename)) {
        Ok(franchise_urls) => franchise_urls,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .server_version_header()
                .finish();
        }
    };

    HttpResponse::Ok()
        .server_version_header()
        .append_header(("Content-Type", "text/xml"))
        .body(franchise_urls)
}

// Sitemap XML
async fn handle_sitemap_xml() -> HttpResponse {
    let franchise_urls = match sitemap_utility::generate_sitemap_dir(GLOBAL_COLLECTION_DIRECTORY, "", false) {
        Ok(franchise_urls) => franchise_urls,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .server_version_header()
                .finish();
        }
    };

    HttpResponse::Ok()
        .server_version_header()
        .append_header(("Content-Type", "text/xml"))
        .body(franchise_urls)
}


//

async fn handle_request(
    info: web::Path<(String,)>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let filename = &info.0;
    let raw_path = &format!("{}/{}", GLOBAL_COLLECTION_DIRECTORY, filename);
    let path = PathBuf::from(raw_path);
    let dropped_no = String::new();

    if query.get("root_dir_search").unwrap_or(&String::default()) == "true" {
        let wiki_found =
            handle_root_directory_search(query.get("search_input").unwrap_or(&dropped_no));
        return HttpResponse::Ok()
            .server_version_header()
            .append_header(("Content-Type", "application/json"))
            .body(wiki_found.to_string());
    }

    if raw_path == &format!("{}/", GLOBAL_COLLECTION_DIRECTORY) {
        return HttpResponse::Forbidden().server_version_header().finish();
    }

    if !path.exists() {
        return HttpResponse::NotFound().server_version_header().finish();
    }

    if query.get("frontmatter_only").unwrap_or(&String::new()) == "true" {
        if path.is_file() {
            let franchise_read = handle_frontmatter(raw_path);

            match franchise_read {
                Ok(value) => {
                    return HttpResponse::Ok()
                        .server_version_header()
                        .append_header(("Content-Type", "application/json"))
                        .body(value.to_string());
                }
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .server_version_header()
                        .finish();
                }
            }
        } else {
            return HttpResponse::BadRequest()
                .server_version_header()
                .body("that was a directory man.");
        }
    }

    if query.get("category_search").unwrap_or(&String::new()) == "true" {
        if path.is_dir() {
            let search_input = query.get("search_input").unwrap_or(&dropped_no);
            let cat_read = handle_category_search(search_input, raw_path);

            match cat_read {
                Ok(value) => {
                    return HttpResponse::Ok()
                        .server_version_header()
                        .append_header(("Content-Type", "application/json"))
                        .body(value.to_string());
                }
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .server_version_header()
                        .finish();
                }
            }
        } else {
            return HttpResponse::BadRequest()
                .server_version_header()
                .body("that was a file man.");
        }
    }

    if path.is_dir() {
        let directory_search = handle_directory(
            &path,
            query.get("search_input").unwrap_or(&String::new()),
            &raw_path,
        );

        return HttpResponse::Ok()
            .server_version_header()
            .append_header(("Content-Type", "application/json"))
            .body(directory_search.to_string());
    } else if path.is_file() {
        let read_file = handle_file(raw_path);

        match read_file {
            Some(string_) => {
                return HttpResponse::Ok()
                    .server_version_header()
                    .append_header(("Content-Type", "text/plain"))
                    .body(string_);
            }
            None => {}
        }
    }

    HttpResponse::InternalServerError()
        .server_version_header()
        .finish()
}

fn handle_file(path: &str) -> Option<String> {
    match frontmatter_extractor::read_file_to_string(path) {
        Some(content) => Some(content),
        None => None,
    }
}

fn handle_category_search(search_input: &str, raw_path: &str) -> Result<serde_json::Value, u16> {
    let mut search: Vec<String> = SearchBuilder::default()
        .location(raw_path)
        .ignore_case()
        .depth(1)
        .search_input(search_input)
        .build()
        .collect();

    similarity_sort(&mut search, search_input);
    let mut categories: Vec<FrontMatter> = Vec::new();

    let mut index: u32 = 0;
    while index < search.len() as u32 {
        search[index as usize] = search[index as usize].replace("\\", "/"); // stop it windows.

        if search[index as usize] == raw_path || search[index as usize].contains("index.md") {
            search.remove(index as usize);
        } else {
            let file_data = frontmatter_extractor::read_file_to_string(&format!(
                "{}/index.md",
                search[index as usize]
            ));
            let value = match file_data {
                Some(string_data) => string_data,
                _ => continue,
            };

            let mut front_matter = match frontmatter_extractor::FrontMatter::from_yaml(&value) {
                Ok(s) => match s {
                    Some(w) => w,
                    _ => continue,
                },
                Err(_) => continue,
            };
            front_matter.dynamic_path = search[index as usize]
                .trim_start_matches(&format!("{}/", raw_path))
                .to_owned();
            categories.push(front_matter);
            index += 1;
        }
    }

    return Ok(json!(categories));
}

fn handle_frontmatter(raw_path: &str) -> Result<serde_json::Value, u16> {
    match frontmatter_extractor::read_file_to_string(raw_path) {
        Some(value) => {
            let _front_matter = frontmatter_extractor::FranchiseData::from_yaml(&value);

            let front_matter = match _front_matter {
                Ok(s) => s,
                Err(_) => {
                    return Err(500);
                }
            };

            let mut page_count_path = raw_path;

            if let Some(index) = raw_path.rfind('/') {
                // Use index to slice the string
                page_count_path = &raw_path[..index];
            }

            let search: Vec<String> = SearchBuilder::default()
                .location(page_count_path)
                .ext(".md")
                .ignore_case()
                .depth(3)
                .build()
                .collect();

            match front_matter {
                Some(mut some_more_value) => {
                    some_more_value.page_count = search.len() as u64;
                    return Ok(json!(some_more_value));
                }
                None => {
                    return Err(500);
                }
            };
        }
        None => {
            return Err(404);
        }
    }
}

fn handle_root_directory_search(search_input: &str) -> serde_json::Value {
    let mut search: Vec<String> = SearchBuilder::default()
        .location(GLOBAL_COLLECTION_DIRECTORY)
        .ignore_case()
        .search_input(search_input)
        .depth(1)
        .build()
        .collect();

    similarity_sort(&mut search, search_input);
    let mut wiki_list: Vec<frontmatter_extractor::FranchiseData> = Vec::new();

    for file_path in &mut search {
        let index_markdown = PathBuf::from(format!("{}/index.md", file_path.replace("\\", "/")));

        if index_markdown.exists() && index_markdown.is_file() {
            let file_data =
                match frontmatter_extractor::read_file_to_string(&index_markdown.to_str().unwrap())
                {
                    Some(string_data) => string_data,
                    None => continue,
                };

            let mut frontmatter_data =
                match frontmatter_extractor::FranchiseData::from_yaml(&file_data) {
                    Ok(s) => match s {
                        Some(w) => w,
                        _ => continue,
                    },
                    Err(_) => continue,
                };

            frontmatter_data.dynamic_path = file_path
                .replace("\\", "/")
                .split("/")
                .last()
                .unwrap_or("")
                .to_owned();

            wiki_list.push(frontmatter_data);
        }
    }

    json!(wiki_list)
}

fn handle_directory(path: &PathBuf, search_input: &str, raw_path: &str) -> serde_json::Value {
    let mut search: Vec<String> = SearchBuilder::default()
        .location(path)
        .limit(50)
        .ext(".md")
        .ignore_case()
        .depth(1)
        .search_input(search_input)
        .build()
        .collect();

    similarity_sort(&mut search, search_input);

    let mut catalogue_list: Vec<data_types::DirectoryLister> = Vec::new();

    for file_path in &mut search {
        match frontmatter_extractor::read_file_to_string(file_path) {
            Some(some_value) => {
                let _front_matter = frontmatter_extractor::FrontMatter::from_yaml(&some_value);

                let front_matter = match _front_matter {
                    Ok(s) => s,
                    Err(_) => None,
                };

                *file_path = file_path.replace("\\", "/"); // stop it windows.

                match front_matter {
                    Some(front_matter) => {
                        *file_path = file_path
                            .trim_start_matches(&format!("{}/", raw_path))
                            .to_string();
                        *file_path = file_path.trim_end_matches(".md").to_string();

                        let dynamic_path_clone = file_path.clone();

                        let new_data = data_types::DirectoryLister {
                            title: front_matter.title,
                            description: front_matter.description,
                            image: front_matter.image,
                            dynamic_path: dynamic_path_clone,
                            spoiler: front_matter.spoiler,
                        };

                        if file_path.clone() != "index" {
                            catalogue_list.push(new_data);
                        }
                    }
                    None => {}
                }
            }
            None => {}
        };
    }

    json!(catalogue_list)
}
