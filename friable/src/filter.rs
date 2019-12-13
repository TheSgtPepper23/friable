pub mod filter_module {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    ///Represents a separator. The symbol is the character which will be used for separate the string
    /// and the keep if it'll be considered as a token or not.
    pub struct Separator {
        pub symbol: char,
        pub keep: bool,
    }

    impl Separator {
        pub fn new(symbol: char, keep: bool) -> Separator {
            Separator { symbol, keep }
        }
    }

    impl PartialEq for Separator {
        fn eq(&self, other: &Self) -> bool {
            self.symbol == other.symbol
        }
    }
    impl Eq for Separator {}

    impl Hash for Separator {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.symbol.hash(state);
        }
    }

    ///A filter contains a list of tuples with the characters that will be used as separators, and a
    /// boolean indicatinf whether or not they'll be a token
    pub struct Filter {
        separators: HashSet<Separator>,
    }

    impl Filter {
        pub fn new(separators: Vec<(&str, bool)>) -> Filter {
            let mut fixed_separators: HashSet<Separator> = HashSet::new();
            for separator in separators {
                fixed_separators.insert(Separator::new(
                    separator.0.chars().next().unwrap(),
                    separator.1,
                ));
            }
            Filter {
                separators: fixed_separators,
            }
        }

        ///Returns Some(bool) if the set contains the char. The bool represents
        /// if the separator has to be keeped as a token or not.
        pub fn contains(&self, character: char) -> Option<bool> {
            for separator in &self.separators {
                if separator.symbol == character {
                    return Some(separator.keep);
                }
            }
            None
        }

        ///Add a separator to the separators set. Since it is a set,
        /// separators cannot be repeated
        pub fn add_separator(&mut self, character: (&str, bool)) {
            self.separators.insert(Separator::new(
                character.0.chars().next().unwrap(),
                character.1,
            ));
        }

        pub fn get_separators(&self) -> usize {
            self.separators.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_separator_test() {
            let mut filter = Filter::new(vec![(" ", false)]);
            filter.add_separator(("+", true));
            assert_eq!(filter.contains('+'), Some(true));
        }

        #[test]
        fn override_separator_test() {
            let mut filter = Filter::new(vec![(" ", false), ("+", false)]);
            filter.add_separator(("+", false));
            assert_eq!(filter.get_separators(), 2);
            filter.add_separator(("+", true));
            assert_eq!(filter.get_separators(), 2);
        }
    }
}
