
use crate::string_parser;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_addition() {
        let input: &str = "1+1+1";
        string_parser::parse_equation(input);
        // let thing: Equation<i32, i32> = string_parser::parse_equation(input);
        // assert_eq!(thing.operand_a + thing.operand_b, 2);
    }
}