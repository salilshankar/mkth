use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => panic!("No file provided"),
        _ => {}
    }
    let file_path = &args[1];
    let content = 
            match fs::read_to_string(file_path) {
                Ok(file) => file,
                Err(error) => panic!("File open failed with error: {}", error)
            };

    println!("{}", content);
}
