use crate::Type;

pub fn calculate(type_expression: Vec<Type>) -> Option<f64> {
    let mut numbers_stack= vec![];

    for token in type_expression {
        match token {
            Type::Number(num) => numbers_stack.push(num),
            Type::Operation('*') => {
                let op1 = numbers_stack.pop()?;
                let op2 = numbers_stack.pop()?;

                numbers_stack.push(op1 * op2);
            },
            Type::Operation('+') => {
                let op1 = numbers_stack.pop()?;
                let op2 = numbers_stack.pop()?;

                numbers_stack.push(op1 + op2);
            },
            Type::Operation('/') => {
                let op1 = numbers_stack.pop()?;
                let op2 = numbers_stack.pop()?;

                numbers_stack.push(op2 / op1);
            },
            Type::Operation('-') => {
                let op1 = numbers_stack.pop()?;
                let op2 = numbers_stack.pop()?;

                numbers_stack.push(op2 - op1);
            }
            _ => return None
        }
    }

    Some(numbers_stack.pop()?)
}

#[cfg(test)]
mod calculator_tests {
    use super::calculate;
    use crate::parse_expression;

    #[test]
    fn calculate_addition() {
        let expression = "1 2 +";

        assert_eq!(calculate(parse_expression(expression)).unwrap(), 3.0);
    }

    #[test]
    fn calculate_subtraction() {
        let expression = "1 2 -";

        assert_eq!(calculate(parse_expression(expression)).unwrap(), -1.0);
    }

    #[test]
    fn calculate_division() {
        let expression = "3 4 /";

        assert_eq!(calculate(parse_expression(expression)).unwrap(), 0.75);
    }

    #[test]
    fn calculate_multiplication() {
        let expression = "5 5 *";

        assert_eq!(calculate(parse_expression(expression)).unwrap(), 25.0);
    }

    #[test]
    fn calculate_mix() {
        let expression = "12 12 + 24 - 700 +";

        assert_eq!(calculate(parse_expression(expression)).unwrap(), 700.0);
    }

    #[test]
    fn calculate_mix_complex() {
        let expression = "1 2 3 4 / - 4 8 / + 1 + 1 2 / + + 17 12 5 10 / + 3 + * - 32 -";

        assert_eq!(calculate(parse_expression(expression)).unwrap(), -291.25);
    }
}