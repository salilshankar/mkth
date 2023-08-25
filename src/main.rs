use mkth::{parse_file, std_input_handler};

fn main() {
  let file_path = std_input_handler::get_file_path();
  let content = parse_file::file(file_path);
  println!{"{content}"}
}
