use crate::structs::{BinaryOp, Node, Number, UnaryOp};

pub fn interpret(node: Node) -> Number {
    match node {
        Node::Num(k) => k,
        Node::UnaryExpr { op, child } => {
            let child = interpret(*child);
            match op {
                UnaryOp::Positive => child,
                UnaryOp::Negative => -child,
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = interpret(*lhs);
            let rhs = interpret(*rhs);

            match op {
                BinaryOp::Add => lhs + rhs,
                BinaryOp::Subtract => lhs - rhs,
                BinaryOp::Multiply => lhs * rhs,
                BinaryOp::Divide => lhs / rhs,
                BinaryOp::Power => num::pow::Pow::pow(lhs, rhs.to_integer()),
            }
        }
    }
}
