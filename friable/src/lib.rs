pub mod filter {
    ///A filter contains a list of tuples with the characters that will be used as separators, and a
    /// boolean indicatinf whether or not they'll be a token
    pub struct Filter {
        separators: Vec<(String, bool)>,
    }

    impl Filter {
        pub fn new(separators: Vec<(String, bool)>) -> Filter {
            Filter { separators }
        }

        pub fn get_separator(&mut self) -> (String, bool) {
            self.separators.pop().unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_separator_test() {
            let mut filter = Filter::new(vec![
                ("(".to_string(), false),
                (")".to_string(), true),
                (" ".to_string(), false),
            ]);
            let (sep, status) = filter.get_separator();
            assert_eq!(sep, " ".to_string());
            assert!(!status);
        }
    }
}

pub mod token {
    use super::filter;

    pub struct Tokenizer {
        filter: filter::Filter,
    }

    impl Tokenizer {
        ///Initializes a Tokenizer with the default filter which only separates the string
        /// by white spaces and doesn't cosider them as tokens. It has the same functionality
        /// as the std function split_whitespace()
        pub fn new_default() -> Tokenizer {
            let default_filter = filter::Filter::new(vec![(" ".to_string(), false)]);
            Tokenizer {
                filter: default_filter,
            }
        }

        ///Initializes a tokenizer with a filter specified by the user.
        pub fn new(filter: filter::Filter) -> Tokenizer {
            Tokenizer { filter }
        }
        ///Use the filter to generate a Vec of &str in which every element is a "token"
        pub fn tokenize(string: &mut String) -> Vec<&str> {
            unimplemented!();
        }
    }
}
