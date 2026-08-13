use crate::atom::{Value, parse_atom};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
    UnclosedParen,
    UnexpectedEOF,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnclosedParen => write!(f, "Item was not found."),
            ParseError::UnexpectedEOF => write!(f, "Item was not found."),
        }
    }
}

// 3. Implement the Error trait
impl Error for ParseError {}
pub fn parse_file(input: &str) -> Result<Vec<Rc<Value>>, ParseError> {
    let mut symbol_table: HashMap<String, Rc<Value>> = HashMap::new();
    symbol_table.insert(String::from("T"), Rc::new(Value::Bool(true)));
    let total_len = input.len();
    let mut offset = 0;
    let mut stmts: Vec<Rc<Value>> = Vec::new();
    let mut current = input.trim();
    loop {
        if offset == total_len {
            return Ok(stmts);
        }
        if !current.starts_with("(") {
            return Err(ParseError::UnclosedParen);
        }
        let (value, n_taken) = parse_list(&current[1..], &mut symbol_table)?;
        stmts.push(value);
        current = &current[1 + n_taken..];
        offset += 1 + n_taken;
    }
}

pub fn parse_line(input: &str) -> Result<Rc<Value>, ParseError> {
    let mut symbol_table: HashMap<String, Rc<Value>> = HashMap::new();
    symbol_table.insert(String::from("T"), Rc::new(Value::Bool(true)));
    symbol_table.insert(String::from("nil"), Rc::new(Value::Nil));
    let trimmed = input.trim_start();
    if trimmed.starts_with('\'') | trimmed.starts_with('`') {
        return parse_one(&trimmed, &mut symbol_table).map(|val| val.0.unwrap());
    }
    if !trimmed.starts_with('(') {
        // handle bare atom / error, as appropriate
        return Err(ParseError::UnclosedParen);
    }
    let (value, _) = parse_list(&trimmed[1..], &mut symbol_table)?;
    Ok(value)
}

pub(crate) fn parse_list(
    input: &str,
    symbol_table: &mut HashMap<String, Rc<Value>>,
) -> Result<(Rc<Value>, usize), ParseError> {
    let (first_value, first_size) = parse_one(&input, symbol_table)?;
    if let Some(fv) = first_value {
        let (next_value, next_size) = parse_list(&input[first_size..], symbol_table)?;
        return Ok((
            Rc::new(Value::List((fv, next_value))),
            first_size + next_size,
        ));
    }
    Ok((Rc::new(Value::Nil), first_size))
}

fn parse_one(
    input: &str,
    symbol_table: &mut HashMap<String, Rc<Value>>,
) -> Result<(Option<Rc<Value>>, usize), ParseError> {
    let n_blank = input
        .find(|c: char| !c.is_whitespace())
        .ok_or(ParseError::UnclosedParen)?;
    let first = input.chars().nth(n_blank).unwrap();
    if first == ')' {
        return Ok((None, n_blank + 1));
    } else if first == '(' {
        let (val, size) = parse_list(&input[n_blank + 1..], symbol_table)?;
        return Ok((Some(val), size + n_blank + 1));
    } else if first == '\'' {
        let (new_value, new_size) = expand(
            &input[n_blank + 1..],
            Rc::new(Value::Id(format!("quote"))),
            symbol_table,
        )?;

        return Ok((Some(new_value), n_blank + new_size + 1));
    } else if first == '`' {
        let (new_value, new_size) = expand(
            &input[n_blank + 1..],
            Rc::new(Value::Id(format!("quasiquote"))),
            symbol_table,
        )?;

        return Ok((Some(new_value), n_blank + new_size + 1));
    } else if first == ',' && input.chars().nth(n_blank + 1).unwrap_or(' ') == '@' {
        let (new_value, new_size) = expand(
            &input[n_blank + 2..],
            Rc::new(Value::Id(format!("unquote-splicing"))),
            symbol_table,
        )?;

        return Ok((Some(new_value), n_blank + new_size + 2));
    } else if first == ',' {
        let (new_value, new_size) = expand(
            &input[n_blank + 1..],
            Rc::new(Value::Id(format!("unquote"))),
            symbol_table,
        )?;

        return Ok((Some(new_value), n_blank + new_size + 1));
    }

    let (val, n_consumed) = parse_atom(&input[n_blank..], symbol_table)?;
    return Ok((Some(val), n_consumed + n_blank));
}

fn expand(
    input: &str,
    symbol: Rc<Value>,
    symbol_table: &mut HashMap<String, Rc<Value>>,
) -> Result<(Rc<Value>, usize), ParseError> {
    let (inner_value, inner_size) = parse_one(&input, symbol_table)?;
    let inner_list = Value::List((
        inner_value.ok_or(ParseError::UnclosedParen)?,
        Rc::new(Value::Nil),
    ));
    let return_value = Value::List((symbol, Rc::new(inner_list)));
    return Ok((Rc::new(return_value), inner_size));
}
