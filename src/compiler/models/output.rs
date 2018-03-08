use compiler::evaluator::EvalError;
use super::closure::Closure;
use std::fmt::{Display, Formatter, self};

/// Represents the output of an evaluated expression tree.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Output {
    Int(i64),
    Double(f64),
    Bool(bool),
    String(String),
    Fn(Closure),
}

impl Output {
    /// Returns `true` if this value is allowed to be returned where strings are expected. I.e.
    /// when adding with a string or in Html bodies.
    /// If this returns true then `Output::to_string` should succeed.
    pub fn is_stringable(&self) -> bool {
        match self {
            &Output::Int(_) | &Output::Double(_) | 
            &Output::Bool(_) | &Output::String(_) => true,
            &Output::Fn(_) => false,
        }
    }

    pub fn to_string(self) -> Result<String, EvalError> {
        match self {
            Output::Int(x) => Ok(format!("{}", x)),
            Output::Double(x) => Ok(format!("{}", x)),
            Output::Bool(x) => Ok(format!("{}", x)),
            Output::String(x) => Ok(x),
            non_content => Err(EvalError::NotStringable(non_content)),
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Output::Int(ref x) => write!(f, "{}", x),
            &Output::Double(ref x) => write!(f, "{}", x),
            &Output::Bool(ref x) => write!(f, "{}", x),
            &Output::String(ref x) => write!(f, "\"{}\"", x),
            &Output::Fn(ref func) => write!(f, "<function {}: {}>", func.param_name, func.expr),
        }
    }
}

