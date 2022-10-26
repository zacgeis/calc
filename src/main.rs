mod lex;

use lex::tokenize;
use std::iter::Peekable;

// this is a cool approach, but matching becomes more annoying.
// struct NodeBoxed(Node<Box<NodeBoxed>>);

#[derive(Debug)]
enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Mod(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Lit(i32),
}
#[derive(Debug)]
struct ParseError {
    message: String,
}
type ParseResult = Result<Option<Box<Node>>, ParseError>;

fn eval(node: &Node) -> i32 {
    match node {
        Node::Add(a, b) => eval(a) + eval(b),
        Node::Sub(a, b) => eval(a) - eval(b),
        Node::Mul(a, b) => eval(a) * eval(b),
        Node::Div(a, b) => eval(a) / eval(b),
        Node::Mod(a, b) => eval(a) % eval(b),
        Node::Pow(a, b) => i32::pow(eval(a), eval(b) as u32),
        Node::Lit(a) => *a,
    }
}

fn parse(s: &str) -> ParseResult {
    let mut tokens = s.chars().peekable();
    parse_exp(&mut tokens)
}

fn is_num_char(c: &char) -> bool {
    c.is_numeric() || *c == '-'
}

fn parse_exp(tokens: &mut Peekable<impl Iterator<Item = char>>) -> ParseResult {
    let left = match parse_num(tokens) {
        Ok(Some(left)) => left,
        val => return val,
    };
    let op_char = match tokens.next() {
        Some(op_char) => op_char,
        None => return Ok(Some(left)),
    };
    let right = match parse_exp(tokens) {
        Ok(Some(right)) => right,
        val => return val,
    };
    match op_char {
        '+' => Ok(Some(Box::new(Node::Add(left, right)))),
        _ => Err(ParseError {
            message: "Unknown operator.".to_string(),
        }),
    }
}

fn parse_num(tokens: &mut Peekable<impl Iterator<Item = char>>) -> ParseResult {
    let mut buffer = String::new();
    while let Some(&c) = tokens.peek() {
        if is_num_char(&c) {
            tokens.next();
            buffer.push(c);
        } else if c.is_whitespace() {
            tokens.next();
            continue;
        } else {
            break;
        }
    }
    if buffer.is_empty() {
        Err(ParseError {
            message: "TODO error".to_string(),
        })
    } else {
        Ok(Some(Box::new(Node::Lit(buffer.parse().unwrap()))))
    }
}

fn main() {
    let node_1 = Box::new(Node::Lit(1));
    let node_2 = Box::new(Node::Lit(2));
    let node_3 = Box::new(Node::Add(node_1, node_2));

    println!("nodes: {:?}", &node_3);
    println!("eval: {:?}", eval(&node_3));

    // TODO: need to handle for spaces in the string.
    let node_4 = parse("1234 + 1234").unwrap().unwrap();
    println!("parsed eval: {:?}", eval(&node_4));
}

#[test]
fn basic_test() {}
