use crate::structs::{
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
                let (delta_index, integer) = parse_number(index, &text);
                index += delta_index;
                if index + 1 < text.len() && text[index + 1] == '.' {
                    index += 2;
                    let (delta_index, fractional) = parse_number(index, &text);
                    index += delta_index;
                    Some(Num(integer
                        + fractional.clone()
                            / BigInt::from(
                                10u64.pow(fractional.to_string().len() as u32),
                            )))
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
                Some(Act(Sinus))
            }
            _ if is_word(index, &text, "cos") => {
                index += 2;
                Some(Act(Cosin))
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
