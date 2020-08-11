use std::env;
use std::process::exit;

use radix::*;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = Config::new(&argv);

    match run(config) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
