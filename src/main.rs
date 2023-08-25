use mkth::{parse_file, std_input_handler};

fn main() {
    match std_input_handler::get_file_path() {
        Ok(path) => {
            println!("{}", parse_file::file(path.to_string()));
        },
        Err(error) => println!("Error: {error}")
    };
}
