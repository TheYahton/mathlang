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

pub fn ast(mut expression: Vec<Token>) -> Node {
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Node> = Vec::new();

    while !expression.is_empty() {
        let token = expression.remove(0);
        match token {
            Token::Number(k) => output.push(Node::Int(k as i32)),
            Token::Op(k) => {
                if !stack.is_empty() {
                    while !stack.is_empty()
                        && match stack[stack.len() - 1] {
                            Token::Op(_) => true,
                            _ => false,
                        }
                    {
                        let element = stack.remove(stack.len() - 1);
                        let node = token2node(element, &mut output);
                        output.push(node);
                    }
                }
                stack.push(Token::Op(k));
            }
            Token::Lparenthesis => {
                let mut wtf: Vec<Token> = Vec::new();
                loop {
                    wtf.push(expression.remove(0));
                    if matches!(expression[0], Token::Rparenthesis) {
                        expression.remove(0);
                        break;
                    }
                }
                println!("{:?}", wtf);
                output.push(ast(wtf));
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
