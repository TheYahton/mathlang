use crate::{
    Action::*,
    Number,
    Token::{self, *},
    PI,
};
use num::bigint::BigInt;

fn parse_number(index: usize, text: &Vec<char>) -> (usize, Number) {
    let mut j = 1;
    while (index + j) < text.len() && text[index + j].is_digit(10) {
        j += 1;
    }

    (
        j - 1,
        text[index..index + j]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap(),
    )
}

fn is_word(index: usize, text: &Vec<char>, word: &str) -> bool {
    let letters: Vec<char> = word.chars().collect();
    let mut j = 0;
    while index + j < text.len() && j < letters.len() {
        if !(text[index + j] == letters[j]) {
            return false;
        }
        j += 1;
    }
    return true;
}

pub fn tokenize(text: &String) -> Vec<Token> {
    let mut expression: Vec<Token> = Vec::new();
    let text: Vec<char> = text.chars().collect();

    let mut index = 0;
    while index < text.len() {
        let token = match text[index] {
            ' ' | '\n' => None,
            x if x.is_digit(10) => {
                let (num_length, integer) = parse_number(index, &text);
                index += num_length;
                if text.get(index + 1) == Some(&'.') {
                    index += 2;
                    let (num_length, fractional) = parse_number(index, &text);
                    index += num_length;
                    Some(Num(
                        integer + fractional / BigInt::from(10).pow(num_length as u32 + 1)
                    ))
                } else {
                    Some(Num(integer))
                }
            }
            'π' => Some(Num(PI.clone())),
            '+' => Some(Act(Plus)),
            '-' => Some(Act(Minus)),
            '*' => Some(Act(Asterisk)),
            '/' => Some(Act(Slash)),
            '^' => Some(Act(Caret)),
            '(' => Some(Lparenthesis),
            ')' => Some(Rparenthesis),
            _ if is_word(index, &text, "sin") => {
                index += 2;
                Some(Act(Sine))
            }
            _ if is_word(index, &text, "cos") => {
                index += 2;
                Some(Act(Cosine))
            }
            why => panic!("LexerError: '{}' is unexpected!", why),
        };
        if let Some(x) = token {
            expression.push(x);
        }
        index += 1;
    }

    expression
}
