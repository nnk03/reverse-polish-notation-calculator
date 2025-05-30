#![allow(dead_code)]

use std::{
    error::Error,
    ops::{Add, Div, Mul, Sub},
};

use crate::polynomial::Polynomial;

use crate::globals::{Derivative, DisplayRPN, Exponent, Number, EPSILON};

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    numerator: Polynomial,
    denominator: Polynomial,
}

impl Expression {
    pub fn zero(n: usize) -> Expression {
        Expression {
            numerator: Polynomial::zero(n),
            denominator: Polynomial::one(1),
        }
    }

    pub fn one(n: usize) -> Expression {
        Expression {
            numerator: Polynomial::one(n),
            denominator: Polynomial::one(1),
        }
    }

    pub fn degree(&self) -> usize {
        // this function most probably will never be used
        self.numerator.degree() - self.denominator.degree()
    }

    pub fn clean(&mut self) {
        self.numerator.clean();
        self.denominator.clean();
    }

    pub fn from(input: &str) -> Result<Expression, Box<dyn Error>> {
        if input == "x" {
            let mut output = Expression::zero(2);
            output.numerator[1] = 1.0;

            return Ok(output);
        }

        let mut output = Expression::zero(1);

        // currently ignoring that number of digits allowed after decimal point is 5
        let number: Number = input.parse()?;
        output.numerator[0] = number;

        Ok(output)
    }

    pub fn is_denominator_zero(&self) -> bool {
        self.denominator.is_zero_polynomial()
    }

    pub fn exponentiation_number(&self) -> Option<Number> {
        if self.denominator.len() < 1 || self.numerator.len() < 1 {
            return None;
        }

        for i in 1..self.denominator.len() {
            if self.denominator[i].abs() > EPSILON {
                return None;
            }
        }

        for i in 1..self.numerator.len() {
            if self.numerator[i].abs() > EPSILON {
                return None;
            }
        }

        if self.denominator[0].abs() < EPSILON {
            return None;
        }

        Some(self.numerator[0] / self.denominator[0])
    }
}

impl Derivative for Expression {
    type Output = Expression;

    fn d(&self) -> Self::Output {
        let Expression {
            numerator: u,
            denominator: v,
        } = self.clone();

        let numerator = v.clone() * u.d() - u * v.clone().d();
        let denominator = v.square();

        Expression {
            numerator,
            denominator,
        }
    }
}

impl Exponent for Expression {
    type Output = Expression;

    fn square(&self) -> Self::Output {
        Expression {
            numerator: self.numerator.square(),
            denominator: self.denominator.square(),
        }
    }

    fn pow(&self, n: i64) -> Self::Output {
        if n == 0 {
            return Expression::one(1);
        }
        if n < 0 {
            return Expression::one(1) / self.pow(-n);
        }

        let mut output = self
            .pow((n as Number / 2 as Number).floor() as i64)
            .square();

        if n % 2 == 1 {
            output = output * self.clone();
        }

        output
    }
}

impl DisplayRPN for Expression {
    fn rpn_string(&self) -> String {
        format!(
            "{} {} /",
            self.numerator.rpn_string(),
            self.denominator.rpn_string()
        )
    }
}

impl Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        Expression {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
    }
}

impl Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        // take the product of the denominators

        let denominator = self.denominator.clone() * rhs.denominator.clone();

        let left_numerator = self.numerator * rhs.denominator;
        let right_numerator = self.denominator * rhs.numerator;

        Expression {
            numerator: left_numerator + right_numerator,
            denominator,
        }
    }
}

impl Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        return self
            + Expression {
                numerator: Polynomial {
                    coeff: rhs.numerator.coeff.iter().map(|&val| -val).collect(),
                },
                denominator: rhs.denominator,
            };
    }
}
