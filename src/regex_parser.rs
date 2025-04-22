use regex_syntax::hir::{Hir, HirKind, Class, Repetition};
use regex_syntax::Parser;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RegexError {
    Parse(regex_syntax::Error),
    TooManyPossibilities,
    UnsupportedPattern(String),
}

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegexError::Parse(e) => write!(f, "Failed to parse regex: {}", e),
            RegexError::TooManyPossibilities => write!(f, "Pattern would generate too many possibilities"),
            RegexError::UnsupportedPattern(p) => write!(f, "Unsupported regex pattern: {}", p),
        }
    }
}

impl Error for RegexError {}

impl From<regex_syntax::Error> for RegexError {
    fn from(err: regex_syntax::Error) -> RegexError {
        RegexError::Parse(err)
    }
}

pub struct RegexExpander {
    max_combinations: usize,
}

impl RegexExpander {
    pub fn new(max_combinations: usize) -> Self {
        Self { max_combinations }
    }

    pub fn expand_pattern(&self, pattern: &str) -> Result<Vec<String>, RegexError> {
        let hir = Parser::new().parse(pattern)?;
        let mut combinations = Vec::new();
        self.expand_hir(&hir, String::new(), &mut combinations)?;
        
        if combinations.len() > self.max_combinations {
            return Err(RegexError::TooManyPossibilities);
        }
        
        Ok(combinations)
    }

    fn expand_hir(&self, hir: &Hir, current: String, results: &mut Vec<String>) -> Result<(), RegexError> {
        match hir.kind() {
            HirKind::Concat(nodes) => {
                let mut temp_results = vec![current];
                for node in nodes {
                    let mut new_results = Vec::new();
                    for prefix in temp_results {
                        self.expand_hir(node, prefix, &mut new_results)?;
                    }
                    temp_results = new_results;
                }
                results.extend(temp_results);
                Ok(())
            }
            HirKind::Alternation(alts) => {
                for alt in alts {
                    self.expand_hir(alt, current.clone(), results)?;
                }
                Ok(())
            }
            HirKind::Literal(lit) => {
                results.push(current + &String::from_utf8_lossy(&lit.0));
                Ok(())
            }
            HirKind::Class(class) => {
                match class {
                    Class::Unicode(class_unicode) => {
                        for range in class_unicode.iter() {
                            for c in (range.start()..=range.end()).map(char::from) {
                                let mut new_str = current.clone();
                                new_str.push(c);
                                results.push(new_str);
                            }
                        }
                    }
                    _ => return Err(RegexError::UnsupportedPattern(
                        "Only Unicode character classes are supported".to_string(),
                    )),
                }
                Ok(())
            }
            HirKind::Repetition(Repetition { min, max, sub, .. }) => {
                let min = *min;
                let max = max.map(|m| m).unwrap_or(min + 1);
                if max > 10 {
                    return Err(RegexError::UnsupportedPattern(
                        "Repetition count too large".to_string(),
                    ));
                }

                let mut final_results = Vec::new();
                let mut base_expansions = Vec::new();
                self.expand_hir(sub, String::new(), &mut base_expansions)?;
                
                // Generate sequences of length min..=max
                for len in min..=max {
                    let current = current.clone();
                    let mut combinations = vec![String::new()];
                    
                    // Generate all possible combinations of the required length
                    for _ in 0..len {
                        let mut new_combinations = Vec::new();
                        for c in &combinations {
                            for e in &base_expansions {
                                new_combinations.push(c.clone() + e);
                            }
                        }
                        combinations = new_combinations;
                    }
                    
                    // Add the prefix to all generated combinations
                    for c in combinations {
                        final_results.push(current.clone() + &c);
                    }
                }
                results.extend(final_results);
                Ok(())
            }
            HirKind::Capture(capture) => {
                // Just process the inner pattern for captures
                self.expand_hir(&capture.sub, current, results)
            }
            _ => Err(RegexError::UnsupportedPattern(format!(
                "Unsupported regex feature: {:?}",
                hir.kind()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}