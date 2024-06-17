#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Lparenthesis,
    Rparenthesis,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Int(i32),
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
