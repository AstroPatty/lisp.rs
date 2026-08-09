mod atom;
mod eval;
mod parse;

fn main() {
    let input = String::from("(* 7 ( + 3 4 5))");
    let (parsed, _) = parse::parse_list(&input);
    let res = eval::evaluate(&parsed);
    println!("{:?}", res);
}
