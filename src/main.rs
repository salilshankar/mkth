use std::env;

mod parse_file;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {    
        return Err("File not provided");
    }
    let file_path = parse_input(&args); 
    let content = parse_file::file_content(file_path);
    
    println!("{content}");

    Ok(())
}

fn parse_input(args: &[String]) -> &str {
    let file_path = &args[1];

    file_path
}