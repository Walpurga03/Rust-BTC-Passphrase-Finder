#[cfg(test)]
mod tests {
    use rust_btc_passphrase_finder::regex_parser::RegexExpander;

    #[test]
    fn test_simple_character_class() {
        let expander = RegexExpander::new(1000);
        let result = expander.expand_pattern("[0-9]").unwrap();
        assert_eq!(result.len(), 10);
        assert!(result.contains(&"0".to_string()));
        assert!(result.contains(&"9".to_string()));
    }

    #[test]
    fn test_fixed_repetition() {
        let expander = RegexExpander::new(1000);
        let result = expander.expand_pattern("[0-9]{2}").unwrap();
        assert_eq!(result.len(), 100);
        assert!(result.contains(&"00".to_string()));
        assert!(result.contains(&"99".to_string()));
    }

    #[test]
    fn test_alternation() {
        let expander = RegexExpander::new(1000);
        let result = expander.expand_pattern("a(b|c)d").unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"abd".to_string()));
        assert!(result.contains(&"acd".to_string()));
    }

    #[test]
    fn test_complex_pattern() {
        let expander = RegexExpander::new(1000);
        let result = expander.expand_pattern("[A-C][0-9]{2}").unwrap();
        assert_eq!(result.len(), 300); // 3 letters * 100 two-digit numbers
        assert!(result.contains(&"A00".to_string()));
        assert!(result.contains(&"C99".to_string()));
    }

    #[test]
    fn test_too_many_combinations() {
        let expander = RegexExpander::new(10);
        let result = expander.expand_pattern("[0-9]{2}").unwrap_err();
        assert!(matches!(result, rust_btc_passphrase_finder::regex_parser::RegexError::TooManyPossibilities));
    }
}