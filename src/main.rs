use std::env;
use std::process;

use dconv;
use dconv::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        dconv::print_error(err);
        dconv::print_usage();
        process::exit(1);
    });

    dconv::run(config);

}
