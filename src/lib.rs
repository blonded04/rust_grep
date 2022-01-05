use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub expr: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub regex: bool,
}

impl Config {
    pub fn new(argv: &[String]) -> Result<Config, &str> {
        if argv.len() < 3 {
            return Err("Expected at least 2 arguments.");
        }

        let expr = argv[1].clone();
        let filename = argv[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let regex = env::var("NO_REGEX").is_err();

        Ok(Config { expr, filename, case_sensitive, regex })
    }
}

pub fn search<'a>(expr: &str, data: &'a str) -> Vec<(usize, &'a str)> {  // TODO: Add regex.
    let mut res = vec![];
    for (i_line, string) in data.lines().enumerate() {
        if string.contains(expr) {
            res.push((i_line + 1, string));
        }
    }

    res
}

pub fn search_insensitive<'a>(expr: &str, data: &'a str) -> Vec<(usize, &'a str)> {
    let expr = expr.to_lowercase();

    let mut res = vec![];
    for (i_line, string) in data.lines().enumerate() {
        if string.to_lowercase().contains(&expr) {
            res.push((i_line + 1, string));
        }
    }

    res
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(&config.filename)?;

    let found;
    if config.case_sensitive {
        found = search(&config.expr, &data);
    } else {
        found = search_insensitive(&config.expr, &data);
    }

    println!("* Searching for '{}' in '{}'...", &config.expr, &config.filename);
    for (i_line, string) in found {
        println!("    Line {}: '{}'", i_line, string);
    }

    Ok(())
}
