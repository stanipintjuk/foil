mod evaluators;
mod evaluator;
pub use self::evaluator::{Evaluator, EvalResult};

mod scope;
pub use self::scope::{Scope, OpenScope, ClosedScope};

#[cfg(test)] mod tests;
