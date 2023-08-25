use std::fs;

pub fn file_content(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(error) => format!("error: {}", error)
    }
}
