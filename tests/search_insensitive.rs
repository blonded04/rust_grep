#[cfg(test)]
mod tests {
    use rust_grep::*;

    #[test]
    fn multiple_lines() {
        let expr = "r";
        let data = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(1, "Rust:"), (2, "safe, fast, productive."), (3, "Pick three.")], search_insensitive(expr, data));
    }
}