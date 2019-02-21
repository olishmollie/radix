use std::env;
use std::process;

use nconv;
use nconv::Config;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = Config::new(&argv).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match nconv::run(config) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
