use rust_calc::calculate_expression;

#[test]
fn calculate_addition() {
    let expression = "1 + 1";
    let result = 2.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_mutiple_addition() {
    let expression = "1 + 1 + 1 + 3";
    let result = 6.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_subtraction() {
    let expression = "1 - 3";
    let result = -2.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_subtraction() {
    let expression = "1 - 3 - 18";
    let result = -20.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiplication() {
    let expression = "2 * 16";
    let result = 32.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_multiplication() {
    let expression = "2 * 16 * 16";
    let result = 512.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_division() {
    let expression = "2 / 8";
    let result = 0.25;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_division() {
    let expression = "2 / 8 / 0.25";
    let result = 1.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_operation() {
    let expression = "2 - 4 * 11 / 1 + 42.5";
    let result = 0.5;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_negative_multiplication() {
    let expression = "1 * -1";
    let result = -1.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_negative_multiplication() {
    let expression = "1 * -1 * -15.6";
    let result = 15.6;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_negative_division() {
    let expression = "-0.5 / 1";
    let result = -0.5;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_negative_division() {
    let expression = "-0.5 / 1 / 4 / 2";
    let result = -0.0625;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_negative_addition() {
    let expression = "2 + -2";
    let result = 0.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_negative_addition() {
    let expression = "2 + -2 + -13";
    let result = -13.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_negative_subtraction() {
    let expression = "2 - -4";
    let result = 6.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_multiple_negative_subtraction() {
    let expression = "2 - -6 - -13";
    let result = 21.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_parentheses() {
    let expression = "(1 + 2) / 2";
    let result = 1.0;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}

#[test]
fn calculate_complex_parentheses() {
    let expression = "((18 / 6 + 15) / 9) - 2 + 13.7 / 2";
    let result = 6.85;

    assert_eq!(calculate_expression(expression).unwrap(), result);
}