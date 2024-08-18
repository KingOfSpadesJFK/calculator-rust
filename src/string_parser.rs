use std::ops;

use regex::Regex;

pub fn parse_equation(input: &str) -> i64 {
    // Basic
    let mut result: i64 = 0;
    let mut re = Regex::new(r"[+-/\*]").unwrap();
    let operands: Vec<&str> = re.split(input).collect();
    re = Regex::new(r"\d+").unwrap();
    let mut operators: Vec<&str> = re.split(input).collect();
    operators.remove(0);                    // There's always two extra blank entries, each at the beginning and end
    operators.remove(operators.len()-1);    //  Just remove those for the time being.
    result = operands[0].trim().parse::<i32>().unwrap() as i64;
    
    // Parse these two parts
    //  Initial parsing should be i32. We can't just let numbers be that big lol
    for i in 0..operators.len() {
        let a: i64 = result;
        let b: i64 = operands[i+1].trim().parse::<i32>().unwrap() as i64;

        // TODO: Order of operations
        result = equate(a, b, operators[i]);
    }

    // I should find a better way to do this in rust
    // if parts[1].trim() == "+" {
    //     equation.1 = ParsedOperator::Add;
    // } else if parts[1].trim() == "-" {
    //     equation.1 = ParsedOperator::Subtract;
    // } else if parts[1].trim() == "*" {
    //     equation.1 = ParsedOperator::Multiply;
    // } else if parts[1].trim() == "/" {
    //     equation.1 = ParsedOperator::Divide;
    // }

    return result;
} 

fn equate(a: i64, b: i64, op: &str) -> i64 {
    let mut result: i64 = 0;
    if op == "+" {
        result = a + b;
    } else if op == "-" {
        result = a - b;
    }
    return result;
}