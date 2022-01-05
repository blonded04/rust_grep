#[cfg(test)]
mod tests {
    use rust_grep::*;

    #[test]
    fn single_line() {
        let expr = "duct";
        let data = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(expr, data));
    }

    #[test]
    fn multiple_lines() {
        let expr = "co";
        let data = "\
I really enjoy cold weather,
I think only cool guys enjoy winter.";

        assert_eq!(vec![(1, "I really enjoy cold weather,"), (2, "I think only cool guys enjoy winter.")], search(expr, data));
    }
}