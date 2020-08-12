use std::env;
use std::process::exit;

use radix::*;

fn main() {
    let options = Options::new(
        env::args()
            .collect::<Vec<String>>()
            .iter()
            .map(String::as_str)
            .collect(),
    );

    match convert(options) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
