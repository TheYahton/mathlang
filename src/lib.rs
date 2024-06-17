pub mod interpreter;
pub mod lexer;
pub mod parser;
mod structs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Node::*;
    use crate::structs::Operator::*;
    use crate::structs::Token::*;

    #[test]
    fn lexer() {
        let result = lexer::tokenize(&"1+7-9".to_string());
        let goal = vec![Number(1), Op(Plus), Number(7), Op(Minus), Number(9)];
        assert!(matches!(result, goal));
    }

    #[test]
    fn parser() {
        let result = parser::ast(vec![Number(3), Op(Plus), Number(4)]);
        let goal = BinaryExpr {
            op: Plus,
            lhs: Box::new(Int(3)),
            rhs: Box::new(Int(4)),
        };
        assert!(matches!(result, goal));
    }
}
