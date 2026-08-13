use crate::atom::Value;
use crate::env::Env;
use crate::eval::{EvalError, evaluate};
use crate::list::build_list;
use crate::special::progn;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Macro {
    params: Vec<String>,
    rest: Option<String>,
    body: Rc<Value>,
    env: Rc<RefCell<Env>>,
}

impl Macro {
    pub(crate) fn run(
        &self,
        pars: Rc<Value>,
        env: Rc<RefCell<Env>>,
    ) -> Result<Rc<Value>, EvalError> {
        if pars.length() < self.params.len() {
            return Err(EvalError::ArgumentCount((self.params.len(), pars.length())));
        } else if pars.length() > self.params.len() && self.rest.is_none() {
            return Err(EvalError::ArgumentCount((self.params.len(), pars.length())));
        }

        let macro_env = Rc::new(RefCell::new(Env::make_child(self.env.clone())));
        let mut curr = pars.clone();
        for par in self.params.iter() {
            if let Value::List((car, cdr)) = curr.as_ref() {
                macro_env.borrow_mut().insert(par, car.clone());
                curr = cdr.clone();
            } else {
                panic!()
            }
        }

        if let Some(rest_name) = &self.rest {
            macro_env.borrow_mut().insert(&rest_name, curr);
        }

        let expanded = evaluate(self.body.clone(), macro_env)?;
        evaluate(expanded, env)
    }

    pub(crate) fn parse(data: Rc<Value>, env: Rc<RefCell<Env>>) -> Result<Self, EvalError> {
        if data.length() != 2 {
            return Err(EvalError::ArgumentCount((2, data.length())));
        }
        let (params, rest) = Macro::parse_arg_list(data.car()?.clone())?;
        return Ok(Macro {
            params,
            rest,
            body: data.cdr()?.car()?.clone(),
            env: env.clone(),
        });
    }

    fn parse_arg_list(args: Rc<Value>) -> Result<(Vec<String>, Option<String>), EvalError> {
        let mut params: Vec<String> = Vec::new();
        if matches!(args.as_ref(), Value::Nil) {
            return Ok((params, None));
        }
        if let Value::List((car, cdr)) = args.as_ref() {
            match car.as_ref() {
                Value::Id(id) if id == "&rest" => {
                    let rest_id = Some(Macro::parse_rest(cdr.clone())?);
                    return Ok((params, rest_id));
                }
                Value::Id(id) => {
                    params.push(id.to_owned());
                    let (tail_params, rest) = Macro::parse_arg_list(cdr.clone())?;
                    params.extend(tail_params);
                    return Ok((params, rest));
                }
                _ => return Err(EvalError::TypeError(format!("Expected a list!"))),
            }
        }
        Err(EvalError::TypeError(format!("Expected a list!")))
    }
    fn parse_rest(args: Rc<Value>) -> Result<String, EvalError> {
        if let Value::List((car, cdr)) = args.as_ref() {
            match (car.as_ref(), cdr.as_ref()) {
                (Value::Id(id), Value::Nil) => return Ok(id.to_owned()),
                _ => {}
            }
        }

        return Err(EvalError::TypeError(format!(
            "&rest should be followed by a single id"
        )));
    }
}
