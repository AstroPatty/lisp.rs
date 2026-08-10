use crate::atom::Value;
use crate::eval::EvalError;
use crate::numeric::{add, divide, equals, lt, multiply, subtract};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct Env {
    values: HashMap<String, Rc<Value>>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub(crate) fn default() -> Self {
        let mut values = HashMap::new();
        values.insert(String::from("+"), Rc::new(Value::Function(add)));
        values.insert(String::from("*"), Rc::new(Value::Function(multiply)));
        values.insert(String::from("/"), Rc::new(Value::Function(divide)));
        values.insert(String::from("-"), Rc::new(Value::Function(subtract)));
        values.insert(String::from("="), Rc::new(Value::Function(equals)));
        values.insert(String::from("<"), Rc::new(Value::Function(lt)));
        let parent = None;
        Env { values, parent }
    }
    pub(crate) fn lookup(&self, token: &str) -> Option<Rc<Value>> {
        if let Some(local_value) = self.values.get(token) {
            return Some(local_value.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().lookup(token);
        }
        None
    }
    pub(crate) fn insert(&mut self, token: &str, value: &Value) {
        self.values
            .insert(String::from(token), Rc::new(value.clone()));
    }
    pub(crate) fn set(&mut self, token: &str, value: &Value) -> Result<Value, EvalError> {
        if self.values.contains_key(token) {
            self.values
                .insert(String::from(token), Rc::new(value.clone()));
            return Ok(Value::Nil);
        } else if let Some(parent) = &self.parent {
            return parent.borrow_mut().set(token, value);
        }
        return Err(EvalError::Unknown);
    }
}
