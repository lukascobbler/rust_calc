use crate::postfix_converter::PriorityOperation::{InStack, OutStack, EqualPriority};
use crate::{Type, parse_expression};

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

pub fn convert_to_postfix(prefix: &str) -> Option<Vec<Type>> {
    let mut postfix_sequence = Vec::new();

    let mut operation_stack= Vec::new();

    let mut prev_op = false;
    let mut prev_num = false;

    for token in parse_expression(prefix) {
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
                unimplemented!()
            },
            Type::Parentheses(')') => {
                unimplemented!()
            }
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

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn convert_correct_multiple_op() {
        let prefix_sequence = "1 * 2 * 3 * 4";
        let postfix_sequence = "1 2 * 3 * 4 *";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn convert_different_op() {
        let prefix_sequence = "1 - 2 * 3 - 4 + 5 / 7";
        let postfix_sequence = "1 2 3 * - 4 - 5 7 / +";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn convert_different_op_2() {
        let prefix_sequence = "1 + 7 - 3 * 3 / 5 / 5 + 2 - 1 / 5";
        let postfix_sequence = "1 7 + 3 3 * 5 / 5 / - 2 + 1 5 / -";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn wrong_sequence_none() {
        let prefix_sequence = "1 - 2 - - 3";

        assert_eq!(convert_to_postfix(prefix_sequence), None);
    }

    #[test]
    fn parentheses_sequence() {
        let prefix_sequence = "     ";
        let postfix_sequence = "1 2 3 5 / - 4 15 / + 1 + 1 2 / + + 17 12 5 12 / + 3 + * - 32 -";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn double_digit() {
        let prefix_sequence = "12 + 11";
        let postfix_sequence = "12 11 +";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn double_digit_parentheses() {
        let prefix_sequence = "853 + (12 / (1 + 5)) - 855 + (11 - 7)";
        let postfix_sequence = "853 12 1 5 + / + 855 - 11 7 - +";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }

    #[test]
    fn decimal_digit() {
        let prefix_sequence = "11.5 - 72.889 / 553";
        let postfix_sequence = "11.5 72.889 553 / -";

        assert_eq!(convert_to_postfix(prefix_sequence).unwrap(), parse_expression(postfix_sequence));
    }
}