mod atom;
mod eval;
mod parse;

fn main() {
    let input = String::from("(+ 3 5 (* 4 5 7))");
    let parsed = parse::parse_list(&input);
    if let Ok((res, _)) = parsed {
        let result = eval::evaluate(res.car().unwrap());
        println!("{:?}", result);
    } else {
        println!("{:?}", parsed.unwrap_err())
    }
}
