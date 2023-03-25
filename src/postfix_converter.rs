use crate::postfix_converter::PriorityOperation::{InStack, OutStack, EqualPriority};
use crate::{Type, parse_expression};
use crate::Type::{Operation, Parentheses};

enum PriorityOperation {
    InStack,
    OutStack,
    EqualPriority
}

fn priority_operation(new: char, in_stack: char) -> PriorityOperation {
    match new {
        '+' | '-' => {
            match in_stack {
                '*' | '/' => return InStack,
                '-' | '+' => return EqualPriority,
                _ => panic!("internal error, wrong operands in the stack")
            }
        },
        '*' | '/' => {
            match in_stack {
                '*' | '/' => return EqualPriority,
                '-' | '+' => return OutStack,
                _ => panic!("internal error, wrong operands in the stack")
            }
        }
        _ => {
            panic!("internal error, wrong operands in the stack")
        }
    }
}

pub fn convert_to_postfix(parsed_prefix: Vec<Type>) -> Option<Vec<Type>> {
    let mut postfix_sequence = Vec::new();

    let mut operation_stack= Vec::new();

    let mut prev_op = false;
    let mut prev_num = false;
    let mut index_closed = 0;

    for (index, &token) in parsed_prefix.iter().enumerate() {
        if index <= index_closed && index_closed != 0 {
            continue
        }
        match token {
            Type::Number(_) => {
                if prev_num {
                    return None;
                }
                postfix_sequence.push(token);
                prev_op = false;
                prev_num = true;
            }
            Type::Operation(oper) => {
                if prev_op {
                    return None;
                }
                prev_op = true;
                prev_num = false;
                if operation_stack.is_empty() {
                    operation_stack.push(oper);
                    continue
                }
                if let InStack | EqualPriority = priority_operation(oper, *operation_stack.last()?) {
                    while !operation_stack.is_empty() {
                        if let OutStack = priority_operation(oper, *operation_stack.last()?) {
                            break
                        }
                        postfix_sequence.push(Type::Operation(operation_stack.pop()?));
                    }
                }
                operation_stack.push(oper);
            },
            Type::Parentheses('(') => {
                let mut counter = 1;
                index_closed = index;

                for &token in parsed_prefix[index+1..].iter() {
                    if counter == 0 { break }
                    index_closed += 1;
                    match token {
                        Type::Parentheses('(') => { counter += 1; }
                        Type::Parentheses(')') => { counter -= 1; }
                        _ => continue
                    }
                }

                let mut sub_sequence= Vec::from_iter(parsed_prefix[index+1..index_closed].iter().cloned());

                postfix_sequence.append(
                    &mut convert_to_postfix(sub_sequence).unwrap()
                );

                prev_op = false;
            },
            _ => return None
        }
    }

    while !operation_stack.is_empty() {
        postfix_sequence.push(Type::Operation(operation_stack.pop()?));
    }

    Some(postfix_sequence)
}

#[cfg(test)]
mod conversion_tests {
    use crate::{Type, parse_expression};
    use super::convert_to_postfix;

    #[test]
    fn convert_correct() {
        let prefix_sequence = "3 + 4 - 7";
        let postfix_sequence = "3 4 + 7 -";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn convert_correct_multiple_op() {
        let prefix_sequence = "1 * 2 * 3 * 4";
        let postfix_sequence = "1 2 * 3 * 4 *";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn convert_different_op() {
        let prefix_sequence = "1 - 2 * 3 - 4 + 5 / 7";
        let postfix_sequence = "1 2 3 * - 4 - 5 7 / +";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn convert_different_op_2() {
        let prefix_sequence = "1 + 7 - 3 * 3 / 5 / 5 + 2 - 1 / 5";
        let postfix_sequence = "1 7 + 3 3 * 5 / 5 / - 2 + 1 5 / -";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn wrong_sequence_none() {
        let prefix_sequence = "1 - 2 - - 3";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()),
            None
        );
    }

    #[test]
    fn parentheses_one() {
        let prefix_sequence = "( 11 + 5 )";
        let postfix_sequence = "11 5 +";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_simple() {
        let prefix_sequence = "( 4 + 3 ) / 8";
        let postfix_sequence = "4 3 + 8 /";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_simple_2() {
        let prefix_sequence = "( 4 + 3 ) / ( 8 + 1 )";
        let postfix_sequence = "4 3 + 8 1 + /";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_simple_3() {
        let prefix_sequence = "1 + ( 4 / 4 ) + 1";
        let postfix_sequence = "1 4 4 / + 1 +";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_simple_4() {
        let prefix_sequence = "( 4 / 5 ) - ( 17 + 1 ) / ( 15 + 4 )";
        let postfix_sequence = "4 5 / 17 1 + 15 4 + / -";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_double() {
        let prefix_sequence = "( ( ( 4 / 5 ) ) )";
        let postfix_sequence = "4 5 / ";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_complex() {
        let prefix_sequence = "1 + ( 25 * 4 - 14 / 7 ) + 72 / ( ( 44 + 11 ) / 5 ) + 5";
        let postfix_sequence = "1 25 4 * 14 7 / - + 72 44 11 + 5 / / + 5 +";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_complex_2() {
        let prefix_sequence = "( ( 7 * ( 5 - 2 ) + ( 9 / ( 1 + 2 ) ) ) * ( 6 - 2 ) / ( ( 5 / 1 ) * ( 7 + 2 ) ) ) + ( ( ( 8 - 2 ) * ( 9 / 3 ) - ( ( 2 * 6 ) / ( 4 + 1 ) ) ) * ( 5 + ( 7 / 3 ) ) ) - ( ( 9 - 1 ) * ( ( 2 + 4 ) * ( 3 / 1 ) - ( 6 * 2 ) / ( 5 + 3 ) ) ) / ( ( 35 - 3 ) + ( 9 / 2 ) )";
        let postfix_sequence = "7 5 2 - * 9 1 2 + / + 6 2 - * 5 1 / 7 2 + * / 8 2 - 9 3 / * 2 6 * 4 1 + / - 5 7 3 / + * + 9 1 - 2 4 + 3 1 / * 6 2 * 5 3 + / - * 35 3 - 9 2 / + / -";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn parentheses_complex_mix_space() {
        let prefix_sequence = "(( 7 * ( 5- 2)    + ( 9 / ( 1 + 2 ) ) ) * (6 - 2 ) / ( ( 5/ 1 ) * (7 + 2 ) ))   +  (( (8 - 2 ) * (9 / 3 ) - ( ( 2 * 6 ) / ( 4 + 1) )) * ( 5 + (7 / 3 ) ) )-  ( ( 9   -1 ) * ( ( 2 + 4 ) * ( 3 / 1 ) - ( 6 * 2) /( 5 + 3 ))) / ( ( 35 - 3 ) + ( 9 / 2 ))";
        let postfix_sequence = "7 5 2 - * 9 1 2 + / + 6 2 - * 5 1 / 7 2 + * / 8 2 - 9 3 / * 2 6 * 4 1 + / - 5 7 3 / + * + 9 1 - 2 4 + 3 1 / * 6 2 * 5 3 + / - * 35 3 - 9 2 / + / -";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn double_digit() {
        let prefix_sequence = "12 + 11";
        let postfix_sequence = "12 11 +";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn double_digit_parentheses() {
        let prefix_sequence = "853 + ( 12 / ( 1 + 5 ) ) - 855 + ( 11 - 7 )";
        let postfix_sequence = "853 12 1 5 + / + 855 - 11 7 - +";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }

    #[test]
    fn decimal_digit() {
        let prefix_sequence = "11.5 - 72.889 / 553";
        let postfix_sequence = "11.5 72.889 553 / -";

        assert_eq!(
            convert_to_postfix(parse_expression(prefix_sequence).unwrap()).unwrap(),
            parse_expression(postfix_sequence).unwrap()
        );
    }
}