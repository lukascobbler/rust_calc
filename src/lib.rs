#![allow(unused)]
use regex::Regex;

mod paren_checker;
mod calculator;
mod postfix_converter;

pub fn calculate_expression(expression: &str) -> Result<f64, &str>{
    if !paren_checker::check_pairs(expression) {
        return Err("not every parentheses has a pair");
    }

    let parsed_expression = match parse_expression(expression) {
        Some(pe) => pe,
        None => return Err("the entered expression contains illegal characters")
    };

    let calculation_tokens = match postfix_converter::convert_to_postfix(parsed_expression) {
        None => return Err("the entered expression has too many operands"),
        Some(ct) => ct
    };

    return match calculator::calculate(calculation_tokens) {
        None => Err("math error (ex. 0 division error)"),
        Some(final_result) => Ok(final_result)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Number(f64),
    Operation(char),
    Parentheses(char),
    Illegal,
}

fn parse_expression(expression: &str) -> Option<Vec<Type>> {
    let mut tokens = Vec::new();

    let types = [
        Regex::new(r"[-+]?(\d+(\.\d*)?|\.\d+)([eE][-+]?\d+)?").unwrap(),
        Regex::new(r"[+\-*/]").unwrap(),
        Regex::new(r"[()]").unwrap(),
    ];

    for token in expression.split_whitespace() {
        if types[0].is_match(token) {
            tokens.push(Type::Number(token.parse().ok()?));
        } else if types[1].is_match(token) {
            tokens.push(Type::Operation(token.parse().ok()?));
        } else if types[2].is_match(token) {
            tokens.push(Type::Parentheses(token.parse().ok()?));
        }
    }

    Some(tokens)
}

#[cfg(test)]
mod parse_expression_tests {
    use crate::Type;
    use super::parse_expression;

    #[test]
    fn parse_two() {
        let expression = "1 + 1";
        let result = vec![Type::Number(1.0), Type::Operation('+'), Type::Number(1.0)];

        assert_eq!(parse_expression(expression).unwrap(), result);
    }

    #[test]
    fn parse_two_space() {
        let expression = "1 + 1 ";
        let result = vec![Type::Number(1.0), Type::Operation('+'), Type::Number(1.0)];

        assert_eq!(parse_expression(expression).unwrap(), result);
    }

    #[test]
    fn parse_no_space() {
        let expression = "1+1";
        let result = vec![Type::Number(1.0), Type::Operation('+'), Type::Number(1.0)];

        assert_eq!(parse_expression(expression).unwrap(), result);
    }
}