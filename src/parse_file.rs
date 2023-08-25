use std::fs;

pub fn file(file_path: Result<String, String>) -> String {
  let content = 
    match file_path {
      Ok(path) => read_file(path),
      Err(error) => format!("Error: {error}")
    };

  return content
}

fn read_file(path: String) -> String {
  return 
    match fs::read_to_string(path) {
      Ok(file) => file,
      Err(error) => format!("Error: {error}")
    }
}
