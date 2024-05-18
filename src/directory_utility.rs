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
