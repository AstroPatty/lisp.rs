use crate::atom::Value;
use crate::env::Env;
use crate::eval::EvalError;
use crate::special::progn;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Lambda {
    params: Vec<String>,
    body: Rc<Value>,
    env: Rc<RefCell<Env>>,
}

impl Lambda {
    fn new(params: Vec<String>, body: Rc<Value>, env: Rc<RefCell<Env>>) -> Self {
        Lambda { params, body, env }
    }
    pub(crate) fn apply(&self, pars: Vec<Rc<Value>>) -> Result<Rc<Value>, EvalError> {
        if pars.len() != self.params.len() {
            return Err(EvalError::ArgumentCount((self.params.len(), pars.len())));
        }
        let mut new_env = Env::make_child(self.env.clone());
        for (name, par) in zip(self.params.iter(), pars.iter()) {
            new_env.insert(name, par.clone())
        }
        progn(self.body.clone(), Rc::new(RefCell::new(new_env)))
    }
}

fn as_cons(value: &Value) -> Result<(Rc<Value>, Rc<Value>), EvalError> {
    match value {
        Value::List((left, right)) => Ok((left.clone(), right.clone())),
        _ => Err(EvalError::Unknown),
    }
}

pub(crate) fn lambda(value: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
    let (params_expr, body) = as_cons(value.as_ref())?;
    match body.as_ref() {
        Value::Nil => return Err(EvalError::Unknown),
        _ => {}
    }
    let parameters = collect_names(params_expr.as_ref())?;
    Ok(Rc::new(Value::Lambda(Lambda::new(
        parameters,
        body,
        env.clone(),
    ))))
}

fn collect_names(value: &Value) -> Result<Vec<String>, EvalError> {
    let mut names = Vec::new();
    if matches!(value, Value::Nil) {
        return Ok(names);
    }
    let (mut name, mut rest) = as_cons(value)?;
    loop {
        match name.as_ref() {
            Value::Id(id) => names.push(id.clone()),
            _ => {
                return Err(EvalError::TypeError(String::from(
                    "Lambda arguments must be IDs",
                )));
            }
        }
        (name, rest) = match rest.as_ref() {
            Value::Nil => return Ok(names),
            _ => as_cons(rest.as_ref())?,
        }
    }
}
