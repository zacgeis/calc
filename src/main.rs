mod lex;

use lex::{tokenize, Token};
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
    Neg(Box<Node>),
    Num(i32),
}
type ParseResult = Result<Option<Box<Node>>, String>;

fn eval(node: &Node) -> i32 {
    match node {
        Node::Add(a, b) => eval(a) + eval(b),
        Node::Sub(a, b) => eval(a) - eval(b),
        Node::Mul(a, b) => eval(a) * eval(b),
        Node::Div(a, b) => eval(a) / eval(b),
        Node::Mod(a, b) => eval(a) % eval(b),
        Node::Pow(a, b) => i32::pow(eval(a), eval(b) as u32),
        Node::Neg(a) => -eval(a),
        Node::Num(a) => *a,
    }
}

fn parse(tokens: &[Token]) -> ParseResult {
    let mut token_iter = tokens.iter().peekable();
    parse_exp(&mut token_iter)
}

fn is_num_char(c: &char) -> bool {
    c.is_numeric() || *c == '-'
}

fn parse_exp<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> ParseResult {
    let left = match tokens.next() {
        Some(Token::Num(a)) => Node::Num(*a),
        _ => return Err("todo error.".to_string()),
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
        _ => Err("Unknown operator.".to_string()),
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
