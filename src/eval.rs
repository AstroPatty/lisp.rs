use crate::atom::AtomValue;
use crate::parse::Expression;

pub(crate) fn evaluate(exprs: &[Expression]) -> AtomValue {
    let first = exprs.iter().next().unwrap();
    if let Expression::Atom(av) = first {
        if let AtomValue::Id(id) = av {
            return evaluate_fn(id, &exprs[1..]);
        }
        panic!();
    } else {
        panic!();
    }
}

fn evaluate_fn(id: &str, exprs: &[Expression]) -> AtomValue {
    match id {
        "+" => return add(exprs),
        "*" => return multiply(exprs),
        &_ => panic!(),
    }
}

fn add(exprs: &[Expression]) -> AtomValue {
    let mut value: f64 = 0.;
    let mut is_int: bool = true;
    for expr in exprs {
        let rhs = match expr {
            Expression::Atom(av) => av.to_owned(),
            Expression::List(list_exprs) => evaluate(list_exprs),
        };
        match rhs {
            AtomValue::Int(int) => value += int as f64,
            AtomValue::Float(float) => {
                is_int = false;
                value += float
            }
            _ => panic!(),
        }
    }
    if is_int {
        return AtomValue::Int(value as i64);
    }
    return AtomValue::Float(value);
}

fn multiply(exprs: &[Expression]) -> AtomValue {
    let mut value: f64 = 1.;
    let mut is_int: bool = true;
    for expr in exprs {
        let rhs = match expr {
            Expression::Atom(av) => av.to_owned(),
            Expression::List(list_exprs) => evaluate(list_exprs),
        };
        match rhs {
            AtomValue::Int(int) => value *= int as f64,
            AtomValue::Float(float) => {
                is_int = false;
                value *= float
            }
            _ => panic!(),
        }
    }
    if is_int {
        return AtomValue::Int(value as i64);
    }
    return AtomValue::Float(value);
}
