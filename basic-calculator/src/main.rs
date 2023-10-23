use basic_calculator::func::{cin, show_result::show};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("***********************");
    println!("Basic calculator with rust");
    println!("Example: 10 + 20");
    
    let mut st_in: String = String::new();
    cin(&mut st_in);

    let string_slice: Vec<&str> = st_in.split(" ").collect();

    if string_slice.len() != 3 {
        println!("Error input wrong");
        return Ok(());
    }

    let num1: i32 = match string_slice[0].parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0,
    };

    let num2: i32 = match string_slice[2].parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0,
    };

    show(string_slice[1], num1, num2);
    println!("********* End ************");
    Ok(())
}
