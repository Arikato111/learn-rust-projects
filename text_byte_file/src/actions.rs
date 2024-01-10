use std::fs::{self};

pub fn file_to_bytes(argv: &Vec<String>) {
    let file_path = argv.get(2).expect("not found file name input");
    let mut byte_string: String =
        (file_path.split("/").last().expect("not found file name")).to_string();
    let content = fs::read(file_path).expect("error read file");
    for b in content {
        byte_string += format!(" {:b}", b).as_str();
    }
    fs::write("./byte.txt", byte_string).expect("cannot write file");
}

pub fn bytes_to_file(argv: &Vec<String>) {
    let file_path = argv.get(2).expect("not found file name input");
    let mut file_name: &str = "";
    let content = fs::read_to_string(file_path).expect("error read file");
    let mut byte_file: Vec<u8> = Vec::new();
    let mut is_first = true;
    for b in content.split(" ") {
        if is_first {
            file_name = b;
            is_first = false;
            continue;
        }
        byte_file.push(u8::from_str_radix(b, 2).expect("error convert binary string to number"));
    }
    fs::write(file_name, byte_file).expect("cannot write file");
}
