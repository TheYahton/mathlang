use crate::structs::{
    Operator::{Asterisk, Minus, Plus},
    Token::{self, Lparenthesis, Op, Rparenthesis},
};

const DIGITS: &str = "0123456789";

pub fn tokenize(text: &String) -> Vec<Token> {
    let mut expression: Vec<Token> = Vec::new();
    let fck: Vec<_> = text.chars().collect();

    let mut i = 0;
    while i < fck.len() {
        match fck[i] {
            ' ' | '\n' => (),
            x if DIGITS.contains(x) => {
                let mut j = 1;
                while (i + j) < fck.len() {
                    if !DIGITS.contains(fck[i + j] as char) {
                        break;
                    }
                    j += 1;
                }
                let s: String = fck[i..i + j].iter().collect();
                let number: u32 = s.parse::<u32>().unwrap();
                expression.push(Token::Number(number));
                i += j - 1;
            }
            '+' => expression.push(Op(Plus)),
            '-' => expression.push(Op(Minus)),
            '*' => expression.push(Op(Asterisk)),
            '(' => expression.push(Lparenthesis),
            ')' => expression.push(Rparenthesis),
            why => panic!("LexerError: '{}' is unexpected!", why),
        }
        i += 1;
    }

    expression
}
