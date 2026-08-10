use crate::atom::Value;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum EvalError {
    Unknown,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::Unknown => write!(f, "Unknown error"),
        }
    }
}

// 3. Implement the Error trait
impl Error for EvalError {}

pub(crate) fn evaluate(value: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    match value.as_ref() {
        Value::List((head, rest)) => {
            let op = match head.as_ref() {
                Value::Id(name) => name.clone(),
                _ => return Err(EvalError::Unknown),
            };
            let args = eval_args(rest.clone())?;
            evaluate_fn(&op, args)
        }
        _ => Ok(value),
    }
}

fn eval_args(mut list: Rc<Value>) -> Result<Vec<Rc<Value>>, EvalError> {
    let mut out = Vec::new();
    while let Value::List((car, cdr)) = list.as_ref() {
        out.push(evaluate(car.clone())?);
        list = cdr.clone()
    }
    Ok(out)
}

fn evaluate_fn(id: &str, args: Vec<Rc<Value>>) -> Result<Rc<Value>, EvalError> {
    match id {
        "+" => {
            return Ok(Rc::new(numeric_binop(&args, &|a, b| a + b, &|a, b| a + b)));
        }
        "*" => {
            return Ok(Rc::new(numeric_binop(&args, &|a, b| a * b, &|a, b| a * b)));
        }
        &_ => panic!(),
    }
}
fn numeric_binop(
    args: &[Rc<Value>],
    int_op: &impl Fn(i64, i64) -> i64,
    float_op: &impl Fn(f64, f64) -> f64,
) -> Value {
    if let Some(next_item) = args.iter().next() {
        let rhs = numeric_binop(&args[1..], int_op, float_op);
        return match (next_item.as_ref(), rhs) {
            (lhs, Value::Nil) => lhs.clone(),
            (Value::Int(lhs_val), Value::Int(rhs_val)) => Value::Int(int_op(*lhs_val, rhs_val)),
            (Value::Float(lhs_val), Value::Int(rhs_val)) => {
                Value::Float(float_op(*lhs_val, rhs_val as f64))
            }
            (Value::Int(lhs_val), Value::Float(rhs_val)) => {
                Value::Float(float_op(*lhs_val as f64, rhs_val))
            }

            (Value::Float(lhs_val), Value::Float(rhs_val)) => {
                Value::Float(float_op(*lhs_val, rhs_val))
            }
            _ => panic!(),
        };
    }
    Value::Nil
}
