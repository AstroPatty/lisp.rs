use crate::atom::{AtomValue, parse_atom};

#[derive(Debug)]
pub(crate) enum Expression {
    Atom(AtomValue),
    List(Vec<Expression>),
}

pub(crate) fn parse_list(input: &str) -> (Vec<Expression>, usize) {
    let mut slice = input;
    if slice.chars().next() != Some('(') {
        panic!()
    }
    slice = &slice[1..];
    let n_empty = slice.find(|c: char| !c.is_whitespace()).unwrap();
    let mut counter = 1 + n_empty;
    slice = &slice[n_empty..];

    let mut exps = Vec::new();
    loop {
        let mut expr_size = slice.find(|c: char| !c.is_whitespace()).unwrap();
        slice = &slice[expr_size..];
        if slice.chars().next() == Some(')') {
            return (exps, counter + 1);
        } else if slice.chars().next() == Some('(') {
            let (list, list_size) = parse_list(slice);
            exps.push(Expression::List(list));
            expr_size += list_size;
        } else {
            let (atom, atom_size) = parse_atom(slice);
            exps.push(Expression::Atom(atom));
            expr_size += atom_size;
        }
        counter += expr_size;
        slice = &slice[expr_size..];
    }
}
