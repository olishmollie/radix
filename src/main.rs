use std::env;
use std::process::exit;

use radix::run;
use radix::Config;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = Config::new(&argv).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    match run(config) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
