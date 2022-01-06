use rust_grep::Config;
use std::env;
use std::process;

fn print_error(err: &str) {
    eprintln!("[rust_grep] error: {}", err);
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        print_error(err);
        process::exit(1);
    });

    rust_grep::run(config).unwrap_or_else(|_err| {
        print_error("Couldn't read file or string-to-search.");
    });
}
