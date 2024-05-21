use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{fs, path::Path};

mod core_functions;
mod saihex_wiki_formatter;

// might be useful in the future.
// pub fn list_raw_directories(root_path: &str, exclude: &Vec<&str>) -> Result<Vec<String>, String> {
//     let entries = match core_functions::list_raw_entries(root_path, exclude) {
//         Ok(s) => s,
//         Err(_) => return Err("Failed to read directory".to_string()),
//     };

//     let mut directories: Vec<String> = entries
//         .par_iter() // Convert to a parallel iterator
//         .filter_map(|entry| {
//             if entry.path().is_dir() {
//                 entry.file_name().to_str().map(|name| name.to_string())
//             } else {
//                 None
//             }
//         })
//         .collect();

//     directories.sort();

//     Ok(directories)
// }

pub fn search_directories(
    root_path: &str,
    exclude: &Vec<&str>,
    search_input: &str,
    typo_tolerance: u8,
) -> Result<Vec<String>, String> {
    let entries = match core_functions::list_raw_entries(root_path, exclude) {
        Ok(s) => s,
        Err(_) => return Err("Failed to read directory".to_string()),
    };

    let mut directories: Vec<String> = entries
        .par_iter() // Convert to a parallel iterator
        .filter_map(|entry| {
            if entry.path().is_dir() {
                entry.file_name().to_str().and_then(|name| {
                    let page_title = {
                        let human_sensible = core_functions::path_string_human_sensible(name);
                        let formatted = &format!("{}/{}/index.md", root_path, name);
                        let title = saihex_wiki_formatter::extract_title(&formatted);

                        match title {
                            Some(s) => s,
                            None => human_sensible,
                        }
                    };

                    if core_functions::calculate_string_similarity(&page_title, search_input)
                        >= typo_tolerance
                    {
                        Some(name.to_string())
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .collect();

    directories.sort();

    Ok(directories)
}

pub fn search_files(
    root_path: &str,
    exclude: &Vec<&str>,
    search_input: &str,
    typo_tolerance: u8,
) -> Result<Vec<String>, String> {
    let entries = match core_functions::list_raw_entries(root_path, exclude) {
        Ok(s) => s,
        Err(_) => return Err("Failed to read directory".to_string()),
    };

    let mut directories: Vec<String> = entries
        .par_iter() // Convert to a parallel iterator
        .filter_map(|entry| {
            if entry.path().is_file() {
                entry.file_name().to_str().and_then(|name| {
                    let page_title = {
                        let human_sensible = core_functions::path_string_human_sensible(name);
                        let formatted = &format!("{}/{}", root_path, name);
                        let title: Option<String> = saihex_wiki_formatter::extract_title(&formatted);

                        match title {
                            Some(s) => s,
                            None => human_sensible,
                        }
                    };

                    if core_functions::calculate_string_similarity(&page_title, search_input)
                        >= typo_tolerance && name.ends_with(".md") && name != "index.md"
                    {
                        Some(name.to_string())
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .collect();

    directories.sort();

    Ok(directories)
}

pub fn count_files(root: &str) -> u64 {
    // Convert the root directory to a Path object
    let root_path = Path::new(root);

    // Initialize a counter for the files
    let mut file_count = 0;

    // Create a stack for directories to visit
    let mut dirs_to_visit = vec![root_path.to_path_buf()];

    // Loop while there are directories to visit
    while let Some(current_dir) = dirs_to_visit.pop() {
        // Read the directory entries
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        // If the entry is a file, increment the counter
                        file_count += 1;
                    } else if entry_path.is_dir() {
                        // If the entry is a directory, add it to the stack
                        dirs_to_visit.push(entry_path);
                    }
                }
            }
        }
    }

    // Return the total count of files
    file_count
}