use crate::structs::{BinaryOp, Node, Operator, Token, UnaryOp};

fn token2node(token: Token, output: &mut Vec<Node>) -> Node {
    match token {
        Token::Op(k) => {
            if output.len() >= 2 {
                let op = match k {
                    Operator::Plus => BinaryOp::Add,
                    Operator::Minus => BinaryOp::Subtract,
                    Operator::Asterisk => BinaryOp::Multiply,
                    Operator::Slash => BinaryOp::Divide,
                };
                let rhs = Box::new(output.pop().unwrap());
                let lhs = Box::new(output.pop().unwrap());

                return Node::BinaryExpr { op, lhs, rhs };
            } else if output.len() == 1 {
                let op = match k {
                    Operator::Plus => UnaryOp::Positive,
                    Operator::Minus => UnaryOp::Negative,
                    _ => panic!(),
                };
                let child = Box::new(output.pop().unwrap());

                return Node::UnaryExpr { op, child };
            } else {
                panic!()
            }
        }
        _ => panic!(),
    }
}

pub fn ast(mut expression: Vec<Token>) -> Node {
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Node> = Vec::new();

    while !expression.is_empty() {
        let token = expression.remove(0);
        match token {
            Token::Int(k) => output.push(Node::Int(k as i32)),
            Token::Float(k) => output.push(Node::Float(k)),
            Token::Op(k) => {
                if !stack.is_empty() {
                    while !stack.is_empty() && matches!(stack[stack.len() - 1], Token::Op(_)) {
                        let element = stack.remove(stack.len() - 1);
                        let node = token2node(element, &mut output);
                        output.push(node);
                    }
                }
                stack.push(Token::Op(k));
            }
            Token::Lparenthesis => {
                let mut internal: Vec<Token> = Vec::new();
                let mut n: i32 = 1;
                while !expression.is_empty() {
                    let token = expression.remove(0);
                    match token {
                        Token::Lparenthesis => n += 1,
                        Token::Rparenthesis => n -= 1,
                        _ => (),
                    }
                    if n == 0 {
                        break;
                    }
                    internal.push(token);
                }
                output.push(ast(internal));
            }
            Token::Rparenthesis => panic!(),
        }
    }

    while !stack.is_empty() {
        let element = stack.pop().unwrap();
        let node = token2node(element, &mut output);
        output.push(node);
    }
    output.pop().unwrap()
}
