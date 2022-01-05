use std::env;
use std::process;
use rust_grep::Config;

fn print_error(err: &str) {
    eprintln!("[rust_grep] error: {}", err);
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = Config::new(&argv).unwrap_or_else(|err| {
        print_error(err);
        process::exit(1);
    });

    rust_grep::run(config).unwrap_or_else(|_err| {
        print_error("Couldn't read file. Check if file has correct name and read permission.");
    });
}
