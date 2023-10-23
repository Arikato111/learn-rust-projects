use std::io::{self, Write};

pub fn cin(dest: &mut String) {
    print!("input: ");
    io::stdout().flush().unwrap();
    let mut buff: String = String::new();
    match io::stdin().read_line(&mut buff) {
        Ok(_) => {
            buff = buff.trim().to_string();
            *dest = buff;
        }
        Err(_) => println!("Error some input"),
    }
}
