mod atom;
mod eval;
mod parse;
use std::io;

fn main() {
    let mut input = String::from("(* 7 ( + 3 4))");
    let (parsed, n_chars) = parse::parse_list(&input);
    let res = eval::evaluate(&parsed);
    println!("{:?}", res);
}
