use crate::parse::ParseError;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Id(String),
    List((Rc<Value>, Rc<Value>)),
    Function(fn(&[Value]) -> Value),
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
}

pub(crate) fn parse_atom(input: &str) -> Result<(Value, usize), ParseError> {
    let atom: String = if let Some(index) = input.find(&[' ', ')']) {
        input.chars().take(index).collect()
    } else {
        return Err(ParseError::UnexpectedEOF);
    };
    let len = atom.len();
    if let Ok(val) = atom.parse::<i64>() {
        return Ok((Value::Int(val), len));
    } else if let Ok(val) = atom.parse::<f64>() {
        return Ok((Value::Float(val), len));
    } else {
        return Ok((Value::Id(atom), len));
    }
}
