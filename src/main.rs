mod atom;
mod env;
mod eval;
mod lambda;
mod list;
mod macros;
mod numeric;
mod parse;
mod special;
mod util;
use env::Env;
use std::cell::RefCell;
use std::env as stdenv;
use std::fs;
use std::io::{self, Write};
use std::rc::Rc;

fn main() {
    let args: Vec<String> = stdenv::args().collect();

    let env_ = Rc::new(RefCell::new(Env::default()));
    if args.len() == 2 {
        let file_path = &args[1];
        let contents = fs::read_to_string(file_path).unwrap();
        let cleaned = contents.replace("\r\n", "").replace('\n', "");
        let parsed_result = parse::parse_file(&cleaned);
        if let Ok(parsed) = parsed_result {
            let mut val = Rc::new(atom::Value::Nil);
            for list in parsed {
                let evaluated_result = eval::evaluate(list, env_.clone());
                if let Ok(evaled) = evaluated_result {
                    val = evaled;
                } else {
                    println!("{:?}", evaluated_result);
                    return;
                }
            }
            println!("{}", val);
        }
        return;
    }

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed = parse::parse_line(&input);
        if let Ok(res) = parsed {
            let result = eval::evaluate(res, env_.clone());
            match result {
                Ok(output) => println!("{}", output),
                Err(err) => println!("{}", err),
            }
        } else {
            println!("{:?}", parsed.unwrap_err())
        }
    }
}
