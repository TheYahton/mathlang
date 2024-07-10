use crate::structs::{
    Action::{self, *},
    BinaryAct::{self, *},
    Node, Token,
    UnaryAct::{self, *},
};

fn act2unary(k: &Action) -> Result<UnaryAct, ()> {
    match k {
        Plus => Ok(Positive),
        Minus => Ok(Negative),
        Sinus => Ok(Sin),
        Cosin => Ok(Cos),
        _ => panic!("Оператор {:?} не может быть унарным.", k),
    }
}

fn act2binary(k: &Action) -> Result<BinaryAct, ()> {
    match k {
        Plus => Ok(Add),
        Minus => Ok(Subtract),
        Asterisk => Ok(Multiply),
        Slash => Ok(Divide),
        Caret => Ok(Power),
        Sinus | Cosin => Err(()),
    }
}

fn token2node(k: Action, output: &mut Vec<Node>) -> Node {
    if let Ok(op) = act2binary(&k) {
        let rhs = Box::new(output.pop().unwrap());
        let lhs = Box::new(output.pop().unwrap());
        return Node::BinaryExpr { op, lhs, rhs };
    } else {
        if let Ok(op) = act2unary(&k) {
            let child = Box::new(output.pop().unwrap());
            return Node::UnaryExpr { op, child };
        }
    }
    panic!("Оператор {:?} не к чему применить.", k)
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
                        let element = stack.remove(stack.len() - 1);
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
