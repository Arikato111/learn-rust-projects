use find_subnet_mask::{bin_to_int, get_subnet, help, show_version, subnet_format};
use std::env::args;

fn main() {
    let argv: Vec<String> = args().collect();
    let mut i;

    match argv.get(1) {
        Some(v) => match v.as_str() {
            "--version" | "-v" => show_version(),
            "-h" => help(),
            _ => {
                println!("");
                for arg in argv.iter().skip(1) {
                    i = match arg.parse::<u8>() {
                        Ok(v) => {
                            if v > 32 {
                                println!("{} is Error max 32", arg);
                                continue;
                            }
                            v
                        }
                        Err(_) => {
                            println!("{} is Error input", arg);
                            continue;
                        }
                    };
                    let su = subnet_format(get_subnet(i as u8).to_string());
                    println!("/{} = {} | {}", i, su, bin_to_int(&su));
                }
            }
        },
        None => help(),
    }
}
