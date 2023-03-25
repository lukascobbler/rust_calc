#![allow(unused)]

use std::io;
use rust_calc::calculate_expression;

fn main() {
    println!("Enter an expression in the infix format with spaces: ");

    let mut expression_buffer = String::new();

    io::stdin().read_line(&mut expression_buffer);

    match calculate_expression(&expression_buffer.trim()) {
        Ok(res) => println!("The final resut is: {}", res),
        Err(err) => eprintln!("An error occured: {}", err)
    }
}
