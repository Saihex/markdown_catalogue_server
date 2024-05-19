use std::fs;
use std::io;
use std::path::Path;

pub fn get_directories<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let mut directories = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.to_string() != ".vscode" && name.to_string() != ".git" {
                            directories.push(name.to_string());
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(directories)
}

pub fn get_directories_to_children(path: &str) -> io::Result<Vec<String>> {
    let mut directories = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.to_string() != ".vscode" && name.to_string() != ".git" {
                            let all_the_children = match get_files(&format!("{}/{}", path, name)) {
                                Ok(children) => children,
                                Err(_) => continue,
                            };

                            directories.push(name.to_owned());

                            for child in all_the_children {
                                directories.push(format!("{}/{}", name, child));
                            }
                        }
                    }
                }

                if file_type.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.to_string().ends_with("index.md") {
                            directories.push(name.trim_end_matches("index.md").to_string());
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(directories)
}

pub fn get_files(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        if !name.to_string().ends_with("index.md") && name.ends_with(".md") {
                            files.push(name.to_string());
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(files)
}