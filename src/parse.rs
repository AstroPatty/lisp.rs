use crate::atom::{Value, parse_atom};
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

pub fn parse(input: &str) -> Result<Value, ParseError> {
    let trimmed = input.trim_start();
    if !trimmed.starts_with('(') {
        // handle bare atom / error, as appropriate
        return Err(ParseError::UnclosedParen);
    }
    let (value, _) = parse_list(&trimmed[1..])?;
    Ok(value)
}

pub(crate) fn parse_list(input: &str) -> Result<(Value, usize), ParseError> {
    let (first_value, first_size) = parse_one(&input)?;
    if let Some(fv) = first_value {
        let (next_value, next_size) = parse_list(&input[first_size..])?;
        return Ok((
            Value::List((Rc::new(fv), Rc::new(next_value))),
            first_size + next_size,
        ));
    }
    Ok((Value::Nil, first_size))
}

fn parse_one(input: &str) -> Result<(Option<Value>, usize), ParseError> {
    let n_blank = input
        .find(|c: char| !c.is_whitespace())
        .ok_or(ParseError::UnclosedParen)?;
    let first = input.chars().nth(n_blank).unwrap();
    if first == ')' {
        return Ok((None, n_blank + 1));
    } else if first == '(' {
        let (val, size) = parse_list(&input[n_blank + 1..])?;
        return Ok((Some(val), size + n_blank + 1));
    }

    let (val, n_consumed) = parse_atom(&input[n_blank..])?;
    return Ok((Some(val), n_consumed + n_blank));
}
