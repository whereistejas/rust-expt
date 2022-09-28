#[cfg(test)]
mod tests {
    use fancy_regex::Regex;

    #[test]
    fn it_works() {
        let regex = Regex::new("^[0-9]*$").unwrap();

        assert!(!regex.is_match("abc").unwrap());
        assert!(regex.is_match("123").unwrap());
    }
}
