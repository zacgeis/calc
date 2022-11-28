use calc::eval::eval;
use calc::parse::parse;

fn main() {
    let node = parse("1 - 2 - 3").unwrap().unwrap();
    println!("tree: {:?}", &node);
    println!("results: {}", eval(&node));
    // println!("tree: {:?}, eval: {:?}", &node, eval(&node));
}
