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
            Token::Plus => Box::new(Node::Add(left, right)),
            Token::Minus => Box::new(Node::Sub(left, right)),
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
            Token::Asterisk => Box::new(Node::Mul(left, right)),
            Token::BackSlash => Box::new(Node::Div(left, right)),
            Token::Percent => Box::new(Node::Mod(left, right)),
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
            Token::Caret => Box::new(Node::Pow(left, right)),
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
    let node = parse("1 - 2 - 3").unwrap().unwrap();
    println!("tree: {:?}, eval: {:?}", &node, eval(&node));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_nodes() {
        let node_1 = Box::new(Node::Num(1));
        let node_2 = Box::new(Node::Num(2));
        let node_3 = Box::new(Node::Add(node_1, node_2));

        assert_eq!(eval(&node_3), 3);
    }

    #[test]
    fn test_parsing() {
        assert_eq!(parse("1234"), Ok(Some(Box::new(Node::Num(1234)))));
        assert_eq!(
            parse("-1234"),
            Ok(Some(Box::new(Node::Neg(Box::new(Node::Num(1234))))))
        );
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
