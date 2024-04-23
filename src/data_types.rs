use serde::Serialize;

#[derive(Serialize)]
pub struct DirectoryLister {
    pub dynamic_path: String,
    pub title: String,
    pub description: String,
    pub image: String,
}