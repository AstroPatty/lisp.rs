use crate::eval::EvalError;
use crate::parse::ParseError;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Char(char),
    Id(String),
    List((Rc<Value>, Rc<Value>)),
    Function(fn(&[Rc<Value>]) -> Result<Value, EvalError>),
    Nil,
}

impl Value {
    pub(crate) fn car(&self) -> Option<Rc<Value>> {
        match self {
            Value::List((first, _)) => Some(first.clone()),
            _ => None,
        }
    }
    pub(crate) fn cdr(&self) -> Option<Rc<Value>> {
        match self {
            Value::List((_, second)) => Some(second.clone()),
            _ => None,
        }
    }
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            _ => true,
        }
    }
    pub(crate) fn length(&self) -> usize {
        match self {
            Value::List((_, cdr)) => match cdr.as_ref() {
                Value::List(_) => 1 + cdr.length(),
                Value::Nil => 1,
                _ => 2,
            },
            _ => 0,
        }
    }
}

pub(crate) fn parse_atom(input: &str) -> Result<(Value, usize), ParseError> {
    let atom: String = if let Some(index) = input.find(&[' ', ')']) {
        input.chars().take(index).collect()
    } else {
        return Err(ParseError::UnexpectedEOF);
    };
    let len = atom.len();
    if atom == "Nil" {
        return Ok((Value::Nil, len));
    }
    if let Ok(val) = atom.parse::<i64>() {
        return Ok((Value::Int(val), len));
    } else if let Ok(val) = atom.parse::<f64>() {
        return Ok((Value::Float(val), len));
    } else {
        return Ok((Value::Id(atom), len));
    }
}
