extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};
use frontmatter_extractor::FrontMatter;
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

    if query.get("frontmatter_only").unwrap_or(&String::new()) == "true" {
        if path.is_file() {
            let franchise_read = handle_frontmatter(raw_path);

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
        } else {
            return HttpResponse::BadRequest().body("that was a directory man.");
        }
    }

    if query.get("category_search").unwrap_or(&String::new()) == "true" {
        if path.is_dir() {
            let dropped_no = String::new();
            let search_input = query.get("search_input").unwrap_or(&dropped_no);
            let cat_read = handle_category_search(search_input, raw_path);

            match cat_read {
                Ok(value) => {
                    return HttpResponse::Ok()
                        .append_header(("Content-Type", "application/json"))
                        .body(value.to_string());
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            }
        } else {
            return HttpResponse::BadRequest().body("that was a file man.");
        }
    }

    if path.is_dir() {
        let directory_search = handle_directory(
            &path,
            query.get("search_input").unwrap_or(&String::new()),
            &raw_path,
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
