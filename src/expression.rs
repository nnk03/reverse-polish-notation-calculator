#![allow(dead_code)]

use std::{
    error::Error,
    ops::{Add, Div, Mul},
};

use crate::polynomial::{Derivative, DisplayRPN, Exponent, Number, Polynomial};

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

    pub fn from(input: &str) -> Result<Expression, Box<dyn Error>> {
        let mut output = Expression::zero(2);

        if input == "x" {
            output.numerator[1] = 1.0;

            return Ok(output);
        }

        // currently ignoring that number of digits allowed after decimal point is 5
        let number: Number = input.parse()?;
        output.numerator[1] = number;

        Ok(output)
    }

    pub fn is_denominator_zero(&self) -> bool {
        self.denominator.is_zero_polynomial()
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

    fn pow(&self, n: i32) -> Self::Output {
        if n == 0 {
            return Expression::one(1);
        }
        if n < 0 {
            return Expression::one(1) / self.pow(-n);
        }

        let mut output = self
            .pow((n as Number / 2 as Number).floor() as i32)
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
