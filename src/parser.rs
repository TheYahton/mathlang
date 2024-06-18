use crate::structs::{Node, Token};

fn token2node(token: Token, output: &mut Vec<Node>) -> Node {
    match token {
        Token::Op(k) => {
            if output.len() >= 2 {
                let op = k;
                let rhs = Box::new(output.pop().unwrap());
                let lhs = Box::new(output.pop().unwrap());

                return Node::BinaryExpr { op, lhs, rhs };
            } else if output.len() == 1 {
                let op = k;
                let child = Box::new(output.pop().unwrap());

                return Node::UnaryExpr { op, child };
            } else {
                panic!()
            }
        }
        _ => panic!(),
    }
}

fn to_fractional(k: u32) -> f64 {
    // 18272 -> 0.18272
    k as f64 / 10.0f64.powi((k.checked_ilog10().unwrap_or(0) + 1) as i32)
}

pub fn ast(mut expression: Vec<Token>) -> Node {
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Node> = Vec::new();

    while !expression.is_empty() {
        let token = expression.remove(0);
        match token {
            Token::Number(k) => output.push(Node::Int(k as i32)),
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
                let mut wtf: Vec<Token> = Vec::new();
                let mut l_counter = 0;
                let mut r_counter = 0;
                let mut end_index = 0;
                let mut i = 0;
                while i < expression.len() {
                    match expression[i] {
                        Token::Lparenthesis => l_counter += 1,
                        Token::Rparenthesis => r_counter += 1,
                        _ => (),
                    }
                    if r_counter > l_counter {
                        end_index = i;
                        break;
                    }
                    i += 1;
                }

                for _ in 0..end_index {
                    wtf.push(expression.remove(0));
                }
                expression.remove(0);
                output.push(ast(wtf));
            }
            Token::Rparenthesis => panic!(),
            Token::Dot => {
                if !expression.is_empty() {
                    match &expression[0] {
                        &Token::Number(k) => {
                            if !output.is_empty()
                                && matches!(output[output.len() - 1], Node::Int(_))
                            {
                                let integer: f64 = match output.pop().unwrap() {
                                    Node::Int(k) => k as f64,
                                    _ => panic!(),
                                };
                                let fractional: f64 = to_fractional(k);
                                output.push(Node::Float(integer + fractional));
                                expression.remove(0);
                            } else {
                                let fractional = match expression.remove(0) {
                                    Token::Number(k) => to_fractional(k),
                                    _ => panic!(),
                                };
                                output.push(Node::Float(fractional));
                            }
                        }
                        other => {
                            let integer = Node::Float(match output.pop().unwrap() {
                                Node::Int(k) => k as f64,
                                _ => panic!(),
                            });
                            output.push(integer);
                        }
                    }
                } else {
                    let item = Node::Float(match output.pop().unwrap() {
                        Node::Int(k) => k as f64,
                        _ => panic!(),
                    });
                    output.push(item);
                }
            }
        }
    }

    while !stack.is_empty() {
        let element = stack.pop().unwrap();
        let node = token2node(element, &mut output);
        output.push(node);
    }
    output.pop().unwrap()
}
