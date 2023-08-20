use std::fs;
use std::env;
fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {    
        return Err("File not provided");
    }
    let file_path = &args[1];
    let content = 
            match fs::read_to_string(file_path) {
                Ok(file) => file,
                Err(error) => panic!("File open failed with error: {}", error)
            };
    println!("{}", content);

    Ok(())
}
