use crate::atom::Value;
use crate::eval::EvalError;
use crate::eval::evaluate;
use crate::list::default;
use crate::numeric::{add, divide, equals, lt, multiply, subtract};
use crate::parse::parse_file;
use crate::special::load;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(crate) struct Env {
    values: HashMap<String, Rc<Value>>,
    parent: Option<Rc<RefCell<Env>>>,
    cwd: PathBuf,
}

impl Env {
    pub(crate) fn default() -> Rc<RefCell<Self>> {
        let mut values = default();
        values.insert(String::from("+"), Rc::new(Value::Function(add)));
        values.insert(String::from("*"), Rc::new(Value::Function(multiply)));
        values.insert(String::from("/"), Rc::new(Value::Function(divide)));
        values.insert(String::from("-"), Rc::new(Value::Function(subtract)));
        values.insert(String::from("="), Rc::new(Value::Function(equals)));
        values.insert(String::from("<"), Rc::new(Value::Function(lt)));
        let parent = None;
        let cwd = current_dir().unwrap();

        let env = Rc::new(RefCell::new(Env {
            values,
            parent,
            cwd,
        }));

        return Env::load_prelude(env);
    }
    fn load_prelude(env: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let prelude_path = Rc::new(Value::Str(format!("lib/prelude.lisp")));
        let prelude_args = Value::List((prelude_path, Rc::new(Value::Nil)));

        _ = load(Rc::new(prelude_args), env.clone());
        return env;
    }

    pub(crate) fn make_child(env: Rc<RefCell<Env>>) -> Self {
        Env {
            values: HashMap::new(),
            parent: Some(env.clone()),
            cwd: env.borrow_mut().get_cwd().to_path_buf(),
        }
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
    pub(crate) fn insert(&mut self, token: &str, value: Rc<Value>) {
        self.values.insert(String::from(token), value);
    }
    pub(crate) fn defpar(&mut self, token: &str, value: Rc<Value>) {
        if let Some(parent) = &self.parent {
            parent.borrow_mut().defpar(token, value)
        } else {
            self.insert(token, value)
        }
    }
    pub(crate) fn set(&mut self, token: &str, value: Rc<Value>) -> Result<Rc<Value>, EvalError> {
        if self.values.contains_key(token) {
            self.values.insert(String::from(token), value.clone());
            return Ok(value.clone());
        } else if let Some(parent) = &self.parent {
            return parent.borrow_mut().set(token, value);
        }
        return Err(EvalError::UnknownVariable(String::from(token)));
    }

    pub(crate) fn get_cwd(&self) -> &Path {
        return self.cwd.as_path();
    }
    pub(crate) fn set_cwd(&mut self, path: PathBuf) {
        self.cwd = path;
    }
}
