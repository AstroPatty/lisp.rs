use crate::atom::Value;
use crate::env::Env;
use crate::eval::EvalError;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn add(values: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    numeric_binop(values, &|a, b| a + b, &|a, b| a + b)
}

pub(crate) fn multiply(
    values: &[Rc<Value>],
    _env: Rc<RefCell<Env>>,
) -> Result<Rc<Value>, EvalError> {
    numeric_binop(values, &|a, b| a * b, &|a, b| a * b)
}

pub(crate) fn subtract(
    values: &[Rc<Value>],
    _env: Rc<RefCell<Env>>,
) -> Result<Rc<Value>, EvalError> {
    numeric_binop(values, &|a, b| a - b, &|a, b| a - b)
}

pub(crate) fn divide(values: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    numeric_binop(values, &|a, b| a / b, &|a, b| a / b)
}

pub(crate) fn equals(values: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if values.len() != 2 {
        return Err(EvalError::ArgumentCount((2, values.len())));
    }
    Ok(Rc::new(Value::Bool(values[0] == values[1])))
}

pub(crate) fn lt(values: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if values.len() != 2 {
        return Err(EvalError::ArgumentCount((2, values.len())));
    }
    let result = numeric_binop(
        values,
        &|a, b| {
            if a < b {
                return 1;
            }
            0
        },
        &|a, b| {
            if a < b {
                return 1.;
            }
            0.
        },
    )?;
    return Ok(Rc::new(Value::Bool(
        (*result == Value::Int(1)) | (*result == Value::Float(1.0)),
    )));
}

fn numeric_binop(
    args: &[Rc<Value>],
    int_op: &impl Fn(i64, i64) -> i64,
    float_op: &impl Fn(f64, f64) -> f64,
) -> Result<Rc<Value>, EvalError> {
    let mut vals = args.iter();
    let mut acc = match vals.next() {
        Some(first) => first.clone(),
        None => return Ok(Rc::new(Value::Nil)),
    };
    for item in vals {
        acc = do_binop(acc.as_ref(), item.as_ref(), int_op, float_op)?
    }
    Ok(acc)
}
fn do_binop(
    lhs: &Value,
    rhs: &Value,
    int_op: &impl Fn(i64, i64) -> i64,
    float_op: &impl Fn(f64, f64) -> f64,
) -> Result<Rc<Value>, EvalError> {
    return match (lhs, rhs) {
        (lhs, Value::Nil) => Ok(Rc::new(lhs.clone())),
        (Value::Int(lhs_val), Value::Int(rhs_val)) => {
            Ok(Rc::new(Value::Int(int_op(*lhs_val, *rhs_val))))
        }
        (Value::Float(lhs_val), Value::Int(rhs_val)) => {
            Ok(Rc::new(Value::Float(float_op(*lhs_val, *rhs_val as f64))))
        }
        (Value::Int(lhs_val), Value::Float(rhs_val)) => {
            Ok(Rc::new(Value::Float(float_op(*lhs_val as f64, *rhs_val))))
        }

        (Value::Float(lhs_val), Value::Float(rhs_val)) => {
            Ok(Rc::new(Value::Float(float_op(*lhs_val, *rhs_val))))
        }
        _ => Err(EvalError::TypeError(String::from(
            "Expected numeric arguments",
        ))),
    };
}
