use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
    LeftParen,
    RightParen,
    Modulus,
    Exponent,
}

pub fn tokenize(input: String) -> Result<VecDeque<Token>, String> {
    let plus_re = Regex::new(r"^\+").unwrap();
    let minus_re = Regex::new(r"^-").unwrap();
    let times_re = Regex::new(r"^\*").unwrap();
    let divide_re = Regex::new(r"^/").unwrap();
    let left_paren_re = Regex::new(r"^\(").unwrap();
    let right_paren_re = Regex::new(r"^\)").unwrap();
    let number_re = Regex::new(r"^\d+(\.\d+)?").unwrap();
    let modulus_re = Regex::new(r"^\%").unwrap();
    let exponent_re = Regex::new(r"^\^").unwrap();
    let mut input = input.trim();
    let mut tokens = VecDeque::new();
    if input.is_empty(){
        return Err(" ".to_owned())
    }
    while !input.is_empty() {
        if plus_re.is_match(input) {
            tokens.push_back(Token::Plus);
            input = &input[1..];
        } else if minus_re.is_match(input) {
            tokens.push_back(Token::Minus);
            input = &input[1..];
        } else if times_re.is_match(input) {
            tokens.push_back(Token::Times);
            input = &input[1..];
        } else if divide_re.is_match(input) {
            tokens.push_back(Token::Divide);
            input = &input[1..];
        } else if modulus_re.is_match(input) {
            tokens.push_back(Token::Modulus);
            input = &input[1..];
        } else if exponent_re.is_match(input) {
            tokens.push_back(Token::Exponent);
            input = &input[1..];
        } else if left_paren_re.is_match(input) {
            tokens.push_back(Token::LeftParen);
            input = &input[1..];
        } else if right_paren_re.is_match(input) {
            tokens.push_back(Token::RightParen);
            input = &input[1..];
        } else if number_re.is_match(input) {
            let value = number_re.captures(input).unwrap()[0].to_string();
            let length = value.len();
            let value = value.parse::<f64>().unwrap();
            tokens.push_back(Token::Number(value));
            input = &input[length..];
        } else {
            println!("Parse failed on: {}", input);
            return Err(format!("Invalid Entry: {}", input))
        }
        input = input.trim();
    }
    return Ok(tokens);
}

#[cfg(test)]
mod test {
    use crate::tokens::{Token, tokenize};

    #[test]
    fn numbers_tokenize_correctly() {
        assert_eq!(tokenize("123".to_string()).unwrap(), vec![Token::Number(123.0)]);
        assert_eq!(tokenize("123.456".to_string()).unwrap(), vec![Token::Number(123.456)]);
    }

    #[test]
    fn simple_expression() {
        assert_eq!(tokenize("1+2".to_string()).unwrap(), vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)])
    }

    #[test]
    fn symbols_tokenize_correctly() {
        let input = "+ - * / ( 123 )".to_string();
        let input = tokenize(input).unwrap();
        assert_eq!(input, vec![Token::Plus, Token::Minus, Token::Times, Token::Divide, Token::LeftParen, Token::Number(123.0), Token::RightParen]);
    }
    #[test]
    fn negatives_tokenize_correctly() {
        let input = tokenize("-2".to_string()).unwrap();
        assert_eq!(input, vec![Token::Minus, Token::Number(2.0)]);
        let input = tokenize("--2".to_string()).unwrap();
        assert_eq!(input, vec![Token::Minus, Token::Minus, Token::Number(2.0)]);
        let input = tokenize("3*-2".to_string()).unwrap();
        assert_eq!(input, vec![Token::Number(3.0), Token::Times, Token::Minus, Token::Number(2.0)]);
    }

    #[test]
    fn carrot_test() {
        let input = "5^5".to_string();
        let input = tokenize(input).unwrap();
        assert_eq!(input, vec![Token::Number(5.0), Token::Exponent, Token::Number(5.0)])
    }
}