use crate::atom::Value;
use crate::env::Env;
use crate::eval::{EvalError, evaluate};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) fn default() -> HashMap<String, Rc<Value>> {
    let mut values = HashMap::new();
    values.insert(String::from("cons"), Rc::new(Value::Function(cons)));
    values.insert(String::from("list"), Rc::new(Value::Function(list)));
    values.insert(String::from("car"), Rc::new(Value::Function(car)));
    values.insert(String::from("cdr"), Rc::new(Value::Function(cdr)));
    values.insert(String::from("null"), Rc::new(Value::Function(null)));
    values.insert(String::from("atom"), Rc::new(Value::Function(atom)));
    values.insert(String::from("consp"), Rc::new(Value::Function(consp)));
    values.insert(String::from("symbolp"), Rc::new(Value::Function(symbolp)));
    values.insert(String::from("cadr"), Rc::new(Value::Function(cadr)));
    values.insert(String::from("caddr"), Rc::new(Value::Function(caddr)));
    values.insert(String::from("reverse"), Rc::new(Value::Function(reverse)));
    values.insert(String::from("length"), Rc::new(Value::Function(length)));
    values.insert(String::from("eq"), Rc::new(Value::Function(eq)));
    values.insert(String::from("eql"), Rc::new(Value::Function(eql)));
    values.insert(String::from("append"), Rc::new(Value::Function(append)));
    values
}

fn cons(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 2 {
        return Err(EvalError::ArgumentCount((2, vals.len())));
    };
    Ok(Rc::new(Value::List((vals[0].clone(), vals[1].clone()))))
}

fn null(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((2, vals.len())));
    };
    match vals[0].as_ref() {
        Value::Nil => Ok(Rc::new(Value::Bool(true))),
        _ => Ok(Rc::new(Value::Nil)),
    }
}

fn atom(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    match vals[0].as_ref() {
        Value::List(_) => Ok(Rc::new(Value::Bool(false))),
        _ => Ok(Rc::new(Value::Bool(true))),
    }
}

fn consp(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    match vals[0].as_ref() {
        Value::List(_) => Ok(Rc::new(Value::Bool(true))),
        _ => Ok(Rc::new(Value::Bool(false))),
    }
}
fn symbolp(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    match vals[0].as_ref() {
        Value::Id(_) => Ok(Rc::new(Value::Bool(true))),
        _ => Ok(Rc::new(Value::Bool(false))),
    }
}

fn list(vals: &[Rc<Value>], env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() == 0 {
        return Ok(Rc::new(Value::Nil));
    }
    if vals.len() == 1 {
        return Ok(Rc::new(Value::List((vals[0].clone(), Rc::new(Value::Nil)))));
    }
    let rhs = list(&vals[1..], env.clone())?;
    Ok(Rc::new(Value::List((vals[0].clone(), rhs))))
}

fn car(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    vals[0].car()
}

fn cdr(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    vals[0].cdr()
}

fn cadr(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    match &vals[0].as_ref() {
        Value::List((_, cdr)) => cdr.as_ref().car(),
        _ => Err(EvalError::TypeError(String::from("Expected a list"))),
    }
}

fn caddr(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    match &vals[0].as_ref() {
        Value::List((_, cdr)) => cdr.as_ref().cdr()?.as_ref().car(),
        _ => Err(EvalError::TypeError(String::from("Expected a list"))),
    }
}

fn reverse(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    match &vals[0].as_ref() {
        Value::List(_) => _reverse(vals[0].clone()),
        _ => Err(EvalError::TypeError(String::from("Expected a list"))),
    }
}

fn _reverse(val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    match val.as_ref() {
        Value::Nil => Ok(val.clone()),
        Value::List((car, cdr)) => match cdr.as_ref() {
            Value::Nil => Ok(val.clone()),
            Value::List(_) => _append(_reverse(cdr.clone())?, car.clone()),
            _ => Err(EvalError::Unknown),
        },
        _ => Err(EvalError::TypeError(String::from("Expected a list"))),
    }
}
fn length(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 1 {
        return Err(EvalError::ArgumentCount((1, vals.len())));
    };
    Ok(Rc::new(Value::Int(vals[0].as_ref().length() as i64)))
}

fn eq(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 2 {
        return Err(EvalError::ArgumentCount((2, vals.len())));
    };
    Ok(Rc::new(Value::Bool(Rc::ptr_eq(&vals[0], &vals[1]))))
}

fn eql(vals: &[Rc<Value>], env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    if vals.len() != 2 {
        return Err(EvalError::ArgumentCount((2, vals.len())));
    };
    match (vals[0].as_ref(), vals[1].as_ref()) {
        (Value::Int(l), Value::Int(r)) => Ok(Rc::new(Value::Bool(l == r))),
        (Value::Float(l), Value::Float(r)) => Ok(Rc::new(Value::Bool(l == r))),
        (Value::Char(l), Value::Char(r)) => Ok(Rc::new(Value::Bool(l == r))),
        _ => eq(&vals, env),
    }
}

fn append(vals: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let n_vals = vals.len();
    if n_vals == 0 {
        return Ok(Rc::new(Value::Nil));
    } else if n_vals == 1 {
        return Ok(vals[0].clone());
    }
    let mut return_value = _duplicate(vals[0].clone())?;
    for i in 1..n_vals - 1 {
        return_value = _append(return_value, _duplicate(vals[i].clone())?)?;
    }
    Ok(_append(return_value, vals[n_vals - 1].clone())?)
}

pub(crate) fn build_list(vals: &[Rc<Value>]) -> Rc<Value> {
    if vals.len() == 0 {
        Rc::new(Value::Nil)
    } else {
        let list = Value::List((vals[0].clone(), build_list(&vals[1..])));
        Rc::new(list)
    }
}

pub(crate) fn _append(lhs: Rc<Value>, rhs: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    if matches!(lhs.as_ref(), Value::Nil) {
        return _duplicate(rhs);
    } else if matches!(rhs.as_ref(), Value::Nil) {
        return Ok(lhs.clone());
    }

    if let Value::List((car, cdr)) = lhs.as_ref() {
        match cdr.as_ref() {
            Value::Nil => {
                let new_cdr = match rhs.as_ref() {
                    Value::List(_) => rhs.clone(),
                    _ => Rc::new(Value::List((rhs.clone(), cdr.clone()))),
                };
                let new_list = Value::List((car.clone(), new_cdr));
                return Ok(Rc::new(new_list));
            }
            Value::List(_) => {
                let new_cdr = _append(cdr.clone(), rhs.clone())?;
                return Ok(Rc::new(Value::List((car.clone(), new_cdr))));
            }
            _ => return Err(EvalError::TypeError(format!("Expected a list!"))),
        }
    }
    return Err(EvalError::TypeError(format!("Expected a list!")));
}

fn _duplicate(val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    if matches!(val.as_ref(), Value::Nil) {
        return Ok(val);
    }
    if let Value::List((car, cdr)) = val.as_ref() {
        let cons = match cdr.as_ref() {
            Value::Nil => Value::List((car.clone(), cdr.clone())),
            Value::List(_) => Value::List((car.clone(), _duplicate(cdr.clone())?)),
            _ => return Err(EvalError::TypeError(format!("Expected a list"))),
        };
        return Ok(Rc::new(cons));
    }
    return Err(EvalError::TypeError(format!("Expected a list")));
}
