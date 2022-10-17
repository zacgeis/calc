use std::iter::Peekable;

#[derive(Debug)]
enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Mod(Box<Node>, Box<Node>),
    Exp(Box<Node>, Box<Node>),
    Lit(i32),
}

fn eval(node: Box<Node>) -> i32 {
    match *node {
        Node::Add(a, b) => eval(a) + eval(b),
        Node::Sub(a, b) => eval(a) - eval(b),
        Node::Mul(a, b) => eval(a) * eval(b),
        Node::Div(a, b) => eval(a) / eval(b),
        Node::Mod(a, b) => eval(a) % eval(b),
        Node::Exp(a, b) => i32::pow(eval(a), eval(b) as u32),
        Node::Lit(a) => a,
    }
}

fn parse(s: &str) -> Box<Node> {
    let mut tokens = s.chars().peekable();
    parse_exp(&mut tokens)
}

fn is_lit_char(c: &char) -> bool {
    c.is_numeric() || *c == '-'
}

fn parse_exp(tokens: &mut Peekable<impl Iterator<Item = char>>) -> Box<Node> {
    match tokens.peek() {
        Some(c) => {
            if is_lit_char(c) {
                parse_lit(tokens)
            } else {
                todo!()
            }
        }
        None => panic!("reached end of token stream."),
    }
}

fn parse_lit(tokens: &mut Peekable<impl Iterator<Item = char>>) -> Box<Node> {
    let mut buffer = String::new();
    while let Some(&c) = tokens.peek() {
        if is_lit_char(&c) {
            tokens.next();
            buffer.push(c);
        }
    }
    Box::new(Node::Lit(buffer.parse().unwrap()))
}

fn main() {
    let node_1 = Box::new(Node::Lit(1));
    let node_2 = Box::new(Node::Lit(2));
    let node_3 = Box::new(Node::Add(node_1, node_2));

    println!("nodes: {:?}", &node_3);
    println!("eval: {:?}", eval(node_3));

    let node_4 = parse("1234");
    println!("parsed eval: {:?}", eval(node_4));
}

#[test]
fn basic_test() {
}
