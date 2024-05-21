use crate::frontmatter_extractor;

pub fn extract_title(markdown_path: &str) -> Option<String> {
    let yaml = match frontmatter_extractor::read_file_to_string(markdown_path) {
        Some(string_data) => string_data,
        None => return None,
    };

    let out_str;
    
    if yaml.contains("franchise_proper_name") {
        let frontmatter = match frontmatter_extractor::FranchiseData::from_yaml(&yaml) {
            Ok(s) => match s {
                Some(w) => w,
                _ => return None,
            },
            Err(_) => return None,
        };

        out_str = frontmatter.franchise_proper_name;
    } else {
        let frontmatter = match frontmatter_extractor::FrontMatter::from_yaml(&yaml) {
            Ok(s) => match s {
                Some(w) => w,
                _ => return None,
            },
            Err(_) => return None,
        };

        out_str = frontmatter.title;
    }

    Some(out_str)
}