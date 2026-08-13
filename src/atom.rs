use crate::env::Env;
use crate::eval::EvalError;
use crate::lambda::Lambda;
use crate::macros::Macro;
use crate::parse::ParseError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
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
    Function(fn(&[Rc<Value>], Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError>),
    Lambda(Lambda),
    Macro(Macro),
    Nil,
}

impl Value {
    pub(crate) fn car(&self) -> Result<Rc<Value>, EvalError> {
        match self {
            Value::List((first, _)) => Ok(first.clone()),
            Value::Nil => Ok(Rc::new(Value::Nil)),
            _ => Err(EvalError::TypeError(String::from("Expected a list"))),
        }
    }
    pub(crate) fn cdr(&self) -> Result<Rc<Value>, EvalError> {
        match self {
            Value::List((_, second)) => Ok(second.clone()),
            Value::Nil => Ok(Rc::new(Value::Nil)),
            _ => Err(EvalError::TypeError(String::from("Expected a list"))),
        }
    }
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Char(c) => write!(f, "#\\{}", c),
            Value::Id(id) => write!(f, "{}", id),
            Value::Macro(_) => write!(f, "Macro"),
            Value::List((car, cdr)) => {
                write!(f, "({}", car)?;
                let mut rest = cdr.clone();
                loop {
                    match rest.as_ref() {
                        Value::Nil => break,
                        Value::List((car, cdr)) => {
                            write!(f, " {}", car)?;
                            rest = cdr.clone();
                        }
                        other => {
                            write!(f, " . {}", other)?;
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
            Value::Function(_) => write!(f, "#<function>"),
            Value::Lambda(_) => write!(f, "#<lambda>"),
            Value::Nil => write!(f, "()"),
        }
    }
}

pub(crate) fn parse_atom(
    input: &str,
    symbol_table: &mut HashMap<String, Rc<Value>>,
) -> Result<(Rc<Value>, usize), ParseError> {
    let atom: String = if let Some(index) = input.find(&[' ', ')', '(']) {
        input.chars().take(index).collect()
    } else {
        return Err(ParseError::UnexpectedEOF);
    };
    let len = atom.len();
    if let Ok(val) = atom.parse::<i64>() {
        return Ok((Rc::new(Value::Int(val)), len));
    } else if let Ok(val) = atom.parse::<f64>() {
        return Ok((Rc::new(Value::Float(val)), len));
    } else {
        if let Some(symbol) = symbol_table.get(&atom) {
            return Ok((symbol.clone(), len));
        }
        let value = Rc::new(Value::Id(atom.clone()));
        symbol_table.insert(String::from(atom.clone()), value.clone());
        return Ok((value, len));
    }
}
