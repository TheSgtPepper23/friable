pub mod filter {
    ///A filter contains a list of tuples with the characters that will be used as separators, and a
    /// boolean indicatinf whether or not they'll be a token
    pub struct Filter {
        separators: Vec<(char, bool)>,
    }

    impl Filter {
        pub fn new(separators: Vec<(&str, bool)>) -> Filter {
            let mut fixed_separators: Vec<(char, bool)> = Vec::new();
            for x in separators {
                fixed_separators.push((x.0.chars().next().unwrap(), x.1));
            }
            Filter {
                separators: fixed_separators,
            }
        }

        pub fn next_separator(&mut self) -> (char, bool) {
            self.separators.pop().unwrap()
        }

        pub fn contains(&self, character: &char) -> Option<bool> {
            for separator in &self.separators {
                if separator.0 == *character {
                    return Some(separator.1);
                }
            }
            return None;
        }

        pub fn add_separator(&mut self, character: (char, bool)) {
            self.separators.push(character);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_separator_test() {
            let mut filter = Filter::new(vec![("(", false), (")", true), (" ", false)]);
            let (sep, status) = filter.next_separator();
            assert_eq!(sep, " ".chars().next().unwrap());
            assert!(!status);
        }

        #[test]
        fn add_separator_test() {
            let mut filter = Filter::new(vec![(" ", false)]);
            filter.add_separator(('+', true));
            assert_eq!(filter.next_separator().0, '+')
        }
    }
}

pub mod token {
    use super::filter::Filter;

    pub struct Tokenizer {
        filter: Filter,
    }

    impl Tokenizer {
        ///Initializes a Tokenizer with the default filter which only separates the string
        /// by white spaces and doesn't cosider them as tokens. It has the same functionality
        /// as the std function split_whitespace()
        pub fn new_default() -> Tokenizer {
            let default_filter = Filter::new(vec![(" ", false)]);
            Tokenizer {
                filter: default_filter,
            }
        }

        ///Initializes a tokenizer with a filter specified by the user.
        pub fn new(filter: Filter) -> Tokenizer {
            Tokenizer { filter }
        }
        ///Use the filter to generate a Vec of &str in which every element is a "token"
        pub fn tokenize(&self, string: String) -> Vec<String> {
            let mut start;
            let mut end = 0;
            let mut tokens: Vec<String> = vec![];
            let chars: Vec<char> = string.chars().collect();
            for (index, character) in chars.iter().enumerate() {
                match self.filter.contains(&character) {
                    Some(keep) => {
                        start = end;
                        end = index;
                        if let Some(x) = self.create_token(&chars, start, end) {
                            tokens.push(x)
                        };
                        if keep {
                            if let Some(x) = self.create_token(&chars, end, end + 1) {
                                tokens.push(x)
                            };
                        }
                        end += 1;
                    }
                    None => (),
                }
            }
            if let Some(x) = self.create_token(&chars, end, chars.len()) {
                tokens.push(x)
            };
            tokens
        }

        fn create_token(&self, chars: &Vec<char>, start: usize, end: usize) -> Option<String> {
            let token = chars[start..end]
                .iter()
                .collect::<String>()
                .trim()
                .to_string();
            if token.is_empty() {
                None
            } else {
                Some(token)
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::Tokenizer;
        use crate::filter::Filter;

        #[test]
        fn tokenize_test() {
            let tokenizer = Tokenizer::new_default();
            let tokens = tokenizer.tokenize("perro estúpido".to_string());
            for token in tokens {
                println!(".:{}:.", token);
            }
        }
        #[test]
        fn tokenize_test_two() {
            let filter = Filter::new(vec![("+", true), ("(", true), (")", true), (",", true)]);
            let tokenizer = Tokenizer::new(filter);
            let tokens = tokenizer.tokenize("(+ 10, 10)".to_string());
            for token in tokens {
                println!(".:{}:.", token);
            }
        }
    }
}
