mod html_parser;
pub use self::html_parser::parse_html;

mod keyword_parser;
pub use self::keyword_parser::parse_keyword;

mod let_parser;
pub use self::let_parser::parse_let;

mod fn_parser;
pub use self::fn_parser::parse_fn;

mod import_parser;
pub use self::import_parser::parse_import;

mod set_parser;
pub use self::set_parser::parse_set;

mod binop_parser;
pub use self::binop_parser::parse_binop;

mod call_parser;
pub use self::call_parser::parse_call;
