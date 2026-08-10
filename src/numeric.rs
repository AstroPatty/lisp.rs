use crate::atom::Value;
use crate::eval::EvalError;
use std::rc::Rc;

pub(crate) fn add(values: &[Rc<Value>]) -> Result<Value, EvalError> {
    Ok(numeric_binop(values, &|a, b| a + b, &|a, b| a + b))
}

pub(crate) fn multiply(values: &[Rc<Value>]) -> Result<Value, EvalError> {
    Ok(numeric_binop(values, &|a, b| a * b, &|a, b| a * b))
}

pub(crate) fn subtract(values: &[Rc<Value>]) -> Result<Value, EvalError> {
    Ok(numeric_binop(values, &|a, b| a - b, &|a, b| a - b))
}

pub(crate) fn divide(values: &[Rc<Value>]) -> Result<Value, EvalError> {
    Ok(numeric_binop(values, &|a, b| a / b, &|a, b| a / b))
}

pub(crate) fn equals(values: &[Rc<Value>]) -> Result<Value, EvalError> {
    if values.len() != 2 {
        return Err(EvalError::Unknown);
    }
    Ok(Value::Bool(values[0] == values[1]))
}

pub(crate) fn lt(values: &[Rc<Value>]) -> Result<Value, EvalError> {
    if values.len() != 2 {
        return Err(EvalError::Unknown);
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
    );
    return Ok(Value::Bool(
        (result == Value::Int(1)) | (result == Value::Float(1.0)),
    ));
}

fn numeric_binop(
    args: &[Rc<Value>],
    int_op: &impl Fn(i64, i64) -> i64,
    float_op: &impl Fn(f64, f64) -> f64,
) -> Value {
    let mut vals = args.iter();
    let mut acc = match vals.next() {
        Some(first) => first.as_ref().clone(),
        None => return Value::Nil,
    };
    for item in vals {
        acc = do_binop(&acc, item.as_ref(), int_op, float_op)
    }
    acc
}
fn do_binop(
    lhs: &Value,
    rhs: &Value,
    int_op: &impl Fn(i64, i64) -> i64,
    float_op: &impl Fn(f64, f64) -> f64,
) -> Value {
    return match (lhs, rhs) {
        (lhs, Value::Nil) => lhs.clone(),
        (Value::Int(lhs_val), Value::Int(rhs_val)) => Value::Int(int_op(*lhs_val, *rhs_val)),
        (Value::Float(lhs_val), Value::Int(rhs_val)) => {
            Value::Float(float_op(*lhs_val, *rhs_val as f64))
        }
        (Value::Int(lhs_val), Value::Float(rhs_val)) => {
            Value::Float(float_op(*lhs_val as f64, *rhs_val))
        }

        (Value::Float(lhs_val), Value::Float(rhs_val)) => {
            Value::Float(float_op(*lhs_val, *rhs_val))
        }
        _ => {
            println!("{:?}, {:?}", lhs, rhs);
            panic!();
        }
    };
}
