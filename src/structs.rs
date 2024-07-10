pub type Number = num::rational::BigRational;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
}

#[derive(Debug)]
pub enum Token {
    Num(Number),
    Op(Operator),
    Lparenthesis,
    Rparenthesis,
}

#[derive(Debug)]
pub enum UnaryOp {
    Positive,
    Negative,
}

#[derive(Debug)]
pub enum BinaryOp {
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
        op: UnaryOp,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
