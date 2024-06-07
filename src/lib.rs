pub mod lexer;
pub mod parser;


#[derive(Debug, PartialEq)]
pub enum Digits {
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

#[derive(Debug, PartialEq)]
pub enum Operators {
    PLUS,
    MINUS,
}

#[derive(Debug, PartialEq)]
pub enum Tokens {
    NUMBER(Digits),
    OP(Operators),
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tokens::*;
    use Operators::*;
    use Digits::*;

    /*#[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/

    #[test]
    fn token() {
        let result = lexer::tokenize(&"1+7-9".to_string());
        assert_eq!(result, vec![NUMBER(ONE), OP(PLUS), NUMBER(SEVEN), OP(MINUS), NUMBER(NINE)]);
    }

    #[test]
    fn parse() {
        let result = parser::ass(vec![NUMBER(THREE), OP(PLUS), NUMBER(FOUR)]);
        assert_eq!(result, vec![NUMBER(THREE), NUMBER(FOUR), OP(PLUS)]);
    }
}
