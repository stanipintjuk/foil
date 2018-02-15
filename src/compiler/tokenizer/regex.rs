use regex::{Regex, Match};

pub fn end_of_whitespace(text: &str) -> Option<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*").unwrap();
    }
    RE.find(text).map(|m|{m.end()})
}

pub fn match_bare_word<'a>(text: &'a str) -> Option<Match<'a>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9_]*").unwrap();
    }
    RE.find(text)
}

pub fn match_double<'a>(text: &'a str) -> Option<Match<'a>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^-?[0-9_]*\.[0-9_]+").unwrap();
    }
    RE.find(text)
}

pub fn match_int<'a>(text: &'a str) -> Option<Match<'a>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^-?[0-9_]+").unwrap();
    }
    RE.find(text)
}

#[cfg(test)]
mod tests {

    mod end_of_whitespace_tests {
        use super::super::end_of_whitespace;

        #[test]
        fn works_with_spaces() {
            let input = "   sdf  ";
            let expected = 3;
            let actual = end_of_whitespace(input);
            
            assert_eq!(Some(expected), actual)
        }

        #[test]
        fn works_with_tabs() {
            let input = " \t\tsdf\t ";
            let expected = 3;
            let actual = end_of_whitespace(input);
            
            assert_eq!(Some(expected), actual)
        }

        #[test]
        fn works_with_new_line() {
            let input = " \n\nsdf\n ";
            let expected = 3;
            let actual = end_of_whitespace(input);
            
            assert_eq!(Some(expected), actual)
        }

    }
    mod match_bare_word_tests {
        use super::super::match_bare_word;
        #[test]
        fn matches_bare_word() {
            let input = "Test123  ";
            let expected = "Test123";
            let actual = match_bare_word(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }

        #[test]
        fn doesnt_match_special_chars() {
            let input = "Test123-someOther";
            let expected = "Test123";
            let actual = match_bare_word(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn works_with_allowed_variable_chars() {
            let input = "Test_123";
            let expected = "Test_123";
            let actual  = match_bare_word(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }
    }

    mod match_double_tests {
        use super::super::match_double;

        #[test]
        fn match_double_matches_double() {
            let input = "234.12 ";
            let expected = "234.12";
            let actual = match_double(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }

        #[test]
        fn does_not_match_int() {
            let input = "234";
            let actual = match_double(input);
            assert_eq!(None, actual);
        }

        #[test]
        fn works_with_underline() {
            let input = "1_000_000.000_00_1";
            let expected = input;
            let actual = match_double(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }

        #[test]
        fn works_without_whole_digit() {
            let input = ".001";
            let expected = input;
            let actual = match_double(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }

        #[test]
        fn works_with_negatives() {
            let input = "-.02";
            let expected = input;
            let actual = match_double(input).unwrap().as_str();
            assert_eq!(expected, actual);
        }
    }

    mod match_int_tests {
        use super::super::match_int;
        
        #[test]
        fn match_int_works() {
            let input = "123 ";
            let expected = "123";
            let actual = match_int(input).unwrap().as_str();
            assert_eq!(expected,  actual);
        }

        #[test]
        fn match_int_works_with_underline() {
            let input = "1_000_000 ";
            let expected =  "1_000_000";
            let actual = match_int(input).unwrap().as_str();
            assert_eq!(actual,  expected);
        }

        #[test]
        fn works_with_negatives() {
            let input = "-10";
            let expected = "-10";
            let actual = match_int(input).unwrap().as_str();
            assert_eq!(actual, expected);
        }
    }
}
