use crate::structs::{BinaryAct::*, Math, Node, Number, UnaryAct::*};

pub fn interpret(node: Node) -> Number {
    match node {
        Node::Num(k) => k,
        Node::UnaryExpr { op, child } => {
            let child = interpret(*child);
            match op {
                Positive => child,
                Negative => -child,
                Sin => child.sin(),
                Cos => child.cos(),
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = interpret(*lhs);
            let rhs = interpret(*rhs);

            match op {
                Add => lhs + rhs,
                Subtract => lhs - rhs,
                Multiply => lhs * rhs,
                Divide => rhs / lhs,
                Power => lhs.pow(rhs),
            }
        }
    }
}
