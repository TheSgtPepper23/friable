mod filter;

pub use crate::filter::filter_module;

pub mod token {
    use super::filter_module::Filter;

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
        ///Use the filter to generate a Vec of Strings in which every element is a "token".
        /// If the separator is set to true, the separator will also be added as a token.
        pub fn tokenize(&self, string: String) -> Vec<String> {
            let mut start;
            let mut end = 0;
            let mut tokens: Vec<String> = vec![];
            let chars: Vec<char> = string.chars().collect();
            for (index, character) in chars.iter().enumerate() {
                match self.filter.contains(*character) {
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

        ///Splits a char Vec at the specified points, start and end, then collects that array into
        /// a String and finally if that string is empty returns None, otherwise returns
        /// the string.
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
        use crate::filter_module::Filter;

        #[test]
        fn tokenize_test() {
            let tokenizer = Tokenizer::new_default();
            let tokens = tokenizer.tokenize("perro estúpido".to_string());
            assert_eq!(tokens[0], "perro");
            assert_eq!(tokens[1], "estúpido");
        }
        #[test]
        fn tokenize_test_two() {
            let filter = Filter::new(vec![("+", true), ("(", true), (")", true), (",", true)]);
            let tokenizer = Tokenizer::new(filter);
            let tokens = tokenizer.tokenize("(+ 10, 10)".to_string());
            assert_eq!(tokens[0], "(");
            assert_eq!(tokens[tokens.len() - 1], ")");
        }
    }
}
