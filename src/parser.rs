use crate::{
    consts,
    Action::{self, *},
    BinaryAct::*,
    Node, Token,
    UnaryAct::{self, *},
};

fn action2node(k: Action, output: &mut Vec<Node>) -> Node {
    let arg_count = output.len();
    match k {
        Action::Sine | Action::Cosine => {
            if arg_count < 1 {
                panic!(
                    "Действию {:?} необходим 1 аргумент. Сейчас их {}",
                    k, arg_count
                )
            }
            Node::UnaryExpr {
                op: match k {
                    Action::Sine => UnaryAct::Sine,
                    Action::Cosine => UnaryAct::Cosine,
                    _ => unreachable!(),
                },
                child: Box::new(output.pop().unwrap()),
            }
        }
        Asterisk | Slash | Caret => {
            if arg_count < 2 {
                panic!(
                    "Действию {:?} необходимо 2 аргумента. Сейчас их {}",
                    k, arg_count
                )
            }
            Node::BinaryExpr {
                op: match k {
                    Asterisk => Multiply,
                    Slash => Divide,
                    Caret => Power,
                    _ => unreachable!(),
                },
                rhs: Box::new(output.pop().unwrap()),
                lhs: Box::new(output.pop().unwrap()),
            }
        }
        Plus | Minus => {
            if arg_count >= 2 {
                Node::BinaryExpr {
                    op: match k {
                        Plus => Add,
                        Minus => Subtract,
                        _ => unreachable!(),
                    },
                    rhs: Box::new(output.pop().unwrap()),
                    lhs: Box::new(output.pop().unwrap()),
                }
            } else if arg_count == 1 {
                Node::UnaryExpr {
                    op: match k {
                        Plus => Positive,
                        Minus => Negative,
                        _ => unreachable!(),
                    },
                    child: Box::new(output.pop().unwrap()),
                }
            } else {
                panic!(
                    "Действию {:?} необходимо 1 или 2 аргумента. Сейчас их {}",
                    k, arg_count
                )
            }
        }
    }
}

pub fn ast(mut expression: Vec<Token>) -> Node {
    expression.reverse();

    let mut stack: Vec<Action> = Vec::new();
    let mut output: Vec<Node> = Vec::new();

    while !expression.is_empty() {
        let token = expression.pop().unwrap();
        match token {
            Token::Num(x) => output.push(Node::Num(x)),
            Token::Act(k) => {
                if !stack.is_empty() {
                    while !stack.is_empty() {
                        let action = stack.remove(stack.len() - 1);
                        let node = action2node(action, &mut output);
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
        let action = stack.pop().unwrap();
        let node = action2node(action, &mut output);
        output.push(node);
    }

    match output.pop() {
        Some(x) => x,
        None => Node::Num(consts::ZERO.clone()),
    }
}
