use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub expr: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut argv: env::Args) -> Result<Config, &'static str> {
        argv.next();

        let expr = match argv.next() {
            Some(arg) => arg,
            None => return Err("No string to search provided."),
        };

        let filename = match argv.next() {
            Some(arg) => arg,
            None => return Err("No file provided."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            expr,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(expr: &str, data: &'a str) -> Vec<(usize, &'a str)> {
    // TODO: Add regex.
    data.lines()
        .enumerate()
        .filter(|line| line.1.contains(&expr))
        .map(|line| (line.0 + 1, line.1))
        .collect()
}

pub fn search_insensitive<'a>(expr: &str, data: &'a str) -> Vec<(usize, &'a str)> {
    // TODO: Add regex.
    let expr = expr.to_lowercase();

    data.lines()
        .enumerate()
        .filter(|line| line.1.to_lowercase().contains(&expr))
        .map(|line| (line.0 + 1, line.1))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(&config.filename)?;

    let found;
    if config.case_sensitive {
        found = search(&config.expr, &data);
    } else {
        found = search_insensitive(&config.expr, &data);
    }

    println!(
        "* Searching for '{}' in '{}'...",
        &config.expr, &config.filename
    );
    for (i_line, string) in found {
        println!("    Line {}: '{}'", i_line, string);
    }

    Ok(())
}
