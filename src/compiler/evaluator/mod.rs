mod evaluators;
mod evaluator;
pub use self::evaluator::{Evaluator, EvalResult};

mod error;
pub use  self::error::EvalError;

mod output;
pub use self::output::Output;

mod closure;
pub use self::closure::Closure;

mod scope;
pub use self::scope::{Scope, OpenScope, ClosedScope};

#[cfg(test)] mod tests;
