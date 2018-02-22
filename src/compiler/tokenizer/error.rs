#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum TokenError {
    Garbage(usize, String),
}

