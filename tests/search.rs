#[cfg(test)]
mod tests {
    use std::error::Error;
    use regex::Regex;
    use rust_grep::*;

    #[test]
    fn single_line() -> Result<(), Box<dyn Error>> {
        let expr = Regex::new(".*duct.*")?;
        let data = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(&expr, data));

        Ok(())
    }

    #[test]
    fn multiple_lines() -> Result<(), Box<dyn Error>> {
        let expr = Regex::new(".*co.*")?;
        let data = "\
I really enjoy cold weather,
I think only cool guys enjoy winter.";

        assert_eq!(
            vec![
                (1, "I really enjoy cold weather,"),
                (2, "I think only cool guys enjoy winter.")
            ],
            search(&expr, data)
        );

        Ok(())
    }
}
