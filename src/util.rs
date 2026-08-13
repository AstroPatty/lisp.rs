use crate::atom::Value;
use crate::eval::EvalError;
use std::rc::Rc;
macro_rules! builtins {
    ($(
            $vis:vis fn $name:ident($env:ident : Rc<RefCell<Env>> $(, $arg:ident : Rc<Value>)* $(,)?) -> Result<Rc<Value>, EvalError> $body:block
    )*) => {
        $(
            $vis fn $name(args: &[Rc<Value>], _env: Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> {
                fn inner($($arg: Rc<Value>),* , _env:Rc<RefCell<Env>>) -> Result<Rc<Value>, EvalError> $body
                let [$($arg),*] = args else {
                    return Err(EvalError::ArgumentCount(([$(stringify!($arg)),*].len(), args.len())))
                };
                inner($($arg.clone()),*, _env)
            }
        )*
    };
}

pub(crate) use builtins;
