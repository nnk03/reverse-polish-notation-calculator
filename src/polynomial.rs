use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::globals::{Derivative, DisplayRPN, Exponent, Number, EPSILON};

#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    pub coeff: Vec<Number>,
}

// is it necessary to check for empty polynomial
impl Polynomial {
    #[allow(dead_code)]
    pub fn new(coeff: Vec<Number>) -> Polynomial {
        Polynomial { coeff }
    }

    pub fn zero(n: usize) -> Polynomial {
        Polynomial {
            coeff: vec![0.0; n],
        }
    }

    pub fn one(n: usize) -> Polynomial {
        Polynomial {
            coeff: vec![1.0; n],
        }
    }

    pub fn len(&self) -> usize {
        self.coeff.len()
    }

    pub fn is_zero_polynomial(&self) -> bool {
        // if the absolute value of a number is less than EPSILON, consider it to be zero
        self.coeff.iter().all(|val| val.abs() < EPSILON)
    }

    pub fn clean(&mut self) {
        for val in self.coeff.iter_mut() {
            if val.abs() <= EPSILON {
                *val = 0.0
            }
        }

        while let Some(u) = self.coeff.last() {
            if u.abs() < EPSILON {
                self.coeff.pop();
            } else {
                break;
            }
        }

        if self.coeff.len() == 0 {
            self.coeff = vec![0.0]
        }
    }

    pub fn is_constant_polynomial(&self) -> bool {
        if self.len() == 0 {
            return false;
        }

        if self.len() == 1 {
            return true;
        }

        for i in 1..self.len() {
            if self[i].abs() > EPSILON {
                return false;
            }
        }

        true
    }
}

impl DisplayRPN for Polynomial {
    fn rpn_string(&self) -> String {
        let mut q = Vec::new();

        let n = self.len();

        for i in 0..n {
            // if i == 0 {
            //     result = format!("{}", self[0]);
            // } else if i == 1 {
            //     result = format!("{} {} x * +", result, self[1]);
            // } else {
            //     result = format!("{} {} x {} ^ * +", result, self[i], i)
            // }
            if self[i].abs() > EPSILON {
                // q.push(format!("{}"));
                if i == 0 {
                    q.push(format!("{}", self[0]));
                } else if i == 1 {
                    q.push(format!("{} x *", self[1]));
                } else {
                    q.push(format!("{} x {} ^ *", self[i], i));
                }
            }
        }

        if q.len() == 0 {
            return "0".to_string();
        }

        let mut result = String::new();
        for i in 0..q.len() {
            if i == 0 {
                result = format!("{}", q[0]);
            } else {
                result = format!("{} {} +", result, q[i]);
            }
        }

        result
    }
}

impl Derivative for Polynomial {
    type Output = Polynomial;

    fn d(&self) -> Polynomial {
        if self.len() <= 1 {
            return Polynomial::zero(1);
        }

        // length is atleast 2
        // i.e degree is atleast 1
        //

        let n = self.len();

        let mut output = Polynomial::zero(n - 1);

        for i in 0..self.len() - 1 {
            output[i] = (i + 1) as Number * self[i + 1]
        }

        output
    }
}

impl Exponent for Polynomial {
    type Output = Polynomial;

    fn square(&self) -> Self::Output {
        self.clone() * self.clone()
    }

    fn pow(&self, n: i64) -> Self::Output {
        if n == 0 {
            return Polynomial::one(1);
        }
        if n == 1 {
            return Polynomial {
                coeff: self.coeff.clone(),
            };
        }
        if n < 0 {
            panic!("Negative Exponent for Polynomial Structure\n Use it with Expression Structure")
        }

        // using binary exponentiation

        let mut output = self
            .pow((n as Number / 2 as Number).floor() as i64)
            .square();

        if n % 2 == 1 {
            output = output * self.clone();
        }

        output
    }
}

impl Index<usize> for Polynomial {
    type Output = Number;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("Out of bounds");
        }

        &self.coeff[index]
    }
}

impl IndexMut<usize> for Polynomial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.len() {
            panic!("Out of bounds")
        }

        &mut self.coeff[index]
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let a = self.len();
        let b = rhs.len();

        let mut output = match a > b {
            true => self.clone(),
            false => rhs.clone(),
        };

        if a > b {
            for i in 0..b {
                output[i] += rhs[i];
            }
        } else {
            for i in 0..a {
                output[i] += self[i];
            }
        }

        output
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        return self
            + Polynomial {
                coeff: rhs.coeff.iter().map(|&val| -val).collect(),
            };
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.len();
        let b = rhs.len();

        let m = a + b;

        let mut output = Polynomial::zero(m);

        for i in 0..a {
            for j in 0..b {
                output[i + j] += self[i] * rhs[j];
            }
        }

        output
    }
}
