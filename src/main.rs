extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};
use rust_search::{similarity_sort, SearchBuilder};
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

mod data_types;
mod frontmatter_extractor;

const GLOBAL_COLLECTION_DIRECTORY: &str = "./collection";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening...");
    HttpServer::new(|| {
        App::new().service(web::resource("/{filename:.*}").route(web::get().to(handle_request)))
        // Route for image resizing
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn handle_request(
    info: web::Path<(String,)>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let filename = &info.0;
    let raw_path = &format!("{}/{}", GLOBAL_COLLECTION_DIRECTORY, filename);
    let path = PathBuf::from(raw_path);

    if !path.exists() {
        return HttpResponse::NotFound().finish();
    }

    if path.is_dir() && query.get("franchise").unwrap_or(&String::new()) == "true" {
        let franchise_read = handle_franchise_data(&format!("{}/index.md", raw_path));

        match franchise_read {
            Ok(value) => {
                return HttpResponse::Ok()
                    .append_header(("Content-Type", "application/json"))
                    .body(value.to_string());
            }
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    if path.is_dir() {
        let directory_search = handle_directory(
            &path,
            query.get("dir_search").unwrap_or(&String::new()),
            &filename,
            &raw_path
        );

        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(directory_search.to_string());
    } else if path.is_file() {
        let read_file = handle_file(raw_path);

        match read_file {
            Some(string_) => {
                return HttpResponse::Ok()
                    .append_header(("Content-Type", "text/plain"))
                    .body(string_);
            }
            None => {}
        }
    }

    HttpResponse::InternalServerError().finish()
}

fn handle_file(path: &str) -> Option<String> {
    match frontmatter_extractor::read_file_to_string(path) {
        Some(content) => Some(content),
        None => None,
    }
}

fn handle_franchise_data(path: &str) -> Result<serde_json::Value, u16> {
    match frontmatter_extractor::read_file_to_string(path) {
        Some(value) => {
            let _front_matter = frontmatter_extractor::FranchiseData::from_yaml(&value);

            let front_matter = match _front_matter {
                Ok(s) => s,
                Err(_) => {
                    return Err(500);
                }
            };

            match front_matter {
                Some(some_more_value) => {
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

fn handle_directory(path: &PathBuf, search_input: &str, filename: &str, raw_path: &str) -> serde_json::Value {
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

                *file_path = file_path.replace("\\","/"); // stop it windows.

                match front_matter {
                    Some(front_matter) => {
                        *file_path = file_path
                            .trim_start_matches(&format!("{}/", raw_path))
                            .to_string();
                        *file_path = file_path.trim_end_matches(".md").to_string();

                        let dynamic_route_clone = file_path.clone();

                        let new_data = data_types::DirectoryLister {
                            title: front_matter.title,
                            description: front_matter.description,
                            image: front_matter.image,
                            dynamic_route: dynamic_route_clone,
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
