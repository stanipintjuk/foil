mod binop;
pub use self::binop::evaluate_binop;
mod path;
pub use self::path::evaluate_path;
mod html;
pub use self::html::{evaluate_html, evaluate_html_closed};
mod import;
pub use self::import::evaluate_import;
mod closure;
pub use self::closure::evaluate_closure;
mod call;
pub use self::call::evaluate_call;
mod val;
pub use self::val::evaluate_val;
mod id;
pub use self::id::evaluate_id;

// let is a keyword in rust so I can't use it as a module name :(
mod let_evaluator;
pub use self::let_evaluator::evaluate_let;
