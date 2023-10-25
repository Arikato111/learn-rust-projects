use env_args::{
    action::{divided, times},
    add,
};
use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = args().collect();

    if argv.len() > 3 {
        match argv[1].as_str() {
            "add" => {
                println!("{} {} to {}", argv[1], argv[2], argv[3]);
                println!("result = {}", add(argv[2].clone(), argv[3].clone()));
            }
            "mul" => {
                println!("{} {} to {}", argv[1], argv[2], argv[3]);
                println!("result = {}", times(argv[2].clone(), argv[3].clone()));
            }
            "div" => {
                println!("{} {} to {}", argv[1], argv[2], argv[3]);
                println!("result = {}", divided(argv[2].clone(), argv[3].clone()));
            }
            _ => {
                println!("'{}' command not found", argv[1]);
                help();
            }
        }
    } 
    if argv.len() > 1 && (argv[1] == "-h".to_string() || argv[1] == "--help".to_string()) {
        help();
    }

    Ok(())
}

fn help() {
    println!("\nexample: <action> num1 num2 | action=['add', 'mul', 'div']");
    print!("")
}
