use crate::structs::{
    Operator::{Asterisk, Minus, Plus, Slash},
    Token::{self, Dot, Lparenthesis, Op, Rparenthesis},
};

const DIGITS: &str = "0123456789";

pub fn tokenize(text: &String) -> Vec<Token> {
    let mut expression: Vec<Token> = Vec::new();
    let text: Vec<_> = text.chars().collect();

    let mut i = 0;
    while i < text.len() {
        let token = match text[i] {
            ' ' | '\n' => None,
            x if DIGITS.contains(x) => {
                let mut j = 1;
                while (i + j) < text.len() {
                    if !DIGITS.contains(text[i + j] as char) {
                        break;
                    }
                    j += 1;
                }
                let s: String = text[i..i + j].iter().collect();
                let number: u32 = s.parse::<u32>().unwrap();
                i += j - 1;
                Some(Token::Number(number))
            }
            '+' => Some(Op(Plus)),
            '-' => Some(Op(Minus)),
            '*' => Some(Op(Asterisk)),
            '/' => Some(Op(Slash)),
            '(' => Some(Lparenthesis),
            ')' => Some(Rparenthesis),
            '.' => Some(Dot),
            why => panic!("LexerError: '{}' is unexpected!", why),
        };
        if let Some(x) = token {
            expression.push(x);
        }
        i += 1;
    }

    expression
}
