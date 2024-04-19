use serde::Serialize;

#[derive(Serialize)]
pub struct DirectoryLister {
    pub dynamic_route: String,
    pub title: String,
    pub description: String,
    pub image: String,
}