use crate::atom::Value;
use crate::env::Env;
use crate::special::get_special_forms;
use std::cell::RefCell;
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

pub(crate) fn evaluate(value: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    match value.as_ref() {
        Value::List((head, rest)) => {
            let op = match head.as_ref() {
                Value::Id(name) => {
                    if let Some(form) = get_special_forms().get(name) {
                        return form(rest, env.clone()).map(Rc::new);
                    }
                    env.borrow().lookup(name).ok_or(EvalError::Unknown)?
                }
                _ => evaluate(head.clone(), env.clone())?,
            };
            let args = eval_args(rest.clone(), env.clone())?;
            evaluate_fn(&op, args)
        }
        Value::Id(id) => env.borrow().lookup(id).map_or(Ok(value), Ok),

        _ => Ok(value),
    }
}

fn eval_args(mut list: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Vec<Rc<Value>>, EvalError> {
    let mut out = Vec::new();
    while let Value::List((car, cdr)) = list.as_ref() {
        out.push(evaluate(car.clone(), env.clone())?);
        list = cdr.clone()
    }
    Ok(out)
}

fn evaluate_fn(callable: &Value, args: Vec<Rc<Value>>) -> Result<Rc<Value>, EvalError> {
    match callable {
        Value::Function(func) => Ok(Rc::new(func(&args)?)),
        Value::Lambda(func) => Ok(func.apply(args)?),
        _ => Err(EvalError::Unknown),
    }
}
