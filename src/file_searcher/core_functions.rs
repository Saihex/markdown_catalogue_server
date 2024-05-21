use std::fs::{self, DirEntry};
use std::path::Path;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strsim::levenshtein;

pub fn any_in_vec<T: PartialEq>(me: &T, vec: &Vec<T>) -> bool {
    for ele in vec {
        if *ele == *me {
            return true;
        }
    }
    false
}

pub fn path_string_human_sensible(str: &str) -> String {
    str.replace("_", " ").trim_end_matches(".md").to_owned()
}

pub fn calculate_string_similarity(s1: &str, s2: &str) -> u8 {
    let binding = s1.to_lowercase();
    let s1 = binding.as_str();

    let binding2 = s2.to_lowercase();
    let s2 = binding2.as_str();

    let max_len = s1.len().max(s2.len());
    if max_len == 0 {
        return 255; // Both strings are empty, they are equal
    }

    if s1 == s2 {
        return 255; // Both strings are equal, they are equal
    }

    if s1.contains(s2) || s2.contains(s1) {
        return 255; // One string contains the other, they are equal
    }

    let lev_distance = levenshtein(&s1, &s2);
    let similarity = 255 - ((lev_distance * 255) / max_len).min(255);
    similarity as u8
}

pub fn list_raw_entries(root_path: &str, exclude: &Vec<&str>) -> Result<Vec<DirEntry>, String> {
    let parent = Path::new(root_path);

    if !parent.is_dir() {
        return Err("NOT DIRECTORY".to_string());
    }

    let entries = match fs::read_dir(parent) {
        Ok(w) => w,
        Err(_) => return Err("Failed to read directory".to_string()),
    };

    let directories: Vec<DirEntry> = entries
        .par_bridge()
        .filter_map(|entry| entry.ok())  
        .filter(|entry| {
            if let Some(name) = entry.file_name().to_str() {
                !any_in_vec(&name.to_string().as_str(), exclude)
            } else {
                false
            }
        })
        .collect();

    Ok(directories)
}
