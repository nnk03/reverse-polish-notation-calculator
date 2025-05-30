use thiserror::Error;

pub type Number = f64;
pub const EPSILON: Number = 1e-6;

pub trait Derivative {
    type Output;
    fn d(&self) -> Self::Output;
}

pub trait Exponent {
    type Output;

    fn pow(&self, n: i64) -> Self::Output;

    fn square(&self) -> Self::Output;
}

pub trait DisplayRPN {
    fn rpn_string(&self) -> String;
}

#[derive(Debug, Error)]
pub enum MathError {
    #[error("Parse Error")]
    ParseError,

    #[error("NAN")]
    NAN,
}
