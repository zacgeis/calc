mod lex;

use lex::{tokenize, Token};
use std::iter::Peekable;

// this is a cool approach, but matching becomes more annoying.
// struct NodeBoxed(Node<Box<NodeBoxed>>);

#[derive(Debug, Eq, PartialEq)]
enum Node {
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
    fn add(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Add(left, right))
    }
    fn sub(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Sub(left, right))
    }
    fn mul(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Mul(left, right))
    }
    fn div(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Div(left, right))
    }
    fn modu(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Modu(left, right))
    }
    fn pow(left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self::Pow(left, right))
    }
    fn neg(value: Box<Node>) -> Box<Self> {
        Box::new(Self::Neg(value))
    }
    fn num(value: i64) -> Box<Self> {
        Box::new(Self::Num(value))
    }
}

type ParseResult = Result<Option<Box<Node>>, String>;
type TokenIter<'a> = Peekable<std::slice::Iter<'a, Token>>;

fn eval_str(input: &str) -> i64 {
    let root_node = parse(input).unwrap().unwrap();
    eval(&root_node)
}

// TODO: Update eval to return a result and handle things like divide by zero?
fn eval(node: &Node) -> i64 {
    match node {
        Node::Add(a, b) => eval(a) + eval(b),
        Node::Sub(a, b) => eval(a) - eval(b),
        Node::Mul(a, b) => eval(a) * eval(b),
        Node::Div(a, b) => eval(a) / eval(b),
        Node::Modu(a, b) => eval(a) % eval(b),
        Node::Pow(a, b) => i64::pow(eval(a), eval(b) as u32),
        Node::Neg(a) => -eval(a),
        Node::Num(a) => *a,
    }
}

fn parse(input: &str) -> ParseResult {
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

fn main() {
    let node = parse("1 - 2 - 3").unwrap().unwrap();
    println!("tree: {:?}, eval: {:?}", &node, eval(&node));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_nodes() {
        let node_1 = Node::num(1);
        let node_2 = Node::num(2);
        let node_3 = Node::add(node_1, node_2);

        assert_eq!(eval(&node_3), 3);
    }

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

    // TODO: Maybe add some kind of fuzzing here?
    #[test]
    fn test_eval() {
        assert_eq!(eval_str("1"), 1);

        assert_eq!(eval_str("1 + 1"), 2);
        assert_eq!(eval_str("1 + 1 + 1"), 3);
        assert_eq!(eval_str("1 - 1"), 0);

        assert_eq!(eval_str("1 - 2 - 3"), -4);

        assert_eq!(eval_str("2 ^ 4 + 1"), 17);
        assert_eq!(eval_str("1 + 2 ^ 4"), 17);
    }
}
