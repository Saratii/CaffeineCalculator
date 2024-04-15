use std::collections::VecDeque;
use regex::Regex;
use crate::eval::FUNCTIONS;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
    Modulus,
    Exponent,
    LeftParen,
    RightParen,
    FunctionCall(String),
    Comma,
    Help,
    Graph,
    Variable(String),
    Equal,
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
    let function_re = Regex::new(r"^([a-z]+)\(").unwrap();
    let comma_re = Regex::new(r"^,").unwrap();
    let help_re = Regex::new(r"^help").unwrap();
    let graph_re = Regex::new(r"^graph\(").unwrap();
    let variable_re = Regex::new(r"^[a-z]+").unwrap();
    let eq_re = Regex::new(r"^=").unwrap();
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
        } else if function_re.is_match(input) && FUNCTIONS.contains(&function_re.captures(input).unwrap().get(1).unwrap().as_str()) {
            let capture = function_re.captures(input).unwrap();
            let matching_word = capture.get(1).unwrap().as_str();
            tokens.push_back(Token::FunctionCall(matching_word.to_string()));
            let length = matching_word.len() + 1;
            input = &input[length..];
        } else if comma_re.is_match(input) {
            tokens.push_back(Token::Comma);
            input = &input[1..];
        } else if help_re.is_match(input) {
            tokens.push_back(Token::Help);
            input = &input[4..];
        } else if graph_re.is_match(input) {
            tokens.push_back(Token::Graph);
            input = &input[6..];
        } else if eq_re.is_match(input) {
            tokens.push_back(Token::Equal);
            input = &input[1..];
        } else if variable_re.is_match(input) {
            let capture = variable_re.captures(input).unwrap();
            let matching_word = capture.get(0).unwrap().as_str();
            tokens.push_back(Token::Variable(matching_word.to_string()));
            input = &input[matching_word.len()..];
        } else if number_re.is_match(input) {
            let value = number_re.captures(input).unwrap()[0].to_string();
            let length = value.len();
            let value = value.parse::<f64>().unwrap();
            tokens.push_back(Token::Number(value));
            input = &input[length..];
        } else {
            return Err(format!("Syntax Error: {}", input))
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

    #[test]
    fn average_test() {
        let input = "average(1,2,3)".to_string();
        let input = tokenize(input).unwrap();
        assert_eq!(input, vec![Token::FunctionCall("average".to_string()), Token::Number(1.0), Token::Comma, Token::Number(2.0), Token::Comma, Token::Number(3.0), Token::RightParen])
    }

    #[test]
    fn graph_tokenizes_correctly() {
        let tokens = tokenize("graph(y=5x)".to_string()).unwrap();
        assert_eq!(tokens, vec![Token::Graph, Token::Variable("y".to_string()), Token::Equal, Token::Number(5.0), Token::Variable("x".to_string()), Token::RightParen]);
    }

    #[test]
    fn one_plus_ln() {
        let tokens = tokenize("1+ln(2)".to_string()).unwrap();
        assert_eq!(tokens, vec![Token::Number(1.), Token::Plus, Token::FunctionCall("ln".to_string()), Token::Number(2.), Token::RightParen]);
    }
}
