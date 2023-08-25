use std::env;

use mkth::{parse_file, std_input_handler};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Err("File not provided");
    }
    let file_path = std_input_handler::parse(&args);
    let content = parse_file::file(file_path);

    println!("{content}");

    Ok(())
}
