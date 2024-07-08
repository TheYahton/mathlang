use crate::structs::{BinaryOp, Node, Operator, Token, UnaryOp};

fn op2unary(k: Operator) -> UnaryOp {
    match k {
        Operator::Plus => UnaryOp::Positive,
        Operator::Minus => UnaryOp::Negative,
        _ => panic!("Оператор {:?} не может быть унарным.", k),
    }
}

fn op2binary(k: Operator) -> BinaryOp {
    match k {
        Operator::Plus => BinaryOp::Add,
        Operator::Minus => BinaryOp::Subtract,
        Operator::Asterisk => BinaryOp::Multiply,
        Operator::Slash => BinaryOp::Divide,
        Operator::Caret => BinaryOp::Power,
    }
}

fn token2node(k: Operator, output: &mut Vec<Node>) -> Node {
    if output.len() >= 2 {
        let op = op2binary(k);
        let rhs = Box::new(output.pop().unwrap());
        let lhs = Box::new(output.pop().unwrap());

        return Node::BinaryExpr { op, lhs, rhs };
    } else if output.len() == 1 {
        let op = op2unary(k);
        let child = Box::new(output.pop().unwrap());

        return Node::UnaryExpr { op, child };
    } else {
        panic!("Оператор {:?} не к чему применить.", k)
    }
}

pub fn ast(mut expression: Vec<Token>) -> Node {
    expression.reverse();

    let mut stack: Vec<Operator> = Vec::new();
    let mut output: Vec<Node> = Vec::new();

    while !expression.is_empty() {
        let token = expression.pop().unwrap();
        match token {
            Token::Int(k) => output.push(Node::Int(k as i32)),
            Token::Float(k) => output.push(Node::Float(k)),
            Token::Op(k) => {
                if !stack.is_empty() {
                    while !stack.is_empty() {
                        let element: Operator = stack.remove(stack.len() - 1);
                        let node = token2node(element, &mut output);
                        output.push(node);
                    }
                }
                stack.push(k);
            }
            Token::Lparenthesis => {
                let mut internal: Vec<Token> = Vec::new();
                let mut n: i32 = 1;
                while !expression.is_empty() {
                    let token = expression.pop().unwrap();
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
                if expression.is_empty() && n != 0 {
                    panic!("Синтаксическая ошибка: '(' не закрывается");
                }
                output.push(ast(internal));
            }
            Token::Rparenthesis => panic!("Синтаксическая ошибка: неожиданное ')'"),
        }
    }

    while !stack.is_empty() {
        let element = stack.pop().unwrap();
        let node = token2node(element, &mut output);
        output.push(node);
    }
    output.pop().expect("Входное выражение пусто.")
}
