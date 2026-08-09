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
    let atoms: Vec<AtomValue> = exprs
        .iter()
        .map(|expr| match expr {
            Expression::List(list_exprs) => evaluate(list_exprs),
            Expression::Atom(atom_val) => atom_val.to_owned(),
        })
        .collect();

    match id {
        "+" => return add(&atoms),
        "*" => return multiply(&atoms),
        "-" => return subtract(&atoms),
        "/" => return divide(&atoms),
        &_ => panic!(),
    }
}
fn numeric_binop(
    lhs: &AtomValue,
    rhs: &AtomValue,
    int_op: impl Fn(i64, i64) -> i64,
    float_op: impl Fn(f64, f64) -> f64,
) -> AtomValue {
    match (lhs, rhs) {
        (AtomValue::Int(acc_int), &AtomValue::Int(val_int)) => {
            AtomValue::Int(int_op(*acc_int, val_int))
        }
        (AtomValue::Int(acc_int), &AtomValue::Float(val_float)) => {
            AtomValue::Float(float_op(*acc_int as f64, val_float))
        }
        (AtomValue::Float(acc_float), &AtomValue::Int(val_float)) => {
            AtomValue::Float(float_op(*acc_float, val_float as f64))
        }
        (AtomValue::Float(acc_float), &AtomValue::Float(val_float)) => {
            AtomValue::Float(float_op(*acc_float, val_float))
        }
        _ => panic!(),
    }
}

fn add(vals: &[AtomValue]) -> AtomValue {
    vals.iter().fold(AtomValue::Int(0), |acc, val| {
        numeric_binop(&acc, val, |a, b| a + b, |a, b| a + b)
    })
}
fn multiply(vals: &[AtomValue]) -> AtomValue {
    vals.iter().fold(AtomValue::Int(1), |acc, val| {
        numeric_binop(&acc, val, |a, b| a * b, |a, b| a * b)
    })
}
fn divide(vals: &[AtomValue]) -> AtomValue {
    vals[1..].iter().fold(vals[0].clone(), |acc, val| {
        numeric_binop(&acc, val, |a, b| a + b, |a, b| a + b)
    })
}
fn subtract(vals: &[AtomValue]) -> AtomValue {
    vals[1..].iter().fold(vals[0].clone(), |acc, val| {
        numeric_binop(&acc, val, |a, b| a + b, |a, b| a + b)
    })
}
