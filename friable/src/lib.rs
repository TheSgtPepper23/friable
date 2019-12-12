pub mod filter {
    ///A filter contains a list of tuples with the characters that will be used as separators, and a
    /// boolean indicatinf whether or not they'll be a token
    pub struct Filter {
        separators: Vec<(char, bool)>,
    }

    impl Filter {
        pub fn new(separators: Vec<(char, bool)>) -> Filter {
            Filter { separators }
        }

        pub fn next_separator(&mut self) -> (char, bool) {
            self.separators.pop().unwrap()
        }

        pub fn contains(&self, character: &char) -> bool {
            for separator in &self.separators {
                if separator.0 == *character {
                    return true;
                }
            }
            return false;
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
            let mut filter = Filter::new(vec![
                ("(".chars().next().unwrap(), false),
                (")".chars().next().unwrap(), true),
                (" ".chars().next().unwrap(), false),
            ]);
            let (sep, status) = filter.next_separator();
            assert_eq!(sep, " ".chars().next().unwrap());
            assert!(!status);
        }

        #[test]
        fn add_separator_test() {
            let mut filter = Filter::new(vec![(" ".chars().next().unwrap(), false)]);
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
            let default_filter = Filter::new(vec![(" ".chars().next().unwrap(), false)]);
            Tokenizer {
                filter: default_filter,
            }
        }

        ///Initializes a tokenizer with a filter specified by the user.
        pub fn new(filter: Filter) -> Tokenizer {
            Tokenizer { filter }
        }
        ///Use the filter to generate a Vec of &str in which every element is a "token"
        pub fn tokenize(&self, string: &mut String) -> Vec<&str> {
            let chars: Vec<char> = string.chars().collect();
            let tokens: Vec<&str> = vec![];
            for character in chars {
                if self.filter.contains(&character) {
                    println!(".:{}:.", character);
                }
            }
            tokens
        }
    }

    #[cfg(test)]
    mod tests {

        use super::Tokenizer;
        use crate::filter::Filter;

        #[test]
        fn tokenize_test() {
            let tokenizer = Tokenizer::new_default();
            tokenizer.tokenize(&mut "perro estúpido".to_string());
        }

        #[test]
        fn tokenize_test_two() {
            let filter = Filter::new(vec![('+', true)]);
            let tokenizer = Tokenizer::new(filter);
            tokenizer.tokenize(&mut "perro+estúpido".to_string());
        }
    }
}
