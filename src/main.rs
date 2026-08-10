mod atom;
mod env;
mod eval;
mod lambda;
mod numeric;
mod parse;
mod special;
use env::Env;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

fn main() {
    let env_ = Rc::new(RefCell::new(Env::default()));
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed = parse::parse(&input);
        if let Ok(res) = parsed {
            let result = eval::evaluate(Rc::new(res), env_.clone());
            println!("{:?}", result);
        } else {
            println!("{:?}", parsed.unwrap_err())
        }
    }
}
