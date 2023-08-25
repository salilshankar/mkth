use std::env;

pub fn get_file_path() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Err("No file provided at stdin.".to_string());
    }
    let file_path = &args[1];

    Ok(file_path.to_string())
}
