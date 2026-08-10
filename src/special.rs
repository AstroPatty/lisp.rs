use crate::atom::Value;
use crate::env::Env;
use crate::eval::{EvalError, evaluate};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub(crate) fn get_special_forms()
-> HashMap<String, fn(&Value, Rc<RefCell<Env>>) -> Result<Value, EvalError>> {
    let mut output: HashMap<String, fn(&Value, Rc<RefCell<Env>>) -> Result<Value, EvalError>> =
        HashMap::new();
    output.insert(String::from("quote"), quote);
    output.insert(String::from("defparameter"), defparameter);
    output.insert(String::from("setq"), setq);
    output.insert(String::from("progn"), progn);
    output.insert(String::from("if"), conditional);
    output
}

fn quote(val: &Value, _: Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    match val {
        Value::List((first, cdr)) if matches!(cdr.as_ref(), Value::Nil) => Ok((**first).clone()),
        Value::List(_) => Err(EvalError::Unknown), // more than one arg
        Value::Nil => Err(EvalError::Unknown),     // zero args
        _ => Err(EvalError::Unknown),
    }
}

fn defparameter(val: &Value, env: Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let first = val.car().ok_or(EvalError::Unknown)?;
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
        env.borrow_mut().insert(id, result.as_ref());
        return Ok(result.as_ref().clone());
    };
    Ok(Value::Nil)
}

fn setq(val: &Value, env: Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let first = val.car().ok_or(EvalError::Unknown)?;
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
        return env.borrow_mut().set(id, result.as_ref());
    };
    return Err(EvalError::Unknown);
}

fn progn(val: &Value, env: Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let mut current_list = val;
    while let Value::List((car, cdr)) = current_list {
        let car_value = evaluate(car.clone(), env.clone());
        match cdr.as_ref() {
            Value::Nil => return car_value.map(|val| val.as_ref().clone()),
            _ => current_list = cdr,
        }
    }
    Err(EvalError::Unknown)
}

fn conditional(val: &Value, env: Rc<RefCell<Env>>) -> Result<Value, EvalError> {
    let is_true = if let Value::List((car, _)) = val {
        let cond_value = evaluate(car.clone(), env.clone())?;
        cond_value.is_truthy()
    } else {
        return Err(EvalError::Unknown);
    };
    if is_true {
        Ok(evaluate(val.cdr().unwrap().car().unwrap(), env.clone())?
            .as_ref()
            .clone())
    } else if val.length() < 3 {
        Ok(Value::Nil)
    } else {
        Ok(evaluate(
            val.cdr().unwrap().cdr().unwrap().car().unwrap(),
            env.clone(),
        )?
        .as_ref()
        .clone())
    }
}
