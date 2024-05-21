use serde::Serialize;
use std::time::SystemTime;
use std::vec::Vec;
use std::fs::{self, File};
use std::io::Read;

#[derive(Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub image: String,
    pub dynamic_path: String,
    pub spoiler: bool,
    pub last_modified: u64,
}

impl FrontMatter {
    pub fn from_yaml(yaml: &str) -> Result<Option<Self>, String> {
        let mut front_matter = FrontMatter {
            title: String::new(),
            description: String::new(),
            image: String::new(),
            dynamic_path: String::new(),
            spoiler: false,
            last_modified: 0,
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
                "title" => front_matter.title = value.to_string().remove_quotes(),
                "description" => front_matter.description = value.to_string().remove_quotes(),
                "image" => front_matter.image = value.to_string().remove_quotes(),
                "spoiler" => front_matter.spoiler = value.to_string().remove_quotes() == "true",
                _ => (),
            }
        }

        // Check if all fields are populated
        Ok(Some(front_matter))
    }
}

// pub fn cut_off_string_safety(value: String) -> String {
//     value.trim_start_matches('"').trim_end_matches('"').to_owned()
// }

trait CutOffStringSafety {
    fn remove_quotes(&self) -> String;
}

impl CutOffStringSafety for String {
    fn remove_quotes(&self) -> String {
        self.trim_start_matches('"').trim_end_matches('"').to_string()
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
    pub saihex_creation: bool,
    pub last_modified: u64
}

impl FranchiseData {
    pub fn from_yaml(yaml: &str) -> Result<Option<Self>, String> {
        let mut front_matter = FranchiseData {
            title: String::new(),
            image: String::new(),
            description: String::new(),
            dynamic_path: String::new(),
            ico_image: String::new(),
            saihex_creation: false,
            wiki_head_image: String::new(),
            default_embed_image: String::new(),
            franchise_proper_name: String::new(),
            page_count: 0,
            last_modified: 0,
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
                "title" => front_matter.title = value.to_string().remove_quotes(),
                "image" => front_matter.image = value.to_string().remove_quotes(),
                "description" => front_matter.description = value.to_string().remove_quotes(),
                "ico_image" => front_matter.ico_image = value.to_string().remove_quotes(),
                "saihex_creation" => front_matter.saihex_creation = value.to_string().remove_quotes() == "true",
                "wiki_head_image" => front_matter.wiki_head_image = value.to_string().remove_quotes(),
                "default_embed_image" => front_matter.default_embed_image = value.to_string().remove_quotes(),
                "franchise_proper_name" => front_matter.franchise_proper_name = value.to_string().remove_quotes(),
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

pub fn get_last_modified_seconds(file_path: &str) -> u64 {
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified_time) = metadata.modified() {
            if let Ok(duration_since_epoch) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                return duration_since_epoch.as_secs();
            }
        }
    }
    0 // Return 0 if any operation fails
}