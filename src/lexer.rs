use crate::structs::{
    Operator::*,
    Token::{self, *},
};

fn parse_number(index: usize, text: &Vec<char>) -> (usize, String) {
    let mut j = 1;
    while (index + j) < text.len() && text[index + j].is_digit(10) {
        j += 1;
    }

    (j - 1, text[index..index + j].iter().collect())
}

pub fn tokenize(text: &String) -> Vec<Token> {
    let mut expression: Vec<Token> = Vec::new();
    let text: Vec<char> = text.chars().collect();

    let mut index = 0;
    while index < text.len() {
        let token = match text[index] {
            ' ' | '\n' => None,
            x if x.is_digit(10) => {
                let (delta_index, integer) = parse_number(index, &text);
                index += delta_index;
                if index + 1 < text.len() && text[index + 1] == '.' {
                    index += 2;
                    let (delta_index, fractional) = parse_number(index, &text);
                    index += delta_index;
                    Some(Float((integer + "." + &fractional).parse().unwrap()))
                } else {
                    Some(Int(integer.parse().unwrap()))
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
        index += 1;
    }

    expression
}
