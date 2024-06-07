use crate::{Digits, Operators, Tokens};

pub fn tokenize(text: &String) -> Vec<Tokens> {
    let text = text.replace("\n", "").replace(" ", "");

    let mut expression: Vec<Tokens> = Vec::new();
    for symbol in text.chars() {
        let token: Tokens = match symbol {
            '0' => Tokens::NUMBER(Digits::ZERO),
            '1' => Tokens::NUMBER(Digits::ONE),
            '2' => Tokens::NUMBER(Digits::TWO),
            '3' => Tokens::NUMBER(Digits::THREE),
            '4' => Tokens::NUMBER(Digits::FOUR),
            '5' => Tokens::NUMBER(Digits::FIVE),
            '6' => Tokens::NUMBER(Digits::SIX),
            '7' => Tokens::NUMBER(Digits::SEVEN),
            '8' => Tokens::NUMBER(Digits::EIGHT),
            '9' => Tokens::NUMBER(Digits::NINE),
            '+' => Tokens::OP(Operators::PLUS),
            '-' => Tokens::OP(Operators::MINUS),
            s => panic!("'{}' is unexpected", s),
        };

        expression.push(token);
    }

    expression
}
