use std::env::args;
use text_byte_file::{bytes_to_file, file_to_bytes};


fn main() {
    let argv = args().collect::<Vec<String>>();

    match argv.get(1) {
        Some(action) => {
            match action.as_str() {
                "byte" => file_to_bytes(&argv),
                "file" => bytes_to_file(&argv),
                _ => {
                    println!("not found command");
                }
            }
            println!("action = {}", action);
        }
        None => {
            println!("Not found command");
        }
    }
}
