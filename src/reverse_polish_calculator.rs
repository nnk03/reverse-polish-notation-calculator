#![allow(dead_code)]

use std::io::BufRead;

use crate::expression::Expression;
use crate::globals::*;

type Stack<T> = Vec<T>;

pub fn calculate(line: String) -> Result<String, MathError> {
    let mut stack: Stack<Expression> = Stack::new();

    for word in line.split_whitespace() {
        let result;

        match word {
            "+" => {
                if stack.len() < 2 {
                    return Err(MathError::ParseError);
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                result = a + b;
            }
            "-" => {
                if stack.len() < 2 {
                    return Err(MathError::ParseError);
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                result = a - b
            }
            "*" => {
                if stack.len() < 2 {
                    return Err(MathError::ParseError);
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                result = a * b;
            }
            "/" => {
                if stack.len() < 2 {
                    return Err(MathError::ParseError);
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                result = a / b;
            }
            "^" => {
                if stack.len() < 2 {
                    return Err(MathError::ParseError);
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let n = b.exponentiation_number();
                if n.is_none() {
                    return Err(MathError::ParseError);
                }

                let n = n.unwrap();

                if n.floor() != n.ceil() {
                    return Err(MathError::ParseError);
                }

                let n = n as i64;

                // need to change here
                result = a.pow(n);
            }
            "d" => {
                //
                if stack.len() < 1 {
                    return Err(MathError::ParseError);
                }

                let a = stack.pop().unwrap();
                result = a.d();
            }
            _ => {
                // need to parse the number or variable x
                let expression = Expression::from(word);

                if expression.is_err() {
                    return Err(MathError::ParseError);
                }

                let expression = expression.unwrap();
                result = expression;
            }
        }

        if result.is_denominator_zero() {
            return Err(MathError::NAN);
        }
        stack.push(result)
    }

    if stack.len() == 1 {
        Ok(stack[0].rpn_string())
    } else {
        return Err(MathError::ParseError);
    }
}

pub fn run() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();

    // read the standard input and then output
    for line in reader.lines() {
        let line = line.unwrap();

        let result = calculate(line);
        match result {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err.to_string()),
        }
    }
}
