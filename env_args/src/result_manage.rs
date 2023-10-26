use crate::action::{add, divided, times};
use crate::help;

pub fn show_result(action: String, num1: String, num2: String) {
    match action.as_str() {
        "add" => {
            println!("{} {} to {}", action, num1, num2);
            println!("result = {}", add(num1.clone(), num2.clone()));
        }
        "mul" => {
            println!("{} {} to {}", action, num1, num2);
            println!("result = {}", times(num1.clone(), num2.clone()));
        }
        "div" => {
            println!("{} {} to {}", action, num1, num2);
            println!("result = {}", divided(num1.clone(), num2.clone()));
        }
        _ => {
            println!("'{}' command not found", action);
            help();
        }
    }
}
