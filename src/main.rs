use std::env;
use std::process::exit;

use radix::*;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let options = Options::new(argv.iter().map(String::as_str).collect());

    match convert(options) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
