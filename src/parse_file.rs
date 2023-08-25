use std::fs;

pub fn file(file_path: Result<String, String>) -> String {
    match file_path {
      Ok(path) => {
        match fs::read_to_string(path) {
          Ok(file) => file,
          Err(error) => format!("Error: {error}")
        }
      },
      Err(error) => format!("Error: {error}")
    }
}
