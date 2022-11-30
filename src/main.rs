use calc::eval::eval;
use calc::parse::parse;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let node = parse(&line).unwrap().unwrap();
                println!("tree: {:?}, eval: {}", &node, eval(&node));
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
