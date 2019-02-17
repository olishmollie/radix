use std::env;
use std::process;

use dconv;
use dconv::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        print_error(err);
        print_usage();
        process::exit(1);
    });

    match dconv::run(config) {
        Ok(s) => println!("{}", s),
        Err(s) => print_error(s)
    }

}

pub fn print_error(msg: &str) {
    eprintln!("error: {}", msg);
}

pub fn print_usage() {
    eprintln!("Usage: dconv [options] <value>");
}
