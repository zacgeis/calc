use crate::parse::{parse, Node};

pub fn eval_str(input: &str) -> i64 {
    let root_node = parse(input).unwrap().unwrap();
    eval(&root_node)
}

// TODO: Update eval to return a result and handle things like divide by zero?
pub fn eval(node: &Node) -> i64 {
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
