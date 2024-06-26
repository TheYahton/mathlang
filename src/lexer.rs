use crate::structs::{
    Operator::*,
    Token::{self, *},
};

fn parse_number(index: &mut usize, text: &Vec<char>) -> u32 {
    let mut j = 1;
    while (*index + j) < text.len() {
        if !text[*index + j].is_digit(10) {
            break;
        }
        j += 1;
    }
    let s: String = text[*index..*index + j].iter().collect();
    *index += j - 1;

    s.parse().unwrap()
}

fn to_fractional(k: u32) -> f64 {
    // 18272 -> 0.18272
    k as f64 / 10.0f64.powi((k.checked_ilog10().unwrap_or(0) + 1) as i32)
}

pub fn tokenize(text: &String) -> Vec<Token> {
    let mut expression: Vec<Token> = Vec::new();
    let text: Vec<char> = text.chars().collect();

    let mut i = 0;
    while i < text.len() {
        let token = match text[i] {
            ' ' | '\n' => None,
            x if x.is_digit(10) => {
                let integer: u32 = parse_number(&mut i, &text);
                if i + 1 < text.len() && text[i + 1] == '.' {
                    i += 2;
                    let fractional: f64 = to_fractional(parse_number(&mut i, &text));
                    Some(Float(integer as f64 + fractional))
                } else {
                    Some(Int(integer))
                }
            }
            '+' => Some(Op(Plus)),
            '-' => Some(Op(Minus)),
            '*' => Some(Op(Asterisk)),
            '/' => Some(Op(Slash)),
            '^' => Some(Op(Caret)),
            '(' => Some(Lparenthesis),
            ')' => Some(Rparenthesis),
            why => panic!("LexerError: '{}' is unexpected!", why),
        };
        if let Some(x) = token {
            expression.push(x);
        }
        i += 1;
    }

    expression
}
