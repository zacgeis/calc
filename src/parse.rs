use std::iter::Peekable;

use crate::lex::{tokenize, Token};

// this is a cool approach, but matching becomes more annoying.
// struct NodeBoxed(Node<Box<NodeBoxed>>);

#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Modu(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Num(i64),
}

impl Node {
    pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Add(left, right))
    }
    pub fn sub(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Sub(left, right))
    }
    pub fn mul(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Mul(left, right))
    }
    pub fn div(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Div(left, right))
    }
    pub fn modu(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Modu(left, right))
    }
    pub fn pow(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Pow(left, right))
    }
    pub fn neg(value: Box<Node>) -> Box<Self> {
        Box::new(Self::Neg(value))
    }
    pub fn num(value: i64) -> Box<Self> {
        Box::new(Self::Num(value))
    }
}

type ParseResult = Result<Option<Box<Node>>, String>;
type TokenIter<'a> = Peekable<std::slice::Iter<'a, Token>>;

pub fn parse(input: &str) -> ParseResult {
    let tokens = tokenize(input)?;
    let mut token_iter = tokens.iter().peekable();
    parse_exp(&mut token_iter)
}

fn parse_exp(tokens: &mut TokenIter) -> ParseResult {
    let mut left = match parse_exp_l1(tokens)? {
        Some(node) => node,
        None => return Ok(None),
    };
    loop {
        let op = match tokens.peek() {
            Some(Token::Plus | Token::Minus) => tokens.next().unwrap(),
            _ => break,
        };
        let right = match parse_exp_l1(tokens)? {
            Some(node) => node,
            None => return Err("Right side of expression missing.".to_string()),
        };
        let node = match op {
            Token::Plus => Node::add(left, right),
            Token::Minus => Node::sub(left, right),
            _ => return Err(format!("Unexpected token: {:?}.", op)),
        };
        left = node;
    }
    Ok(Some(left))
}

fn parse_exp_l1(tokens: &mut TokenIter) -> ParseResult {
    let mut left = match parse_exp_l2(tokens)? {
        Some(node) => node,
        None => return Ok(None),
    };
    loop {
        let op = match tokens.peek() {
            Some(Token::Asterisk | Token::BackSlash | Token::Percent) => tokens.next().unwrap(),
            _ => break,
        };
        let right = match parse_exp_l2(tokens)? {
            Some(node) => node,
            None => return Err("Right side of expression missing.".to_string()),
        };
        let node = match op {
            Token::Asterisk => Node::mul(left, right),
            Token::BackSlash => Node::div(left, right),
            Token::Percent => Node::modu(left, right),
            _ => return Err(format!("Unexpected token: {:?}.", op)),
        };
        left = node;
    }
    Ok(Some(left))
}

fn parse_exp_l2(tokens: &mut TokenIter) -> ParseResult {
    let mut left = match parse_unary(tokens)? {
        Some(node) => node,
        None => return Ok(None),
    };
    loop {
        let op = match tokens.peek() {
            Some(Token::Caret) => tokens.next().unwrap(),
            _ => break,
        };
        let right = match parse_unary(tokens)? {
            Some(node) => node,
            None => return Err("Right side of expression missing.".to_string()),
        };
        let node = match op {
            Token::Caret => Node::pow(left, right),
            _ => return Err(format!("Unexpected token: {:?}.", op)),
        };
        left = node;
    }
    Ok(Some(left))
}

fn parse_unary(tokens: &mut TokenIter) -> ParseResult {
    match tokens.peek() {
        Some(Token::OpenParen) => {
            tokens.next();
            let exp = parse_exp(tokens);
            match tokens.next() {
                Some(Token::CloseParen) => (),
                _ => return Err("Expected a closing paren.".to_string()),
            }
            exp
        }
        Some(Token::Minus) => {
            tokens.next();
            let right = match parse_unary(tokens)? {
                Some(node) => node,
                None => return Err("Expected expression.".to_string()),
            };
            Ok(Some(Node::neg(right)))
        }
        None => Ok(None),
        _ => parse_number(tokens),
    }
}

fn parse_number(tokens: &mut TokenIter) -> ParseResult {
    match tokens.next() {
        Some(Token::Num(a)) => Ok(Some(Node::num(*a))),
        _ => Err("todo error.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let expected = Node::num(1234);
        assert_eq!(parse("1234"), Ok(Some(expected)));

        let expected = Node::neg(Node::num(1234));
        assert_eq!(parse("-1234"), Ok(Some(expected)));

        let expected = Node::add(Node::num(1), Node::num(2));
        assert_eq!(parse("1 + 2"), Ok(Some(expected)));

        // TODO: add more test cases here and split out the eval part into a separate file.
    }
}
