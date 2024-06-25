pub mod interpreter;
pub mod lexer;
pub mod parser;
mod structs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::BinaryOp::*;
    use crate::structs::Node::{self, *};
    use crate::structs::Operator::*;
    use crate::structs::Token::{self, *};

    #[test]
    fn lexer() {
        let result = lexer::tokenize(&"1+7-9".to_string());
        let _goal = vec![
            Token::Int(1),
            Op(Plus),
            Token::Int(7),
            Op(Minus),
            Token::Int(9),
        ];
        assert!(matches!(result, _goal));
    }

    #[test]
    fn parser() {
        let result = parser::ast(vec![Token::Int(3), Op(Plus), Token::Int(4)]);
        let _goal = BinaryExpr {
            op: Add,
            lhs: Box::new(Node::Int(3)),
            rhs: Box::new(Node::Int(4)),
        };
        assert!(matches!(result, _goal));
    }
}
