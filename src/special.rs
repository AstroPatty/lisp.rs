use crate::atom::Value;
use crate::env::Env;
use crate::eval::{EvalError, evaluate};
use crate::lambda::lambda;
use crate::list::_append;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub(crate) fn get_special_forms()
-> HashMap<String, fn(Rc<Value>, Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError>> {
    let mut output: HashMap<
        String,
        fn(Rc<Value>, Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError>,
    > = HashMap::new();
    output.insert(String::from("quote"), quote);
    output.insert(String::from("defparameter"), defparameter);
    output.insert(String::from("setq"), setq);
    output.insert(String::from("progn"), progn);
    output.insert(String::from("if"), conditional);
    output.insert(String::from("lambda"), lambda);
    output.insert(String::from("quasiquote"), quasiquote);
    output
}

fn quote(val: Rc<Value>, _: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    match val.as_ref() {
        Value::List((first, cdr)) if matches!(cdr.as_ref(), Value::Nil) => Ok(first.clone()),
        Value::List(_) => Err(EvalError::ArgumentCount((1, val.length()))), // more than one arg
        _ => Err(EvalError::TypeError(String::from("Expected a list"))),    // zero args
    }
}

fn defparameter(val: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let first = val.car()?;
    if let Value::Id(id) = first.as_ref() {
        let value = val.cdr().unwrap();
        let unwrapped_val = match value.as_ref() {
            Value::List((val, rhs)) => match rhs.as_ref() {
                Value::Nil => val.clone(),
                _ => return Err(EvalError::Unknown),
            },
            _ => value,
        };
        let result = evaluate(unwrapped_val, env.clone())?;
        env.borrow_mut().insert(id, result.clone());
        return Ok(result);
    };
    Ok(Rc::new(Value::Nil))
}

fn setq(val: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let first = val.car()?;
    if let Value::Id(id) = first.as_ref() {
        let value = val.cdr().unwrap();
        let unwrapped_val = match value.as_ref() {
            Value::List((val, rhs)) => match rhs.as_ref() {
                Value::Nil => val.clone(),
                _ => return Err(EvalError::Unknown),
            },
            _ => value,
        };
        let result = evaluate(unwrapped_val, env.clone())?;
        return env.borrow_mut().set(id, result);
    };
    return Err(EvalError::Unknown);
}

pub(crate) fn progn(val: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let mut current_list = val;
    loop {
        match current_list.as_ref() {
            Value::List((car, cdr)) => {
                let car_value = evaluate(car.clone(), env.clone());
                match cdr.as_ref() {
                    Value::Nil => return car_value,
                    _ => current_list = cdr.clone(),
                }
            }
            _ => return Err(EvalError::Unknown),
        }
    }
}

fn conditional(val: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let is_true = if let Value::List((car, _)) = val.as_ref() {
        let cond_value = evaluate(car.clone(), env.clone())?;
        cond_value.is_truthy()
    } else {
        return Err(EvalError::Unknown);
    };
    if is_true {
        evaluate(val.cdr()?.car()?, env.clone())
    } else if val.length() < 3 {
        Ok(Rc::new(Value::Nil))
    } else {
        evaluate(val.cdr()?.cdr()?.car()?, env.clone())
    }
}

fn quasiquote(val: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let arg = match val.as_ref() {
        Value::List((first, cdr)) if matches!(cdr.as_ref(), Value::Nil) => first.clone(),
        Value::List(_) => return Err(EvalError::ArgumentCount((1, val.length()))), // more than one arg
        _ => return Err(EvalError::TypeError(String::from("Expected a list"))),    // zero args
    };
    match arg.as_ref() {
        Value::List(_) => _quasiquote(arg.clone(), env),
        _ => Ok(arg),
    }
}

fn _quasiquote(val: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if let Value::List((car, cdr)) = val.as_ref() {
        if let Some(x) = match_unary(val.as_ref(), "unquote") {
            return evaluate(x, env);
        }
        if let Some(x) = match_unary(car.as_ref(), "unquote-splicing") {
            let spliced = evaluate(x, env.clone())?;
            let rest = _quasiquote(cdr.clone(), env)?;
            return _append(spliced, rest);
        }
        let new_car = _quasiquote(car.clone(), env.clone())?;
        let new_cdr = _quasiquote(cdr.clone(), env.clone())?;
        let new_list = Value::List((new_car, new_cdr));
        return Ok(Rc::new(new_list));
    }
    Ok(val)
}

fn match_unary(list: &Value, symbol: &str) -> Option<Rc<Value>> {
    // Check if this is a two-item list, with the first being the given symbol.
    if let Value::List((car, cdr)) = list {
        if let Value::Id(id) = car.as_ref() {
            if id == symbol
                && let Value::List((x, rest)) = cdr.as_ref()
            {
                if matches!(rest.as_ref(), Value::Nil) {
                    return Some(x.clone());
                }
            }
        }
    }
    return None;
}
