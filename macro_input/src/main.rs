use std::{
    self,
    error::Error,
    io::{self, Write},
};

macro_rules! cin {
    ($str_input:expr) => {
        print!("input (text): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut $str_input)?;
        $str_input = $str_input.trim().to_string();
    };
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_text: String = String::new();

    cin!(input_text);

    println!("input is \"{}\"", input_text);
    Ok(())
}
