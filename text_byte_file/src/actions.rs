use crate::convert_byte::to_base_string;
use std::fs::{self};

pub fn file_to_bytes(argv: &Vec<String>) {
    let mut base_type: u8 = 2;
    let mut file_path: String = String::new();
    // check argv : base type [ 2 , 4 , 8 , 16 ]
    for i in 2..argv.len() {
        let tmp = argv.get(i).expect("error argv").to_string();
        if tmp == "--base" || tmp == "-b" {
            // get base value
            base_type = argv
                .get(i + 1)
                .expect("error in `file_to_bytes` find value of base argv")
                .parse::<u8>()
                .expect("error at `file_to_bytes` convert argv to string");
            // check base value
            if base_type != 2 && base_type != 8 && base_type != 16 {
                println!("Error base type [2, 8, 16]");
                return;
            }
            // get path file from next positions.
            if let Some(v) = argv.get(i + 2) {
                file_path = v.clone();
            }
            break;
        } else {
            // get path file from current position.
            file_path = argv.get(i).expect("cannot get argv").clone();
        }
    }

    // write file name in first position of text file.
    let file_name: String = (file_path.split("/").last().expect("not found file name")).to_string();
    let mut byte_string: String = file_name.clone();
    // save base type to text file.
    byte_string += format!(" {}", base_type).as_str();
    println!("base type {}", byte_string);
    let content = fs::read(file_path).expect("error read file");
    byte_string += to_base_string(&content, base_type).as_str();
    fs::write(format!("./{}.txt", file_name), byte_string).expect("cannot write file");
}

pub fn bytes_to_file(argv: &Vec<String>) {
    let file_path = argv.get(2).expect("not found file name input");
    let mut base_type: u8 = 2;
    let mut file_name: &str = "";
    let content = fs::read_to_string(file_path).expect("error read file");
    let mut byte_file: Vec<u8> = Vec::new();
    let mut index = 0;
    for b in content.split(" ") {
        if index == 0 {
            file_name = b;
            index += 1;
            continue;
        } else if index == 1 {
            base_type = b
                .parse::<u8>()
                .expect("error cannot convert base type to u8");
            index += 1;
            continue;
        }
        byte_file.push(u8::from_str_radix(b, base_type as u32).expect("error convert binary string to number"));
    }
    fs::write(file_name, byte_file).expect("cannot write file");
}
