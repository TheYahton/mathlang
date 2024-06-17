#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Lparenthesis,
    Rparenthesis,
    Dot,
}

#[derive(Debug)]
pub enum Node {
    Int(i32),
    Float(f64),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
