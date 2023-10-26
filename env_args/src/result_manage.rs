use crate::action::{add, divided, times};
use crate::help;

pub fn show_result(action: &String, num1: &String, num2: &String) {
    match action.as_str() {
        "add" => {
            println!("{} {} to {}", action, num1, num2);
            println!("result = {}", add(&num1, &num2));
        }
        "mul" => {
            println!("{} {} to {}", action, num1, num2);
            println!("result = {}", times(&num1, &num2));
        }
        "div" => {
            println!("{} {} to {}", action, num1, num2);
            println!("result = {}", divided(&num1, &num2));
        }
        _ => {
            println!("'{}' command not found", action);
            help();
        }
    }
}
