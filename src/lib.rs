pub mod interpreter;
pub mod lexer;
pub mod parser;

use num::ToPrimitive;

pub type Number = num::rational::BigRational;

pub trait Math {
    fn pow(self, exp: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

impl Math for Number {
    fn pow(self, exp: Self) -> Self {
        // TODO work with rationals
        num::pow::Pow::pow(self, exp.to_integer())
    }

    fn sin(self) -> Self {
        // TODO don't use f64
        Number::from_float(self.to_f64().unwrap().sin()).unwrap()
    }

    fn cos(self) -> Self {
        // TODO don't use f64
        Number::from_float(self.to_f64().unwrap().cos()).unwrap()
    }
}

use once_cell::sync::Lazy;
pub static PI: Lazy<Number> = Lazy::new(|| Number::from_float(std::f64::consts::PI).unwrap());

#[derive(Debug)]
pub enum Action {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Sine,
    Cosine,
}

#[derive(Debug)]
pub enum Token {
    Num(Number),
    Act(Action),
    Lparenthesis,
    Rparenthesis,
}

#[derive(Debug)]
pub enum UnaryAct {
    Positive,
    Negative,
    Sine,
    Cosine,
}

#[derive(Debug)]
pub enum BinaryAct {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug)]
pub enum Node {
    Num(Number),
    UnaryExpr {
        op: UnaryAct,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryAct,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn lexer() {
        todo!();
    }

    #[test]
    fn parser() {
        todo!();
    }

    #[test]
    fn interpret() {
        todo!();
    }
}
