
use crate::string_parser;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(string_parser::parse_equation("1+1"), 2);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(string_parser::parse_equation("1-1"), 0);
    }
}