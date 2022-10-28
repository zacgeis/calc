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
    Mod(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Num(i64),
}

type ParseResult = Result<Option<Box<Node>>, String>;
type TokenIter<'a> = Peekable<std::slice::Iter<'a, Token>>;

// TODO: Update eval to return a result and handle things like divide by zero?
fn eval(node: &Node) -> i64 {
    match node {
        Node::Add(a, b) => eval(a) + eval(b),
        Node::Sub(a, b) => eval(a) - eval(b),
        Node::Mul(a, b) => eval(a) * eval(b),
        Node::Div(a, b) => eval(a) / eval(b),
        Node::Mod(a, b) => eval(a) % eval(b),
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
    let left = match parse_unary(tokens)? {
        Some(node) => node,
        None => return Ok(None),
    };
    let op = match tokens.next() {
        Some(token) => token,
        None => return Ok(Some(left)),
    };
    let right = match parse_exp(tokens)? {
        Some(node) => node,
        None => return Err("Right side of expression missing.".to_string()),
    };
    let node = match op {
        Token::Plus => Box::new(Node::Add(left, right)),
        Token::Minus => Box::new(Node::Sub(left, right)),
        Token::Asterisk => Box::new(Node::Mul(left, right)),
        Token::BackSlash => Box::new(Node::Div(left, right)),
        Token::Percent => Box::new(Node::Mod(left, right)),
        Token::Caret => Box::new(Node::Pow(left, right)),
        _ => return Err(format!("Unexpected token: {:?}.", op)),
    };
    Ok(Some(node))
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
            Ok(Some(Box::new(Node::Neg(right))))
        }
        None => Ok(None),
        _ => parse_number(tokens),
    }
}

fn parse_number(tokens: &mut TokenIter) -> ParseResult {
    match tokens.next() {
        Some(Token::Num(a)) => Ok(Some(Box::new(Node::Num(*a)))),
        _ => Err("todo error.".to_string()),
    }
}

fn main() {
    let node_1 = Box::new(Node::Num(1));
    let node_2 = Box::new(Node::Num(2));
    let node_3 = Box::new(Node::Add(node_1, node_2));

    println!("nodes: {:?}", &node_3);
    println!("eval: {:?}", eval(&node_3));

    let node_4 = parse("-1234 + 1234").unwrap().unwrap();
    println!("tree: {:?}, eval: {:?}", &node_4, eval(&node_4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(parse("1234"), Ok(Some(Box::new(Node::Num(1234)))));
        assert_eq!(
            parse("-1234"),
            Ok(Some(Box::new(Node::Neg(Box::new(Node::Num(1234))))))
        );
    }

    #[test]
    fn test_eval() {
    }
}
