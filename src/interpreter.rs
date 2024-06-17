use crate::structs::{Node, Operator};

pub fn interpret(node: Node) -> i32 {
    match node {
        Node::Int(n) => n,
        Node::UnaryExpr { op, child } => {
            let child = interpret(*child);
            match op {
                Operator::Plus => child,
                Operator::Minus => -child,
                _ => panic!(),
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = interpret(*lhs);
            let rhs = interpret(*rhs);

            match op {
                Operator::Plus => lhs + rhs,
                Operator::Minus => lhs - rhs,
                Operator::Asterisk => lhs * rhs,
            }
        }
    }
}
