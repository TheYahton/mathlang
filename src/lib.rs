pub mod lexer;
pub mod parser;

use lexer::tokenize;
use parser::ass;

#[derive(Debug)]
enum Digits {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

#[derive(Debug)]
enum Operators {
    PLUS,
    MINUS,
}

#[derive(Debug)]
pub enum Tokens {
    NUMBER(Digits),
    OP(Operators),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
