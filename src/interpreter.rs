use crate::structs::{BinaryOp, Node, UnaryOp};

pub fn interpret(node: Node) -> f64 {
    match node {
        Node::Int(n) => n as f64,
        Node::Float(x) => x,
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
                BinaryOp::Power => lhs.powf(rhs),
            }
        }
    }
}
