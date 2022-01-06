use std::env;
use std::error::Error;
use std::fs;
use regex::Regex;

pub struct Config {
    pub expr: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub regex: bool,
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
        let regex = env::var("NO_REGEX").is_err();

        Ok(Config {
            expr,
            filename,
            case_sensitive,
            regex,
        })
    }
}

pub fn search<'a>(expr: &Regex, data: &'a str) -> Vec<(usize, &'a str)> {
    data.lines()
        .enumerate()
        .filter(|line| expr.is_match(&line.1))
        .map(|line| (line.0 + 1, line.1))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut data = fs::read_to_string(&config.filename)?;

    let mut expr_string = match config.regex {
        true => config.expr.clone(),
        false => format!("{}{}{}", ".*", config.expr.clone(), ".*"),
    };
    if config.case_sensitive {
        expr_string = config.expr.to_lowercase();
        data = data.to_lowercase();
    }

    let expr = Regex::new(&expr_string)?;

    println!(
        "* Searching for '{}' in '{}'...",
        &expr, &config.filename
    );
    for (i_line, string) in search(&expr, &data) {
        println!("    Line {}: '{}'", i_line, string);
    }

    Ok(())
}
