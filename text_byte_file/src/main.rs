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
        }
        None => {
            help();
        }
    }
}

fn help() {
    println!(r"
byfi - bytes to file or file to bytes

Usage:
    byfi [COMMAND] [OPTION] FILE_NAME

Command:
    byte                        convert file to bytes text.
    file                        convert bytes text to file.

Option:
    -b [base]                   ex. `byfi -b 16 file.png` 
                                [base] = 2 | 8 | 16.
                                convert to 2 8 or 16 base.

Example:
    byfi byte file.png          convert `file.png` to bytes text.
                                output is `file.png.txt`.
    byfi file file.png.txt      convert back to file.
    
")
}