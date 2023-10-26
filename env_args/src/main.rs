use env_args::{help, result_manage::show_result};
use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = args().collect();

    if argv.len() > 3 {
        show_result(argv[1].clone(), argv[2].clone(), argv[3].clone());
    }
    if argv.len() > 1 && (argv[1] == "-h".to_string() || argv[1] == "--help".to_string()) {
        help();
    }
    Ok(())
}
