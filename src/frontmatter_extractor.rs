use serde::Serialize;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub image: String,
    pub dynamic_path: String,
}

impl FrontMatter {
    pub fn from_yaml(yaml: &str) -> Result<Option<Self>, String> {
        let mut front_matter = FrontMatter {
            title: String::new(),
            description: String::new(),
            image: String::new(),
            dynamic_path: String::new(),
        };

        // Parse YAML format
        let yaml_doc = yaml
            .trim_start_matches("---")
            .trim_end_matches("---")
            .trim();
        let yaml_pairs: Vec<(&str, &str)> = yaml_doc
            .lines()
            .filter_map(|line| {
                let mut split = line.splitn(2, ": ");
                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    Some((key.trim(), value.trim()))
                } else {
                    None
                }
            })
            .collect();

        // Populate the struct
        for (key, value) in yaml_pairs {
            match key {
                "title" => front_matter.title = value.to_string(),
                "description" => front_matter.description = value.to_string(),
                "image" => front_matter.image = value.to_string(),
                _ => (),
            }
        }

        // Check if all fields are populated
        Ok(Some(front_matter))
    }
}

#[derive(Serialize)]
pub struct FranchiseData {
    pub title: String,
    pub description: String,
    pub ico_image: String,
    pub wiki_head_image: String,
    pub default_embed_image: String,
    pub franchise_proper_name: String,
    pub image: String,
    pub page_count: u64,
    pub dynamic_path: String,
}

impl FranchiseData {
    pub fn from_yaml(yaml: &str) -> Result<Option<Self>, String> {
        let mut front_matter = FranchiseData {
            title: String::new(),
            image: String::new(),
            description: String::new(),
            dynamic_path: String::new(),
            ico_image: String::new(),
            wiki_head_image: String::new(),
            default_embed_image: String::new(),
            franchise_proper_name: String::new(),
            page_count: 0,
        };

        // Parse YAML format
        let yaml_doc = yaml
            .trim_start_matches("---")
            .trim_end_matches("---")
            .trim();
        let yaml_pairs: Vec<(&str, &str)> = yaml_doc
            .lines()
            .filter_map(|line| {
                let mut split = line.splitn(2, ": ");
                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    Some((key.trim(), value.trim()))
                } else {
                    None
                }
            })
            .collect();

        // Populate the struct
        for (key, value) in yaml_pairs {
            match key {
                "title" => front_matter.title = value.to_string(),
                "image" => front_matter.image = value.to_string(),
                "description" => front_matter.description = value.to_string(),
                "ico_image" => front_matter.ico_image = value.to_string(),
                "wiki_head_image" => front_matter.wiki_head_image = value.to_string(),
                "default_embed_image" => front_matter.default_embed_image = value.to_string(),
                "franchise_proper_name" => front_matter.franchise_proper_name = value.to_string(),
                _ => (),
            }
        }

        // Check if all fields are populated
        Ok(Some(front_matter))
    }
}

#[derive(Serialize)]
pub struct CategoryData {
    pub title: String,
    pub description: String,
    pub ico_image: String,
    pub wiki_head_image: String,
    pub default_embed_image: String,
    pub page_count: u64,
}

pub fn read_file_to_string(file_path: &str) -> Option<String> {
    // Attempt to open the file
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return None, // Return None if unable to open the file
    };

    // Read the contents of the file into a String
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return None; // Return None if unable to read the file
    }

    Some(contents) // Return the contents wrapped in Some
}
